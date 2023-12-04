
//! Autogenerated weights for `pallet_liquidity_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_liquidity_rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_liquidity_rewards.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_liquidity_rewards`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_liquidity_rewards::WeightInfo for WeightInfo<T> {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: LiquidityRewards EndOfEpoch (r:1 w:0)
	/// Proof: LiquidityRewards EndOfEpoch (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `x` is `[0, 20]`.
	/// The range of component `y` is `[0, 50]`.
	/// The range of component `z` is `[0, 50]`.
	fn on_initialize(x: u32, y: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295`
		//  Estimated: `1493`
		// Minimum execution time: 9_128_000 picoseconds.
		Weight::from_parts(9_477_856, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			// Standard Error: 784
			.saturating_add(Weight::from_parts(5_659, 0).saturating_mul(x.into()))
			// Standard Error: 321
			.saturating_add(Weight::from_parts(1_649, 0).saturating_mul(y.into()))
			// Standard Error: 321
			.saturating_add(Weight::from_parts(10_543, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: LiquidityRewardsBase Currency (r:1 w:1)
	/// Proof: LiquidityRewardsBase Currency (max_values: None, max_size: Some(863), added: 3338, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase Group (r:1 w:1)
	/// Proof: LiquidityRewardsBase Group (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase StakeAccount (r:1 w:1)
	/// Proof: LiquidityRewardsBase StakeAccount (max_values: None, max_size: Some(143), added: 2618, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:1 w:0)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	fn stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `539`
		//  Estimated: `4328`
		// Minimum execution time: 40_245_000 picoseconds.
		Weight::from_parts(41_858_000, 0)
			.saturating_add(Weight::from_parts(0, 4328))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: LiquidityRewardsBase Currency (r:1 w:1)
	/// Proof: LiquidityRewardsBase Currency (max_values: None, max_size: Some(863), added: 3338, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase Group (r:1 w:1)
	/// Proof: LiquidityRewardsBase Group (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase StakeAccount (r:1 w:1)
	/// Proof: LiquidityRewardsBase StakeAccount (max_values: None, max_size: Some(143), added: 2618, mode: MaxEncodedLen)
	fn unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `176`
		//  Estimated: `4328`
		// Minimum execution time: 27_521_000 picoseconds.
		Weight::from_parts(28_262_000, 0)
			.saturating_add(Weight::from_parts(0, 4328))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: LiquidityRewardsBase Currency (r:1 w:0)
	/// Proof: LiquidityRewardsBase Currency (max_values: None, max_size: Some(863), added: 3338, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase Group (r:1 w:0)
	/// Proof: LiquidityRewardsBase Group (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: LiquidityRewardsBase StakeAccount (r:1 w:1)
	/// Proof: LiquidityRewardsBase StakeAccount (max_values: None, max_size: Some(143), added: 2618, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `449`
		//  Estimated: `4328`
		// Minimum execution time: 57_097_000 picoseconds.
		Weight::from_parts(57_999_000, 0)
			.saturating_add(Weight::from_parts(0, 4328))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: LiquidityRewards NextEpochChanges (r:1 w:1)
	/// Proof: LiquidityRewards NextEpochChanges (max_values: Some(1), max_size: Some(2078), added: 2573, mode: MaxEncodedLen)
	fn set_distributed_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3563`
		// Minimum execution time: 8_666_000 picoseconds.
		Weight::from_parts(9_037_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: LiquidityRewards NextEpochChanges (r:1 w:1)
	/// Proof: LiquidityRewards NextEpochChanges (max_values: Some(1), max_size: Some(2078), added: 2573, mode: MaxEncodedLen)
	fn set_epoch_duration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3563`
		// Minimum execution time: 8_506_000 picoseconds.
		Weight::from_parts(9_188_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: LiquidityRewards NextEpochChanges (r:1 w:1)
	/// Proof: LiquidityRewards NextEpochChanges (max_values: Some(1), max_size: Some(2078), added: 2573, mode: MaxEncodedLen)
	fn set_group_weight() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3563`
		// Minimum execution time: 9_107_000 picoseconds.
		Weight::from_parts(9_438_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: LiquidityRewards NextEpochChanges (r:1 w:1)
	/// Proof: LiquidityRewards NextEpochChanges (max_values: Some(1), max_size: Some(2078), added: 2573, mode: MaxEncodedLen)
	fn set_currency_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3563`
		// Minimum execution time: 9_107_000 picoseconds.
		Weight::from_parts(12_904_000, 0)
			.saturating_add(Weight::from_parts(0, 3563))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
