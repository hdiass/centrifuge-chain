
//! Autogenerated weights for `pallet_block_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

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
		//  Measured:  `678`
		//  Estimated: `12885`
		// Minimum execution time: 59_271 nanoseconds.
		Weight::from_parts(60_203_000, 12885)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: BlockRewards NextSessionChanges (r:1 w:1)
	/// Proof: BlockRewards NextSessionChanges (max_values: Some(1), max_size: Some(2089), added: 2584, mode: MaxEncodedLen)
	/// Storage: BlockRewards ActiveSessionData (r:1 w:0)
	/// Proof: BlockRewards ActiveSessionData (max_values: Some(1), max_size: Some(36), added: 531, mode: MaxEncodedLen)
	fn set_collator_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `138`
		//  Estimated: `3115`
		// Minimum execution time: 12_073 nanoseconds.
		Weight::from_parts(12_533_000, 3115)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: BlockRewards NextSessionChanges (r:1 w:1)
	/// Proof: BlockRewards NextSessionChanges (max_values: Some(1), max_size: Some(2089), added: 2584, mode: MaxEncodedLen)
	/// Storage: BlockRewards ActiveSessionData (r:1 w:0)
	/// Proof: BlockRewards ActiveSessionData (max_values: Some(1), max_size: Some(36), added: 531, mode: MaxEncodedLen)
	fn set_total_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `3115`
		// Minimum execution time: 11_140 nanoseconds.
		Weight::from_parts(11_442_000, 3115)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
