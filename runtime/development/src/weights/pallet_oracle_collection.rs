
//! Autogenerated weights for `pallet_oracle_collection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("centrifuge-local")`, DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-local
// --steps=50
// --repeat=20
// --pallet=pallet_oracle_collection
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_oracle_collection.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_oracle_collection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_oracle_collection::WeightInfo for WeightInfo<T> {
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::NotedChange` (r:0 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	fn propose_update_collection_info(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `222`
		//  Estimated: `3693`
		// Minimum execution time: 24_276_000 picoseconds.
		Weight::from_parts(24_755_058, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			// Standard Error: 8_436
			.saturating_add(Weight::from_parts(511_561, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PoolSystem::NotedChange` (r:1 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::Collection` (r:0 w:1)
	/// Proof: `OraclePriceCollection::Collection` (`max_values`: None, `max_size`: Some(7542), added: 10017, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::CollectionInfo` (r:0 w:1)
	/// Proof: `OraclePriceCollection::CollectionInfo` (`max_values`: None, `max_size`: Some(3058), added: 5533, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	fn apply_update_collection_info(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657 + n * (34 ±0)`
		//  Estimated: `8649`
		// Minimum execution time: 35_065_000 picoseconds.
		Weight::from_parts(35_541_621, 0)
			.saturating_add(Weight::from_parts(0, 8649))
			// Standard Error: 8_931
			.saturating_add(Weight::from_parts(522_188, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::Keys` (r:101 w:0)
	/// Proof: `OraclePriceCollection::Keys` (`max_values`: None, `max_size`: Some(95), added: 2570, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::CollectionInfo` (r:1 w:0)
	/// Proof: `OraclePriceCollection::CollectionInfo` (`max_values`: None, `max_size`: Some(3058), added: 5533, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceFeed::FedValues` (r:500 w:0)
	/// Proof: `OraclePriceFeed::FedValues` (`max_values`: None, `max_size`: Some(711), added: 3186, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::Collection` (r:0 w:1)
	/// Proof: `OraclePriceCollection::Collection` (`max_values`: None, `max_size`: Some(7542), added: 10017, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[1, 100]`.
	fn update_collection(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (326 ±0) + n * (5851 ±0)`
		//  Estimated: `16920 + m * (6039 ±164) + n * (100600 ±3_323)`
		// Minimum execution time: 129_342_000 picoseconds.
		Weight::from_parts(130_825_000, 0)
			.saturating_add(Weight::from_parts(0, 16920))
			// Standard Error: 15_277_105
			.saturating_add(Weight::from_parts(473_216_566, 0).saturating_mul(n.into()))
			// Standard Error: 756_691
			.saturating_add(Weight::from_parts(30_611_211, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().reads((31_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 6039).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 100600).saturating_mul(n.into()))
	}
}
