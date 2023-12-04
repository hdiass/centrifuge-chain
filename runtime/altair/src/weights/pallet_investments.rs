
//! Autogenerated weights for `pallet_investments`
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
// --pallet=pallet_investments
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_investments.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_investments`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_investments::WeightInfo for WeightInfo<T> {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:1 w:1)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrders (r:1 w:1)
	/// Proof: Investments InvestOrders (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:1 w:0)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	fn update_invest_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2122`
		//  Estimated: `6198`
		// Minimum execution time: 100_227_000 picoseconds.
		Weight::from_parts(101_310_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:1 w:1)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrders (r:1 w:1)
	/// Proof: Investments RedeemOrders (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:1 w:0)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	fn update_redeem_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2018`
		//  Estimated: `6198`
		// Minimum execution time: 99_896_000 picoseconds.
		Weight::from_parts(101_139_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrders (r:1 w:1)
	/// Proof: Investments InvestOrders (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:1 w:0)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:10 w:0)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments InvestmentState (r:1 w:0)
	/// Proof: ForeignInvestments InvestmentState (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn collect_investments(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2358 + n * (44 ±0)`
		//  Estimated: `6198 + n * (2555 ±0)`
		// Minimum execution time: 113_412_000 picoseconds.
		Weight::from_parts(109_253_017, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			// Standard Error: 25_573
			.saturating_add(Weight::from_parts(5_124_247, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2555).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrders (r:1 w:1)
	/// Proof: Investments RedeemOrders (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:1 w:0)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:10 w:0)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ForeignInvestments RedemptionState (r:1 w:0)
	/// Proof: ForeignInvestments RedemptionState (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn collect_redemptions(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2395 + n * (44 ±0)`
		//  Estimated: `6198 + n * (2555 ±0)`
		// Minimum execution time: 111_399_000 picoseconds.
		Weight::from_parts(107_538_629, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			// Standard Error: 19_959
			.saturating_add(Weight::from_parts(5_120_124, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 2555).saturating_mul(n.into()))
	}
}
