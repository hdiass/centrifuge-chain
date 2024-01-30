//! Types with Config access. This module does not mutate FI storage

use cfg_traits::{investments::Investment, TokenSwaps};
use cfg_types::investments::{
	CollectedAmount, ExecutedForeignCollect, ExecutedForeignDecreaseInvest, Swap,
};
use frame_support::{dispatch::DispatchResult, ensure};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{
		EnsureAdd, EnsureAddAssign, EnsureDiv, EnsureMul, EnsureSub, EnsureSubAssign, Saturating,
		Zero,
	},
	ArithmeticError, DispatchError,
};
use sp_std::cmp::min;

use crate::{
	pallet::{Config, Error},
	pool_currency_of,
	swaps::Swaps,
	Action, SwapOf,
};

/// Hold the base information of a foreign investment/redemption
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct BaseInfo<T: Config> {
	pub foreign_currency: T::CurrencyId,
	pub collected: CollectedAmount<T::Balance>,
}

impl<T: Config> BaseInfo<T> {
	pub fn new(foreign_currency: T::CurrencyId) -> Result<Self, DispatchError> {
		Ok(Self {
			foreign_currency,
			collected: CollectedAmount::default(),
		})
	}

	pub fn ensure_same_foreign(&self, foreign_currency: T::CurrencyId) -> DispatchResult {
		ensure!(
			self.foreign_currency == foreign_currency,
			Error::<T>::MismatchedForeignCurrency
		);

		Ok(())
	}
}

/// Hold the information of a foreign investment
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct InvestmentInfo<T: Config> {
	/// General info
	pub base: BaseInfo<T>,

	/// Value used to correlate the pool amount into
	/// foreign amount where market the conversion is not known upfront.
	///
	/// A mirror of [`pool_amount_in_system()`] but denominated in foreign
	/// currency. When `pool_amount_in_system()` value increases, this value
	/// increases, and vice-versa. See that function to know exactly which
	/// information it carries.
	pub pool_amount_in_system_but_in_foreign_amount: T::Balance,

	/// Total decrease swapped amount pending to execute.
	pub decrease_swapped_foreign_amount: T::Balance,
}

impl<T: Config> InvestmentInfo<T> {
	pub fn new(foreign_currency: T::CurrencyId) -> Result<Self, DispatchError> {
		Ok(Self {
			base: BaseInfo::new(foreign_currency)?,
			pool_amount_in_system_but_in_foreign_amount: T::Balance::default(),
			decrease_swapped_foreign_amount: T::Balance::default(),
		})
	}

	/// This method is performed before applying the swap.
	pub fn pre_increase_swap(
		&mut self,
		investment_id: T::InvestmentId,
		foreign_amount: T::Balance,
	) -> Result<SwapOf<T>, DispatchError> {
		Ok(Swap {
			currency_in: pool_currency_of::<T>(investment_id)?,
			currency_out: self.base.foreign_currency,
			amount_out: foreign_amount,
		})
	}

	/// Decrease an investment taking into account that a previous increment
	/// could be pending.
	/// This method is performed before applying the swap.
	pub fn pre_decrease_swap(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		foreign_amount: T::Balance,
	) -> Result<SwapOf<T>, DispatchError> {
		// We do not want to decrease the whole `foreign_amount` from the investment
		// amount if there is a pending investment swap.
		let pending_increase_foreign_amount = self.pending_increase_swap(who, investment_id)?;
		let foreign_decrement = foreign_amount.saturating_sub(pending_increase_foreign_amount);

		let mut pool_decrement = T::Balance::default();
		if !foreign_decrement.is_zero() {
			pool_decrement = self.foreign_to_pool(who, investment_id, foreign_decrement)?;

			T::Investment::update_investment(
				who,
				investment_id,
				T::Investment::investment(who, investment_id)?
					.ensure_sub(pool_decrement)
					.map_err(|_| Error::<T>::TooMuchDecrease)?,
			)?;
		}

		// It's ok to use the market ratio because this amount will be
		// cancelled.
		let pending_increase_pool_amount = T::TokenSwaps::convert_by_market(
			pool_currency_of::<T>(investment_id)?,
			self.base.foreign_currency,
			min(pending_increase_foreign_amount, foreign_amount),
		)?;

		Ok(Swap {
			currency_in: self.base.foreign_currency,
			currency_out: pool_currency_of::<T>(investment_id)?,
			amount_out: pending_increase_pool_amount.ensure_add(pool_decrement)?,
		})
	}

	/// Increase an investment taking into account that a previous decrement
	/// could be pending.
	/// This method is performed after resolve the swap.
	pub fn post_increase_swap(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_pool_amount: T::Balance,
		swapped_foreign_amount: T::Balance,
		from_cancel: bool,
	) -> DispatchResult {
		self.decrease_swapped_foreign_amount = T::Balance::default();

		if !swapped_pool_amount.is_zero() {
			T::Investment::update_investment(
				who,
				investment_id,
				T::Investment::investment(who, investment_id)?.ensure_add(swapped_pool_amount)?,
			)?;

			if !from_cancel {
				self.pool_amount_in_system_but_in_foreign_amount
					.ensure_add_assign(swapped_foreign_amount)?;
			}
		}

		Ok(())
	}

	/// This method is performed after resolve the swap.
	#[allow(clippy::type_complexity)]
	pub fn post_decrease_swap(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_foreign_amount: T::Balance,
		pending_pool_amount: T::Balance,
		from_cancel: bool,
	) -> Result<Option<ExecutedForeignDecreaseInvest<T::Balance, T::CurrencyId>>, DispatchError> {
		self.decrease_swapped_foreign_amount
			.ensure_add_assign(swapped_foreign_amount)?;

		if !from_cancel {
			self.pool_amount_in_system_but_in_foreign_amount
				.ensure_sub_assign(swapped_foreign_amount)?;
		}

		if pending_pool_amount.is_zero() {
			let amount_decreased = sp_std::mem::take(&mut self.decrease_swapped_foreign_amount);

			let msg = ExecutedForeignDecreaseInvest {
				amount_decreased,
				foreign_currency: self.base.foreign_currency,
				amount_remaining: self.remaining_foreign_amount(who, investment_id)?,
			};

			return Ok(Some(msg));
		}

		Ok(None)
	}

	/// This method is performed after a collect
	pub fn post_collect(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		collected: CollectedAmount<T::Balance>,
	) -> Result<ExecutedForeignCollect<T::Balance, T::CurrencyId>, DispatchError> {
		self.base.collected.increase(&collected)?;

		let collected_foreign_amount = collected
			.amount_payment
			.ensure_mul(self.pool_amount_in_system_but_in_foreign_amount)?
			.ensure_div(
				self.pool_amount_in_system(who, investment_id)?
					.ensure_add(collected.amount_payment)?,
			)?;

		self.pool_amount_in_system_but_in_foreign_amount
			.ensure_sub_assign(collected_foreign_amount)?;

		Ok(ExecutedForeignCollect {
			currency: self.base.foreign_currency,
			amount_currency_payout: collected_foreign_amount,
			amount_tranche_tokens_payout: collected.amount_collected,
			amount_remaining: self.remaining_foreign_amount(who, investment_id)?,
		})
	}

	/// Contains the amount in the system that is currently denominated in pool
	/// currency, which is:
	/// - The invested amount
	/// - Any pending decrease amount pending to be swapped
	fn pool_amount_in_system(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<T::Balance, DispatchError> {
		Ok(T::Investment::investment(who, investment_id)?
			.ensure_add(self.pending_decrease_swap(who, investment_id)?)?)
	}

	/// Remaining amount to finalize the investment, denominated in foreign
	/// currency. It takes care of:
	/// - Any invested amount.
	/// - Any increase pending amount to be swapped.
	/// - Any decrease pending amount to be swapped.
	/// - Any decrease swapped amount not yet executed.
	fn remaining_foreign_amount(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<T::Balance, DispatchError> {
		Ok(self
			.pool_amount_in_system_but_in_foreign_amount
			.ensure_add(self.decrease_swapped_foreign_amount)?
			.ensure_add(self.pending_increase_swap(who, investment_id)?)?)
	}

	/// Invested amount in foreign currency, which is:
	/// - Any pending increase amount to be swapped.
	/// - Any invested amount.
	/// Using this value for a decrease investment, will cancel the investment.
	pub fn invested_foreign_amount(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<T::Balance, DispatchError> {
		let invested = T::Investment::investment(who, investment_id)?;
		let invested_foreign_amount = match self.pool_to_foreign(who, investment_id, invested) {
			Ok(amount) => amount,
			Err(DispatchError::Arithmetic(ArithmeticError::DivisionByZero)) => T::Balance::zero(),
			Err(err) => Err(err)?,
		};

		Ok(self
			.pending_increase_swap(who, investment_id)?
			.ensure_add(invested_foreign_amount)?)
	}

	/// Get the pool amount representation of a foreign amount that is already
	/// in the system as pool amount. It does not requires a market
	/// conversion because it's done by relative proportions.
	fn foreign_to_pool(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		foreign_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		foreign_amount
			.ensure_mul(self.pool_amount_in_system(who, investment_id)?)?
			.ensure_div(self.pool_amount_in_system_but_in_foreign_amount)
			.map_err(|_| Error::<T>::TooMuchDecrease.into())
	}

	/// Get the foreign amount representation of a pool amount that is already
	/// in the system as pool amount. It does not requires a market conversion
	/// because it's done by relative proportions.
	fn pool_to_foreign(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		pool_amount: T::Balance,
	) -> Result<T::Balance, DispatchError> {
		Ok(pool_amount
			.ensure_mul(self.pool_amount_in_system_but_in_foreign_amount)?
			.ensure_div(self.pool_amount_in_system(who, investment_id)?)?)
	}

	/// In foreign currency denomination
	fn pending_increase_swap(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<T::Balance, DispatchError> {
		Ok(Swaps::<T>::pending_amount_for(
			who,
			investment_id,
			Action::Investment,
			self.base.foreign_currency,
		))
	}

	/// In pool currency denomination
	fn pending_decrease_swap(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<T::Balance, DispatchError> {
		Ok(Swaps::<T>::pending_amount_for(
			who,
			investment_id,
			Action::Investment,
			pool_currency_of::<T>(investment_id)?,
		))
	}

	pub fn is_completed(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<bool, DispatchError> {
		Ok(self.remaining_foreign_amount(who, investment_id)?.is_zero())
	}
}

/// Hold the information of an foreign redemption
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct RedemptionInfo<T: Config> {
	/// General info
	pub base: BaseInfo<T>,

	/// Total swapped amount pending to execute.
	pub swapped_amount: T::Balance,
}

impl<T: Config> RedemptionInfo<T> {
	pub fn new(foreign_currency: T::CurrencyId) -> Result<Self, DispatchError> {
		Ok(Self {
			base: BaseInfo::new(foreign_currency)?,
			swapped_amount: T::Balance::default(),
		})
	}

	pub fn increase(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		tranche_tokens_amount: T::Balance,
	) -> DispatchResult {
		T::Investment::update_redemption(
			who,
			investment_id,
			T::Investment::redemption(who, investment_id)?.ensure_add(tranche_tokens_amount)?,
		)
	}

	pub fn decrease(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		tranche_tokens_amount: T::Balance,
	) -> DispatchResult {
		T::Investment::update_redemption(
			who,
			investment_id,
			T::Investment::redemption(who, investment_id)?.ensure_sub(tranche_tokens_amount)?,
		)
	}

	/// This method is performed after a collect and before applying the swap
	pub fn post_collect_and_pre_swap(
		&mut self,
		investment_id: T::InvestmentId,
		collected: CollectedAmount<T::Balance>,
	) -> Result<SwapOf<T>, DispatchError> {
		self.base.collected.increase(&collected)?;

		Ok(Swap {
			currency_in: self.base.foreign_currency,
			currency_out: pool_currency_of::<T>(investment_id)?,
			amount_out: collected.amount_collected,
		})
	}

	/// This method is performed after resolve the swap.
	#[allow(clippy::type_complexity)]
	pub fn post_swap(
		&mut self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_amount: T::Balance,
		pending_amount: T::Balance,
	) -> Result<Option<ExecutedForeignCollect<T::Balance, T::CurrencyId>>, DispatchError> {
		self.swapped_amount.ensure_add_assign(swapped_amount)?;
		if pending_amount.is_zero() {
			let msg = ExecutedForeignCollect {
				currency: self.base.foreign_currency,
				amount_currency_payout: self.swapped_amount,
				amount_tranche_tokens_payout: self.collected_tranche_tokens(),
				amount_remaining: T::Investment::redemption(who, investment_id)?,
			};

			self.base.collected = CollectedAmount::default();
			self.swapped_amount = T::Balance::default();

			return Ok(Some(msg));
		}

		Ok(None)
	}

	fn collected_tranche_tokens(&self) -> T::Balance {
		self.base.collected.amount_payment
	}

	pub fn is_completed(
		&self,
		who: &T::AccountId,
		investment_id: T::InvestmentId,
	) -> Result<bool, DispatchError> {
		Ok(T::Investment::redemption(who, investment_id)?.is_zero()
			&& self.collected_tranche_tokens().is_zero())
	}
}
