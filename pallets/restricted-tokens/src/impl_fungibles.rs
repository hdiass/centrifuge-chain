// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_traits::PreConditions;
use frame_support::traits::{
	fungible,
	fungibles::{Dust, Inspect, InspectHold, Mutate, MutateHold, Unbalanced},
	tokens::{
		DepositConsequence, Fortitude, Precision, Preservation, Provenance, Restriction,
		WithdrawConsequence,
	},
};

use super::*;

/// Represents the trait `fungibles::Inspect` effects that are called via
/// the pallet-restricted-tokens.
pub enum FungiblesInspectEffects<AssetId, AccountId, Balance> {
	/// A call to the `Inspect::reducible_balance()`.
	///
	/// Interpretation of tuple `(AssetId, AccountId, bool, Balance)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be checked.
	/// * tuple.2 = `keep_alive`. The liveness bool.
	/// * tuple.3 = `<T::Fungibles as
	///   Inspect<T::AccountId>>::reducible_balance()`. The result of the call
	///   to the not-filtered trait `fungibles::Inspect` implementation.
	ReducibleBalance(AssetId, AccountId, bool, Balance),
}

pub struct FungiblesInspectPassthrough;
impl<AssetId, AccountId, Balance>
	PreConditions<FungiblesInspectEffects<AssetId, AccountId, Balance>>
	for FungiblesInspectPassthrough
{
	type Result = Balance;

	fn check(t: FungiblesInspectEffects<AssetId, AccountId, Balance>) -> Self::Result {
		match t {
			FungiblesInspectEffects::ReducibleBalance(_, _, _, amount) => amount,
		}
	}
}

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
	type AssetId = T::CurrencyId;
	type Balance = T::Balance;

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::total_issuance()
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::total_issuance(asset)
		}
	}

	fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<T::NativeFungible as fungible::Inspect<T::AccountId>>::minimum_balance()
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::minimum_balance(asset)
		}
	}

	fn total_balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::total_balance(who)
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::total_balance(asset, who)
		}
	}

	fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::balance(who)
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::balance(asset, who)
		}
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &T::AccountId,
		preservation: Preservation,
		force: Fortitude,
	) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::reducible_balance(
				who,
				preservation,
				force,
			)
		} else {
			T::PreFungiblesInspect::check(FungiblesInspectEffects::ReducibleBalance(
				asset,
				who.clone(),
				preservation != Preservation::Expendable,
				<T::Fungibles as Inspect<T::AccountId>>::reducible_balance(
					asset,
					who,
					preservation,
					force,
				),
			))
		}
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		provenance: Provenance,
	) -> DepositConsequence {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::can_deposit(who, amount, provenance)
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::can_deposit(asset, who, amount, provenance)
		}
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> WithdrawConsequence<Self::Balance> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Inspect<T::AccountId>>::can_withdraw(who, amount)
		} else {
			<T::Fungibles as Inspect<T::AccountId>>::can_withdraw(asset, who, amount)
		}
	}

	fn asset_exists(_asset: Self::AssetId) -> bool {
		todo!("nuno")
	}
}

/// Represents the trait `fungibles::InspectHold` effects that are called via
/// the pallet-restricted-tokens.
pub enum FungiblesInspectHoldEffects<AssetId, AccountId, Balance> {
	/// A call to the `InspectHold::can_hold()`.
	///
	/// Interpretation of tuple `(AccountId, Balance, bool)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be reserved.
	/// * tuple.2 = `amount`. The amount that should be reserved.
	/// * tuple.3 = `<T::Fungibles as InspectHold<T::AccountId>>::can_hold()`.
	///   The result of the call to the not-filtered trait
	///   `fungibles::InspectHold` implementation.
	CanHold(AssetId, AccountId, Balance, bool),
}

impl<T: Config> InspectHold<T::AccountId> for Pallet<T> {
	type Reason = ();

	fn total_balance_on_hold(_asset: Self::AssetId, _who: &T::AccountId) -> Self::Balance {
		todo!("nuno")
	}

	fn reducible_total_balance_on_hold(
		_asset: Self::AssetId,
		_who: &T::AccountId,
		_force: Fortitude,
	) -> Self::Balance {
		todo!("nuno")
	}

	fn balance_on_hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &T::AccountId,
	) -> Self::Balance {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::InspectHold<T::AccountId>>::balance_on_hold(reason, who)
		} else {
			<T::Fungibles as InspectHold<T::AccountId>>::balance_on_hold(asset, reason, who)
		}
	}

	fn hold_available(_asset: Self::AssetId, _reason: &Self::Reason, _who: &T::AccountId) -> bool {
		todo!("nuno")
	}

	fn can_hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> bool {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::InspectHold<T::AccountId>>::can_hold(reason, who, amount)
		} else {
			let can_hold =
				<T::Fungibles as InspectHold<T::AccountId>>::can_hold(asset, reason, who, amount);

			T::PreFungiblesInspectHold::check(FungiblesInspectHoldEffects::CanHold(
				asset,
				who.clone(),
				amount,
				can_hold,
			)) && can_hold
		}
	}
}

/// Represents the trait `fungibles::Mutate` effects that are called via
/// the pallet-restricted-tokens.
pub enum FungiblesMutateEffects<AssetId, AccountId, Balance> {
	/// A call to the `Mutate::mint_into()`.
	///
	/// Interpretation of tuple `(AccountId, Balance, bool)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be altered.
	/// * tuple.2 = `amount`. The amount that should be minted.
	MintInto(AssetId, AccountId, Balance),

	/// A call to the `Mutate::burn_from()`.
	///
	/// Interpretation of tuple `(AccountId, Balance, bool)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be altered.
	/// * tuple.2 = `amount`. The amount that should be burned.
	BurnFrom(AssetId, AccountId, Balance),
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
	fn mint_into(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Mutate<T::AccountId>>::mint_into(who, amount)
		} else {
			ensure!(
				T::PreFungiblesMutate::check(FungiblesMutateEffects::MintInto(
					asset,
					who.clone(),
					amount
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as Mutate<T::AccountId>>::mint_into(asset, who, amount)
		}
	}

	fn burn_from(
		asset: Self::AssetId,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		force: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Mutate<T::AccountId>>::burn_from(who, amount, precision, force)
		} else {
			ensure!(
				T::PreFungiblesMutate::check(FungiblesMutateEffects::BurnFrom(
					asset,
					who.clone(),
					amount
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as Mutate<T::AccountId>>::burn_from(asset, who, amount, precision, force)
		}
	}

	fn transfer(
		asset: Self::AssetId,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		preservation: Preservation,
	) -> Result<Self::Balance, DispatchError> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::Mutate<T::AccountId>>::transfer(
				source,
				dest,
				amount,
				preservation,
			)
		} else {
			ensure!(
				T::PreFungiblesTransfer::check(FungiblesTransferEffects::Transfer(
					asset,
					source.clone(),
					dest.clone(),
					amount,
					preservation != Preservation::Expendable,
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as Mutate<T::AccountId>>::transfer(
				asset,
				source,
				dest,
				amount,
				preservation,
			)
		}
	}
}

/// Represents the trait `fungibles::MutateHold` effects that are called via
/// the pallet-restricted-tokens.
pub enum FungiblesMutateHoldEffects<AssetId, AccountId, Balance> {
	/// A call to the `MutateHold::hold()`.
	///
	/// Interpretation of tuple `(AssetId, AccountId, Balance)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be altered.
	/// * tuple.2 = `amount`. The amount that should be hold.
	Hold(AssetId, AccountId, Balance),

	/// A call to the `MutateHold::release()`.
	///
	/// Interpretation of tuple `(AssetId, AccountId, Balance)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `who`. The person who's balance should be altered.
	/// * tuple.2 = `amount`. The amount that should be released.
	Release(AssetId, AccountId, Balance, bool),

	/// A call to the `MutateHold::transfer_held()`.
	///
	/// Interpretation of tuple `(AssetId, AccountId, AccountId, Balance, bool,
	/// bool)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `send`. The sender of the tokens.
	/// * tuple.2 = `recv`. The receiver of the tokens.
	/// * tuple.3 = `amount`. The amount that should be transferred.
	/// * tuple.4 = `on_hold`. Indicating if on_hold transfers should still be
	///   on_hold at receiver.
	/// * tuple.5 = `best_effort`. Indicating if the transfer should be done on
	///   a best effort base.
	TransferHeld(AssetId, AccountId, AccountId, Balance, bool, bool),
}

impl<T: Config> fungibles::hold::Unbalanced<T::AccountId> for Pallet<T> {
	fn set_balance_on_hold(
		_asset: Self::AssetId,
		_reason: &Self::Reason,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> sp_runtime::DispatchResult {
		todo!("nuno")
	}
}

impl<T: Config> MutateHold<T::AccountId> for Pallet<T> {
	fn hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &T::AccountId,
		amount: Self::Balance,
	) -> DispatchResult {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::MutateHold<T::AccountId>>::hold(reason, who, amount)
		} else {
			ensure!(
				T::PreFungiblesMutateHold::check(FungiblesMutateHoldEffects::Hold(
					asset,
					who.clone(),
					amount
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as MutateHold<T::AccountId>>::hold(asset, reason, who, amount)
		}
	}

	fn release(
		asset: Self::AssetId,
		reason: &Self::Reason,
		who: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
	) -> Result<Self::Balance, DispatchError> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::MutateHold<T::AccountId>>::release(
				reason, who, amount, precision,
			)
		} else {
			ensure!(
				T::PreFungiblesMutateHold::check(FungiblesMutateHoldEffects::Release(
					asset,
					who.clone(),
					amount,
					precision == Precision::BestEffort,
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as MutateHold<T::AccountId>>::release(
				asset, reason, who, amount, precision,
			)
		}
	}

	fn transfer_on_hold(
		asset: Self::AssetId,
		reason: &Self::Reason,
		source: &T::AccountId,
		dest: &T::AccountId,
		amount: Self::Balance,
		precision: Precision,
		mode: Restriction,
		force: Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		if asset == T::NativeToken::get() {
			<Pallet<T> as fungible::MutateHold<T::AccountId>>::transfer_on_hold(
				reason, source, dest, amount, precision, mode, force,
			)
		} else {
			ensure!(
				T::PreFungiblesMutateHold::check(FungiblesMutateHoldEffects::TransferHeld(
					asset,
					source.clone(),
					dest.clone(),
					amount,
					precision == Precision::BestEffort,
					mode == Restriction::OnHold,
				)),
				Error::<T>::PreConditionsNotMet
			);

			<T::Fungibles as MutateHold<T::AccountId>>::transfer_on_hold(
				asset, reason, source, dest, amount, precision, mode, force,
			)
		}
	}
}

/// Represents the trait `fungibles::Transfer` effects that are called via
/// the pallet-restricted-tokens.
pub enum FungiblesTransferEffects<AssetId, AccountId, Balance> {
	/// A call to the `Transfer::transfer()`.
	///
	/// Interpretation of tuple `(AssetId, AccountId, AccountId, Balance,
	/// bool)`:
	/// * tuple.0 = `asset`. The asset that should be used.
	/// * tuple.1 = `send`. The sender of the tokens.
	/// * tuple.2 = `recv`. The receiver of the tokens.
	/// * tuple.3 = `amount`. The amount that should be transferred.
	/// * tuple.4 = `keep_alice`. The lifeness requirements.
	Transfer(AssetId, AccountId, AccountId, Balance, bool),
}

impl<T: Config> Unbalanced<T::AccountId> for Pallet<T> {
	fn handle_dust(_dust: Dust<T::AccountId, Self>) {
		todo!("nuno")
	}

	fn write_balance(
		_asset: Self::AssetId,
		_who: &T::AccountId,
		_amount: Self::Balance,
	) -> Result<Option<Self::Balance>, DispatchError> {
		todo!("nuno")
	}

	fn set_total_issuance(_asset: Self::AssetId, _amount: Self::Balance) {
		todo!("nuno")
	}
}
