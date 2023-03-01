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

use frame_support::{
	dispatch::GetStorageVersion,
	inherent::Vec,
	pallet_prelude::{StorageVersion, Weight},
	traits::{Get, OnRuntimeUpgrade},
};
use sp_runtime::{BoundedVec, SaturatedConversion};
use sp_std::marker::PhantomData;

use crate::{ActiveSessionData, Config, Pallet, SessionData};

pub struct InitBlockRewards<T, CollatorReward, TotalReward>(
	PhantomData<(T, CollatorReward, TotalReward)>,
);

fn get_collators<T: pallet_collator_selection::Config>() -> Vec<T::AccountId> {
	let candidates = BoundedVec::<
		T::AccountId,
		<T as pallet_collator_selection::Config>::MaxCandidates,
	>::truncate_from(
		pallet_collator_selection::Pallet::<T>::candidates()
			.into_iter()
			.map(|c| c.who)
			.collect(),
	);
	pallet_collator_selection::Pallet::<T>::assemble_collators(candidates)
}

impl<T, CollatorReward, TotalReward> OnRuntimeUpgrade
	for InitBlockRewards<T, CollatorReward, TotalReward>
where
	T: frame_system::Config + Config + pallet_collator_selection::Config,
	<T as Config>::Balance: From<u128>,
	CollatorReward: Get<u128>,
	TotalReward: Get<u128>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<T::AccountId>, &'static str> {
		assert_eq!(
			Pallet::<T>::on_chain_storage_version(),
			StorageVersion::new(0),
			"On-chain storage version should be 0 (default)"
		);
		let collators = get_collators::<T>();
		assert!(!collators.is_empty());

		assert!(!CollatorReward::get().is_zero());
		assert!(
			TotalReward::get()
				>= CollatorReward::get().saturating_mul(collators.len().saturated_into())
		);

		log::info!("💰 BlockRewards: Pre migration checks successful");

		Ok(collators)
	}

	// Weight: 2 + collator_count reads and writes
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		if Pallet::<T>::on_chain_storage_version() == StorageVersion::new(2) {
			log::info!("💰 BlockRewards: Initiating migration");
			let mut weight: Weight = Weight::zero();

			let collators = get_collators::<T>();
			weight.saturating_accrue(T::DbWeight::get().reads(2));

			ActiveSessionData::<T>::set(SessionData::<T> {
				collator_reward: CollatorReward::get().into(),
				total_reward: TotalReward::get().into(),
				collator_count: collators.len().saturated_into(),
			});
			weight.saturating_accrue(T::DbWeight::get().writes(1));

			for collator in collators.iter() {
				// TODO: Benching preferred to be precise.
				// However, not necessarily needed as num of collators <= 10.
				Pallet::<T>::do_init_collator(collator)
					.map_err(|e| {
						log::error!("Failed to init genesis collators for rewards: {:?}", e);
					})
					.ok();
				weight.saturating_accrue(T::DbWeight::get().reads_writes(6, 6));
			}

			Pallet::<T>::current_storage_version().put::<Pallet<T>>();
			weight.saturating_add(T::DbWeight::get().writes(1))
		} else {
			// wrong storage version
			log::info!(
				"💰 BlockRewards: Migration did not execute. This probably should be removed"
			);
			T::DbWeight::get().reads_writes(1, 0)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(collators: Vec<T::AccountId>) -> Result<(), &'static str> {
		assert_eq!(
			Pallet::<T>::on_chain_storage_version(),
			StorageVersion::new(1),
			"On-chain storage version should be updated"
		);
		assert_eq!(
			Pallet::<T>::active_session_data(),
			SessionData::<T> {
				collator_reward: CollatorReward::get().into(),
				total_reward: TotalReward::get().into(),
				collator_count: collators.len().saturated_into(),
			}
		);

		for collator in collators.iter() {
			assert!(!<T as Config>::Rewards::account_stake(
				(<T as Config>::Domain::get(), STAKE_CURRENCY_ID,),
				collator,
			)
			.is_zero())
		}

		log::info!("💰 BlockRewards: Post migration checks successful");

		Ok(())
	}
}
