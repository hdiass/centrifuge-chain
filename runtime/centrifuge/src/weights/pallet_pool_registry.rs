
//! Autogenerated weights for `pallet_pool_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_pool_registry
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_pool_registry.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_pool_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_registry::WeightInfo for WeightInfo<T> {
	// Storage: PoolRegistry Pools (r:1 w:1)
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: PoolSystem AccountDeposit (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: OrmlAssetRegistry LocationToAssetId (r:1 w:1)
	// Storage: Permissions PermissionCount (r:1 w:1)
	// Storage: Permissions Permission (r:1 w:1)
	// Storage: PoolSystem PoolDeposit (r:0 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn register(n: u32, ) -> Weight {
		// Minimum execution time: 154_103 nanoseconds.
		Weight::from_ref_time(130_627_382 as u64)
			// Standard Error: 49_140
			.saturating_add(Weight::from_ref_time(28_006_696 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolSystem EpochExecution (r:1 w:0)
	// Storage: PoolSystem Pool (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Investments ActiveRedeemOrders (r:1 w:0)
	// Storage: PoolSystem ScheduledUpdate (r:0 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn update_no_execution(n: u32, ) -> Weight {
		// Minimum execution time: 80_302 nanoseconds.
		Weight::from_ref_time(78_714_119 as u64)
			// Standard Error: 16_407
			.saturating_add(Weight::from_ref_time(2_998_494 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolSystem EpochExecution (r:1 w:0)
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Investments ActiveRedeemOrders (r:1 w:0)
	// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	// Storage: PoolSystem ScheduledUpdate (r:0 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn update_and_execute(n: u32, ) -> Weight {
		// Minimum execution time: 120_203 nanoseconds.
		Weight::from_ref_time(113_234_223 as u64)
			// Standard Error: 39_004
			.saturating_add(Weight::from_ref_time(9_668_906 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: PoolSystem EpochExecution (r:1 w:0)
	// Storage: PoolSystem ScheduledUpdate (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Investments ActiveRedeemOrders (r:1 w:0)
	// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn execute_update(n: u32, ) -> Weight {
		// Minimum execution time: 107_402 nanoseconds.
		Weight::from_ref_time(99_305_869 as u64)
			// Standard Error: 37_358
			.saturating_add(Weight::from_ref_time(10_036_394 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolRegistry PoolMetadata (r:0 w:1)
	/// The range of component `n` is `[0, 46]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Minimum execution time: 43_501 nanoseconds.
		Weight::from_ref_time(45_756_984 as u64)
			// Standard Error: 8_729
			.saturating_add(Weight::from_ref_time(18_369 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
