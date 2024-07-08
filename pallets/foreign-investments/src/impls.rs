//! Trait implementations. Higher level file.

use cfg_traits::{investments::ForeignInvestment, swaps::SwapInfo, StatusNotificationHook};
use cfg_types::investments::CollectedAmount;
use frame_support::pallet_prelude::*;
use sp_std::marker::PhantomData;

use crate::{
	entities::{InvestmentInfo, RedemptionInfo},
	pallet::{Config, Error, Event, ForeignInvestmentInfo, ForeignRedemptionInfo, Pallet},
	pool_currency_of, Action, SwapId,
};

impl<T: Config> ForeignInvestment<T::AccountId> for Pallet<T> {
	type Amount = T::ForeignBalance;
	type CurrencyId = T::CurrencyId;
	type Error = DispatchError;
	type InvestmentId = T::InvestmentId;
	type TrancheAmount = T::TrancheBalance;

	fn increase_foreign_investment(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		foreign_amount: T::ForeignBalance,
		foreign_currency: T::CurrencyId,
	) -> DispatchResult {
		ForeignInvestmentInfo::<T>::mutate(who, investment_id, |entry| {
			let info = entry.get_or_insert(InvestmentInfo::new(foreign_currency));
			info.ensure_same_foreign(foreign_currency)?;
			info.ensure_no_pending_cancel(who, investment_id)?;

			info.increase_swap(who, investment_id, foreign_amount)?;

			if info.foreign_currency == pool_currency_of::<T>(investment_id)? {
				info.post_increase_swap(who, investment_id, foreign_amount.into(), foreign_amount)?;
			}

			Ok(())
		})
	}

	fn cancel_foreign_investment(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		foreign_currency: T::CurrencyId,
	) -> DispatchResult {
		let msg = ForeignInvestmentInfo::<T>::mutate(who, investment_id, |entry| {
			let info = entry.as_mut().ok_or(Error::<T>::InfoNotFound)?;
			info.ensure_same_foreign(foreign_currency)?;
			info.ensure_no_pending_cancel(who, investment_id)?;

			let (cancelled, pending) = info.cancel_swap(who, investment_id)?;
			let msg = info.post_cancel_swap(cancelled, pending)?;

			if info.is_completed(who, investment_id)? {
				*entry = None;
			}

			Ok::<_, DispatchError>(msg)
		})?;

		if let Some(msg) = msg {
			T::DecreasedForeignInvestOrderHook::notify_status_change(
				(who.clone(), investment_id),
				msg,
			)?;
		}

		Ok(())
	}

	fn increase_foreign_redemption(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		tranche_tokens_amount: T::TrancheBalance,
		payout_foreign_currency: T::CurrencyId,
	) -> DispatchResult {
		ForeignRedemptionInfo::<T>::mutate(who, investment_id, |info| -> DispatchResult {
			let info = info.get_or_insert(RedemptionInfo::new(payout_foreign_currency));
			info.ensure_same_foreign(payout_foreign_currency)?;
			info.increase_redemption(who, investment_id, tranche_tokens_amount)
		})
	}

	fn cancel_foreign_redemption(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		payout_foreign_currency: T::CurrencyId,
	) -> DispatchResult {
		ForeignRedemptionInfo::<T>::mutate_exists(who, investment_id, |entry| {
			let info = entry.as_mut().ok_or(Error::<T>::InfoNotFound)?;
			info.ensure_same_foreign(payout_foreign_currency)?;
			info.cancel_redeemption(who, investment_id)?;

			if info.is_completed(who, investment_id)? {
				*entry = None;
			}

			Ok(())
		})
	}
}

pub struct FulfilledSwapHook<T>(PhantomData<T>);
impl<T: Config> StatusNotificationHook for FulfilledSwapHook<T> {
	type Error = DispatchError;
	type Id = (T::AccountId, SwapId<T>);
	type Status = SwapInfo<T::SwapBalance, T::SwapBalance, T::CurrencyId, T::SwapRatio>;

	fn notify_status_change(
		(who, (investment_id, action)): Self::Id,
		swap_info: Self::Status,
	) -> DispatchResult {
		let pool_currency = pool_currency_of::<T>(investment_id)?;
		let swapped_amount_in = swap_info.swapped_in;
		let swapped_amount_out = swap_info.swapped_out;
		let pending_amount = swap_info.remaining.amount_out;

		Pallet::<T>::deposit_event(Event::SwapFullfilled {
			who: who.clone(),
			swap_id: (investment_id, action),
			remaining: swap_info.remaining.clone(),
			swapped_in: swap_info.swapped_in,
			swapped_out: swap_info.swapped_out,
		});

		match action {
			Action::Investment => match pool_currency == swap_info.remaining.currency_in {
				true => SwapDone::<T>::for_increase_investment(
					&who,
					investment_id,
					swapped_amount_in.into(),
					swapped_amount_out.into(),
				),
				false => SwapDone::<T>::for_decrease_investment(
					&who,
					investment_id,
					swapped_amount_in.into(),
					pending_amount.into(),
				),
			},
			Action::Redemption => SwapDone::<T>::for_redemption(
				&who,
				investment_id,
				swapped_amount_in.into(),
				pending_amount.into(),
			),
		}
	}
}

pub struct CollectedInvestmentHook<T>(PhantomData<T>);
impl<T: Config> StatusNotificationHook for CollectedInvestmentHook<T> {
	type Error = DispatchError;
	type Id = (T::AccountId, T::InvestmentId);
	type Status = CollectedAmount<T::TrancheBalance, T::PoolBalance>;

	fn notify_status_change(
		(who, investment_id): (T::AccountId, T::InvestmentId),
		collected: CollectedAmount<T::TrancheBalance, T::PoolBalance>,
	) -> DispatchResult {
		let msg = ForeignInvestmentInfo::<T>::mutate_exists(&who, investment_id, |entry| {
			if let Some(info) = entry.as_mut() {
				info.ensure_no_pending_cancel(&who, investment_id)?;
				let msg = info.post_collect(&who, investment_id, collected)?;

				if info.is_completed(&who, investment_id)? {
					*entry = None;
				}

				return Ok::<_, DispatchError>(Some(msg));
			}
			Ok(None) // Then notification is not for foreign investments
		})?;

		// We send the event out of the Info mutation closure
		if let Some(msg) = msg {
			T::CollectedForeignInvestmentHook::notify_status_change(
				(who.clone(), investment_id),
				msg,
			)?;
		}

		Ok(())
	}
}

pub struct CollectedRedemptionHook<T>(PhantomData<T>);
impl<T: Config> StatusNotificationHook for CollectedRedemptionHook<T> {
	type Error = DispatchError;
	type Id = (T::AccountId, T::InvestmentId);
	type Status = CollectedAmount<T::PoolBalance, T::TrancheBalance>;

	fn notify_status_change(
		(who, investment_id): (T::AccountId, T::InvestmentId),
		collected: CollectedAmount<T::PoolBalance, T::TrancheBalance>,
	) -> DispatchResult {
		let msg = ForeignRedemptionInfo::<T>::mutate(&who, investment_id, |entry| {
			if let Some(info) = entry.as_mut() {
				let (amount, pending) =
					info.post_collect_and_swap(&who, investment_id, collected)?;
				let msg = info.post_swap(amount, pending)?;

				if info.is_completed(&who, investment_id)? {
					*entry = None;
				}

				return Ok::<_, DispatchError>(msg);
			}

			Ok(None) // Then the notification is not for foreign investments
		})?;

		// We send the event out of the Info mutation closure
		if let Some(msg) = msg {
			T::CollectedForeignRedemptionHook::notify_status_change(
				(who.clone(), investment_id),
				msg,
			)?;
		}

		Ok(())
	}
}

/// Internal methods used to execute swaps already done
struct SwapDone<T>(PhantomData<T>);
impl<T: Config> SwapDone<T> {
	/// Notifies that a partial increse swap has been done and applies the
	/// result to an `InvestmentInfo`
	fn for_increase_investment(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_pool_amount: T::PoolBalance,
		swapped_foreign_amount: T::ForeignBalance,
	) -> DispatchResult {
		ForeignInvestmentInfo::<T>::mutate_exists(who, investment_id, |entry| {
			let info = entry.as_mut().ok_or(Error::<T>::InfoNotFound)?;
			info.post_increase_swap(
				who,
				investment_id,
				swapped_pool_amount,
				swapped_foreign_amount,
			)
		})
	}

	/// Notifies that a partial decrease swap has been done and applies the
	/// result to an `InvestmentInfo`
	fn for_decrease_investment(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_foreign_amount: T::ForeignBalance,
		pending_pool_amount: T::PoolBalance,
	) -> DispatchResult {
		let msg = ForeignInvestmentInfo::<T>::mutate_exists(who, investment_id, |entry| {
			let info = entry.as_mut().ok_or(Error::<T>::InfoNotFound)?;
			let msg = info.post_cancel_swap(swapped_foreign_amount, pending_pool_amount)?;

			if info.is_completed(who, investment_id)? {
				*entry = None;
			}

			Ok::<_, DispatchError>(msg)
		})?;

		// We send the event out of the Info mutation closure
		if let Some(msg) = msg {
			T::DecreasedForeignInvestOrderHook::notify_status_change(
				(who.clone(), investment_id),
				msg,
			)?;
		}

		Ok(())
	}

	/// Notifies that a partial swap has been done and applies the result to
	/// an `RedemptionInfo`
	fn for_redemption(
		who: &T::AccountId,
		investment_id: T::InvestmentId,
		swapped_amount: T::ForeignBalance,
		pending_amount: T::PoolBalance,
	) -> DispatchResult {
		let msg = ForeignRedemptionInfo::<T>::mutate_exists(who, investment_id, |entry| {
			let info = entry.as_mut().ok_or(Error::<T>::InfoNotFound)?;
			let msg = info.post_swap(swapped_amount, pending_amount)?;

			if info.is_completed(who, investment_id)? {
				*entry = None;
			}

			Ok::<_, DispatchError>(msg)
		})?;

		// We send the event out of the Info mutation closure
		if let Some(msg) = msg {
			T::CollectedForeignRedemptionHook::notify_status_change(
				(who.clone(), investment_id),
				msg,
			)?;
		}

		Ok(())
	}
}
