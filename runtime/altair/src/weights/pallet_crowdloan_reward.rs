
//! Autogenerated weights for `pallet_crowdloan_reward`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_crowdloan_reward
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_crowdloan_reward.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_crowdloan_reward`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_crowdloan_reward::WeightInfo for WeightInfo<T> {
	// Storage: CrowdloanReward VestingStart (r:0 w:1)
	// Storage: CrowdloanReward VestingPeriod (r:0 w:1)
	// Storage: CrowdloanReward DirectPayoutRatio (r:0 w:1)
	fn initialize() -> Weight {
		// Minimum execution time: 28_200 nanoseconds.
		Weight::from_ref_time(29_600_000 as u64)
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: CrowdloanReward VestingStart (r:0 w:1)
	fn set_vesting_start() -> Weight {
		// Minimum execution time: 25_701 nanoseconds.
		Weight::from_ref_time(26_600_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CrowdloanReward VestingPeriod (r:0 w:1)
	fn set_vesting_period() -> Weight {
		// Minimum execution time: 24_400 nanoseconds.
		Weight::from_ref_time(26_801_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CrowdloanReward DirectPayoutRatio (r:0 w:1)
	fn set_direct_payout_ratio() -> Weight {
		// Minimum execution time: 25_601 nanoseconds.
		Weight::from_ref_time(26_801_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
