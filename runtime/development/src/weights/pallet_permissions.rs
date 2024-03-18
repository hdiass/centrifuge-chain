
//! Autogenerated weights for `pallet_permissions`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("development-local"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=development-local
// --steps=50
// --repeat=20
// --pallet=pallet_permissions
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_permissions.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_permissions`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_permissions::WeightInfo for WeightInfo<T> {
	/// Storage: Permissions PermissionCount (r:1 w:1)
	/// Proof: Permissions PermissionCount (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	/// Storage: Permissions Permission (r:1 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	fn add_as_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3693`
		// Minimum execution time: 21_230_000 picoseconds.
		Weight::from_parts(22_010_000, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Permissions Permission (r:2 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Permissions PermissionCount (r:1 w:1)
	/// Proof: Permissions PermissionCount (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	fn add_as_editor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `6396`
		// Minimum execution time: 28_213_000 picoseconds.
		Weight::from_parts(28_934_000, 0)
			.saturating_add(Weight::from_parts(0, 6396))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Permissions PermissionCount (r:1 w:1)
	/// Proof: Permissions PermissionCount (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	/// Storage: Permissions Permission (r:1 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	fn remove_as_admin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3693`
		// Minimum execution time: 23_995_000 picoseconds.
		Weight::from_parts(24_525_000, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Permissions Permission (r:2 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Permissions PermissionCount (r:1 w:1)
	/// Proof: Permissions PermissionCount (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	fn remove_as_editor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `6396`
		// Minimum execution time: 30_357_000 picoseconds.
		Weight::from_parts(31_138_000, 0)
			.saturating_add(Weight::from_parts(0, 6396))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Permissions Permission (r:1 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	fn purge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `3693`
		// Minimum execution time: 21_059_000 picoseconds.
		Weight::from_parts(21_319_000, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Permissions Permission (r:1 w:1)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	fn admin_purge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `3693`
		// Minimum execution time: 21_691_000 picoseconds.
		Weight::from_parts(22_081_000, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
