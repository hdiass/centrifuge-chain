// Copyright 2023 Centrifuge Foundation (centrifuge.io).
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

//! # Token Mux Pallet
//!
//! Enables proxying variants of the same foreign assets
//! to a local asset representation. By locking a variant, the user receives
//! the corresponding amount of the local representation minted. The reverse
//! process burns the local asset and transfers back the desired variant
//! directions.
//!
//! ## Assumptions
//!
//! - The exchange rate between the local and its variant assets is exactly one.
//! - Orders can be created for local <> variant asset

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
pub(crate) mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use cfg_traits::swaps::TokenSwaps;
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

	use cfg_traits::swaps::{OrderRatio, TokenSwaps};
	use cfg_types::tokens::{CustomMetadata, HasLocalAssetRepresentation};
	use frame_support::{
		pallet_prelude::{DispatchResult, *},
		traits::{
			fungibles,
			fungibles::Mutate,
			tokens::{Fortitude, Precision, Preservation},
		},
		PalletId,
	};
	use frame_system::pallet_prelude::{OriginFor, *};
	use orml_traits::asset_registry::{self, Inspect as _};
	use sp_arithmetic::{traits::AtLeast32BitUnsigned, FixedPointOperand};
	use sp_runtime::traits::{AccountIdConversion, EnsureFixedPointNumber, One};

	use super::*;

	pub type BalanceOf<T> = <<T as Config>::Tokens as fungibles::Inspect<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The source of truth for the existence and potential local
		/// representation of assets.
		type AssetRegistry: asset_registry::Inspect<
			CustomMetadata = CustomMetadata,
			AssetId = Self::CurrencyId,
		>;

		/// Balance type for incoming values
		type BalanceIn: Member
			+ Parameter
			+ FixedPointOperand
			+ AtLeast32BitUnsigned
			+ MaxEncodedLen
			+ Into<BalanceOf<Self>>
			+ From<BalanceOf<Self>>;

		/// Balance type for outgoing values
		type BalanceOut: Member
			+ Parameter
			+ FixedPointOperand
			+ AtLeast32BitUnsigned
			+ MaxEncodedLen
			+ Into<BalanceOf<Self>>
			+ From<BalanceOf<Self>>;

		/// Type for price ratio for cost of incoming currency relative to
		/// outgoing
		type BalanceRatio: Parameter + Member + sp_runtime::FixedPointNumber + MaxEncodedLen;

		/// The token swap order identifying type
		type OrderId: Parameter + Member + Copy + Ord + MaxEncodedLen;

		/// The general asset type
		type CurrencyId: Parameter + Member + Copy + MaxEncodedLen + HasLocalAssetRepresentation;

		/// The local asset type
		type LocalAssetId: From<cfg_types::tokens::LocalAssetId>;

		/// The type for handling transfers, burning and minting of
		/// multi-assets.
		type Tokens: fungibles::Inspect<Self::AccountId, AssetId = Self::CurrencyId>
			+ Mutate<Self::AccountId, AssetId = Self::CurrencyId>;

		/// The type for retrieving and fulfilling swap orders.
		type OrderBook: TokenSwaps<
			Self::AccountId,
			CurrencyId = Self::CurrencyId,
			BalanceIn = Self::BalanceIn,
			BalanceOut = Self::BalanceOut,
			OrderId = Self::OrderId,
			Ratio = Self::BalanceRatio,
		>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Deposited {
			who: T::AccountId,
			currency_out: T::CurrencyId,
			currency_in: T::CurrencyId,
			amount: T::BalanceOut,
		},
		Burned {
			who: T::AccountId,
			currency_out: T::CurrencyId,
			currency_in: T::CurrencyId,
			amount: T::BalanceOut,
		},
		SwapMatched {
			id: T::OrderId,
			amount: T::BalanceOut,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The given currency has no metadata set.
		MetadataNotFound,
		/// The given currency has no local representation and can hence not be
		/// deposited to receive a local representation.
		NoLocalRepresentation,
		/// Swap could not be found by id
		SwapNotFound,
		/// Matching orders does only work if there is a one-to-one conversion
		NotIdenticalSwap,
		/// This means the swap is either not a local to variant or not a
		/// variant to local swap
		InvalidSwapCurrencies,
		/// Variant and local representation have mismatching decimals in their
		/// metadata. A conversion between the two is not possible
		DecimalMismatch,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::deposit())]
		pub fn deposit(
			origin: OriginFor<T>,
			currency_in: T::CurrencyId,
			currency_out: T::CurrencyId,
			amount_out: T::BalanceOut,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::prepare_and(&currency_out, &currency_in, amount_out, |amount| {
				T::Tokens::transfer(
					currency_in,
					&Self::account(),
					who,
					amount.into(),
					Preservation::Expendable,
				)
			})?;

			Self::deposit_event(Event::<T>::Deposited {
				who,
				currency_out,
				currency_in,
				amount: amount_out,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::burn())]
		pub fn burn(
			origin: OriginFor<T>,
			currency_in: T::CurrencyId,
			currency_out: T::CurrencyId,
			amount_out: T::BalanceOut,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::prepare_and(&currency_out, &currency_in, amount_out, |amount| {
				T::Tokens::transfer(
					currency_out,
					&Self::account(),
					who,
					amount.into(),
					Preservation::Expendable,
				)
			})?;

			Self::deposit_event(Event::<T>::Burned {
				who,
				currency_out: local,
				currency_in: currency_out,
				amount: amount_out,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::match_swap())]
		pub fn match_swap(origin: OriginFor<T>, order_id: T::OrderId) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let order =
				T::OrderBook::get_order_details(order_id).ok_or(Error::<T>::SwapNotFound)?;

			let ratio = match order.ratio {
				OrderRatio::Market => T::BalanceRatio::ensure_from_rational(
					order.amount,
					T::OrderBook::convert_by_market(
						order.swap.currency_in,
						order.swap.currency_out,
						order.amount,
					)?,
				)?,
				OrderRatio::Custom(ratio) => ratio,
			};

			ensure!(ratio == One::one(), Error::<T>::NotIdenticalSwap);

			Self::prepare_and(
				&order.swap.currency_out,
				&order.swap.currency_in,
				order.amount,
				|amount| {
					T::OrderBook::fill_order(Self::account(), order_id, amount)?;
				},
			)?;

			Self::deposit_event(Event::<T>::SwapMatched {
				id: order_id,
				amount: order.amount,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn account() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		pub(crate) fn prepare_and(
			currency_a: &T::CurrencyId,
			currency_b: &T::CurrencyId,
			amount: T::Balance,
			and: FnOnce(T::Balance) -> DispatchResult,
		) -> DispatchResult {
			if currency_a.is_local_representation_of(&currency_b) {
				and(Self::adjust_amount(currency_a, currency_b, amount)?)?;
			}

			if currency_b.is_local_representation_of(&currency_a) {}

			Err(Error::<T>::NoLocalRepresentation.into())
		}
	}
}
