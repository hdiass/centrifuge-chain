
//! Autogenerated weights for `pallet_block_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_block_rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_block_rewards.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_block_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_block_rewards::WeightInfo for WeightInfo<T> {
	/// Storage: BlockRewardsBase Currency (r:1 w:0)
	/// Proof: BlockRewardsBase Currency (max_values: None, max_size: Some(79), added: 2554, mode: MaxEncodedLen)
	/// Storage: BlockRewardsBase Group (r:1 w:0)
	/// Proof: BlockRewardsBase Group (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: BlockRewardsBase StakeAccount (r:1 w:1)
	/// Proof: BlockRewardsBase StakeAccount (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `583`
		//  Estimated: `6196`
		// Minimum execution time: 89_777_000 picoseconds.
		Weight::from_parts(90_960_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: BlockRewards NextSessionChanges (r:1 w:1)
	/// Proof: BlockRewards NextSessionChanges (max_values: Some(1), max_size: Some(2097), added: 2592, mode: MaxEncodedLen)
	fn set_collator_reward_per_session() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41`
		//  Estimated: `3582`
		// Minimum execution time: 8_957_000 picoseconds.
		Weight::from_parts(9_327_000, 0)
			.saturating_add(Weight::from_parts(0, 3582))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BlockRewards NextSessionChanges (r:1 w:1)
	/// Proof: BlockRewards NextSessionChanges (max_values: Some(1), max_size: Some(2097), added: 2592, mode: MaxEncodedLen)
	fn set_annual_treasury_inflation_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41`
		//  Estimated: `3582`
		// Minimum execution time: 9_107_000 picoseconds.
		Weight::from_parts(9_418_000, 0)
			.saturating_add(Weight::from_parts(0, 3582))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
