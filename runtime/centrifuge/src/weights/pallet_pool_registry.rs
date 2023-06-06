
//! Autogenerated weights for `pallet_pool_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
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
	// Storage: Permissions PermissionCount (r:1 w:1)
	// Storage: Permissions Permission (r:1 w:1)
	// Storage: PoolSystem PoolDeposit (r:0 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn register(n: u32, ) -> Weight {
		// Minimum execution time: 108_001 nanoseconds.
		Weight::from_ref_time(95_520_181)
			// Standard Error: 38_092
			.saturating_add(Weight::from_ref_time(15_912_320).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolSystem EpochExecution (r:1 w:0)
	// Storage: PoolSystem Pool (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Investments ActiveRedeemOrders (r:1 w:0)
	// Storage: PoolSystem ScheduledUpdate (r:0 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn update_no_execution(n: u32, ) -> Weight {
		// Minimum execution time: 61_345 nanoseconds.
		Weight::from_ref_time(61_293_148)
			// Standard Error: 15_285
			.saturating_add(Weight::from_ref_time(2_286_521).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
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
		// Minimum execution time: 92_363 nanoseconds.
		Weight::from_ref_time(87_110_520)
			// Standard Error: 30_201
			.saturating_add(Weight::from_ref_time(7_930_760).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PoolSystem Pool (r:1 w:1)
	// Storage: PoolSystem EpochExecution (r:1 w:0)
	// Storage: PoolSystem ScheduledUpdate (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Investments ActiveRedeemOrders (r:1 w:0)
	// Storage: OrmlAssetRegistry Metadata (r:2 w:1)
	/// The range of component `n` is `[1, 5]`.
	fn execute_update(n: u32, ) -> Weight {
		// Minimum execution time: 80_120 nanoseconds.
		Weight::from_ref_time(74_863_525)
			// Standard Error: 23_948
			.saturating_add(Weight::from_ref_time(7_884_790).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Permissions Permission (r:1 w:0)
	// Storage: PoolRegistry PoolMetadata (r:0 w:1)
	/// The range of component `n` is `[0, 46]`.
	fn set_metadata(n: u32, ) -> Weight {
		// Minimum execution time: 33_432 nanoseconds.
		Weight::from_ref_time(35_144_895)
			// Standard Error: 1_253
			.saturating_add(Weight::from_ref_time(18_014).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
