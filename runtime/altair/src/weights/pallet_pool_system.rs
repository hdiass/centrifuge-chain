
//! Autogenerated weights for `pallet_pool_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("altair-local")`, DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-local
// --steps=50
// --repeat=20
// --pallet=pallet_pool_system
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_pool_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_pool_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_system::WeightInfo for WeightInfo<T> {
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[0, 100]`.
	fn set_max_reserve(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `554`
		//  Estimated: `4278`
		// Minimum execution time: 24_425_000 picoseconds.
		Weight::from_parts(25_132_176, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			// Standard Error: 539
			.saturating_add(Weight::from_parts(31_636, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:0)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:0)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::PortfolioValuation` (r:1 w:1)
	/// Proof: `PoolFees::PortfolioValuation` (`max_values`: None, `max_size`: Some(4850), added: 7325, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::AssetsUnderManagement` (r:1 w:1)
	/// Proof: `PoolFees::AssetsUnderManagement` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:1)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::TotalIssuance` (r:5 w:0)
	/// Proof: `OrmlTokens::TotalIssuance` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveInvestOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingInvestOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InvestOrderId` (r:5 w:5)
	/// Proof: `Investments::InvestOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::RedeemOrderId` (r:5 w:5)
	/// Proof: `Investments::RedeemOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::Accounts` (r:5 w:0)
	/// Proof: `OrmlTokens::Accounts` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedInvestOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedInvestOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedRedeemOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedRedeemOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[1, 100]`.
	fn close_epoch_no_orders(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1150 + m * (124 ±0) + n * (133 ±0)`
		//  Estimated: `27515 + n * (2604 ±0)`
		// Minimum execution time: 442_405_000 picoseconds.
		Weight::from_parts(84_600_408, 0)
			.saturating_add(Weight::from_parts(0, 27515))
			// Standard Error: 136_918
			.saturating_add(Weight::from_parts(71_704_522, 0).saturating_mul(n.into()))
			// Standard Error: 6_323
			.saturating_add(Weight::from_parts(3_447_067, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2604).saturating_mul(n.into()))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:1)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:0)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::PortfolioValuation` (r:1 w:1)
	/// Proof: `PoolFees::PortfolioValuation` (`max_values`: None, `max_size`: Some(4850), added: 7325, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::AssetsUnderManagement` (r:1 w:1)
	/// Proof: `PoolFees::AssetsUnderManagement` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:1)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::TotalIssuance` (r:5 w:0)
	/// Proof: `OrmlTokens::TotalIssuance` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveInvestOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingInvestOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InvestOrderId` (r:5 w:5)
	/// Proof: `Investments::InvestOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::RedeemOrderId` (r:5 w:5)
	/// Proof: `Investments::RedeemOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn close_epoch_no_execution(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1362 + m * (124 ±0) + n * (133 ±0)`
		//  Estimated: `27515 + n * (2531 ±0)`
		// Minimum execution time: 227_845_000 picoseconds.
		Weight::from_parts(97_065_667, 0)
			.saturating_add(Weight::from_parts(0, 27515))
			// Standard Error: 103_500
			.saturating_add(Weight::from_parts(27_629_142, 0).saturating_mul(n.into()))
			// Standard Error: 4_723
			.saturating_add(Weight::from_parts(3_198_067, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:0)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:0)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::PortfolioValuation` (r:1 w:1)
	/// Proof: `PoolFees::PortfolioValuation` (`max_values`: None, `max_size`: Some(4850), added: 7325, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::AssetsUnderManagement` (r:1 w:1)
	/// Proof: `PoolFees::AssetsUnderManagement` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:1)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::TotalIssuance` (r:5 w:1)
	/// Proof: `OrmlTokens::TotalIssuance` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveInvestOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingInvestOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InvestOrderId` (r:5 w:5)
	/// Proof: `Investments::InvestOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::RedeemOrderId` (r:5 w:5)
	/// Proof: `Investments::RedeemOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::Accounts` (r:7 w:3)
	/// Proof: `OrmlTokens::Accounts` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:2 w:0)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedInvestOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedInvestOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedRedeemOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedRedeemOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn close_epoch_execute(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2054 + m * (124 ±0) + n * (167 ±0)`
		//  Estimated: `27515 + n * (2604 ±0)`
		// Minimum execution time: 537_503_000 picoseconds.
		Weight::from_parts(180_573_847, 0)
			.saturating_add(Weight::from_parts(0, 27515))
			// Standard Error: 169_949
			.saturating_add(Weight::from_parts(74_071_744, 0).saturating_mul(n.into()))
			// Standard Error: 7_755
			.saturating_add(Weight::from_parts(3_460_352, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(10))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2604).saturating_mul(n.into()))
	}
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:1)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:0)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn submit_solution(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `788 + m * (124 ±0) + n * (249 ±0)`
		//  Estimated: `17508`
		// Minimum execution time: 39_894_000 picoseconds.
		Weight::from_parts(33_385_816, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 36_096
			.saturating_add(Weight::from_parts(1_602_761, 0).saturating_mul(n.into()))
			// Standard Error: 1_647
			.saturating_add(Weight::from_parts(207_846, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:1)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:1)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingInvestOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::Accounts` (r:7 w:3)
	/// Proof: `OrmlTokens::Accounts` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:2 w:0)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::TotalIssuance` (r:1 w:1)
	/// Proof: `OrmlTokens::TotalIssuance` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InvestOrderId` (r:5 w:0)
	/// Proof: `Investments::InvestOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveInvestOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveInvestOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::InProcessingRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::InProcessingRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Investments::RedeemOrderId` (r:5 w:0)
	/// Proof: `Investments::RedeemOrderId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:5)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedInvestOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedInvestOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ClearedRedeemOrders` (r:0 w:5)
	/// Proof: `Investments::ClearedRedeemOrders` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn execute_epoch(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2009 + m * (124 ±0) + n * (633 ±0)`
		//  Estimated: `17508 + n * (2604 ±0)`
		// Minimum execution time: 240_970_000 picoseconds.
		Weight::from_parts(149_766_811, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 115_506
			.saturating_add(Weight::from_parts(56_480_792, 0).saturating_mul(n.into()))
			// Standard Error: 5_271
			.saturating_add(Weight::from_parts(391_702, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(9))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2604).saturating_mul(n.into()))
	}
}
