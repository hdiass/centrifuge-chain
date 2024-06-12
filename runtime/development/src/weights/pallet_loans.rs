
//! Autogenerated weights for `pallet_loans`
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
// --pallet=pallet_loans
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_loans.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_loans`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_loans::WeightInfo for WeightInfo<T> {
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:0)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(182), added: 2657, mode: `MaxEncodedLen`)
	/// Storage: `Loans::LastLoanId` (r:1 w:1)
	/// Proof: `Loans::LastLoanId` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Loans::CreatedLoan` (r:0 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:2)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1228`
		//  Estimated: `4278`
		// Minimum execution time: 74_619_000 picoseconds.
		Weight::from_parts(76_393_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::Accounts` (r:2 w:2)
	/// Proof: `OrmlTokens::Accounts` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn borrow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38081 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 257_271_000 picoseconds.
		Weight::from_parts(271_593_289, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 56_742
			.saturating_add(Weight::from_parts(66_655, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:0)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `OrmlTokens::Accounts` (r:2 w:2)
	/// Proof: `OrmlTokens::Accounts` (`max_values`: None, `max_size`: Some(129), added: 2604, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:1 w:0)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn repay(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38267 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 191_979_000 picoseconds.
		Weight::from_parts(195_609_642, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 24_834
			.saturating_add(Weight::from_parts(914_255, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Loans::WriteOffPolicy` (r:1 w:0)
	/// Proof: `Loans::WriteOffPolicy` (`max_values`: None, `max_size`: Some(5126), added: 7601, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41068 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 291_074_000 picoseconds.
		Weight::from_parts(306_451_616, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 65_729
			.saturating_add(Weight::from_parts(889_240, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Loans::WriteOffPolicy` (r:1 w:0)
	/// Proof: `Loans::WriteOffPolicy` (`max_values`: None, `max_size`: Some(5126), added: 7601, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `41319 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 336_618_000 picoseconds.
		Weight::from_parts(351_659_527, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 59_611
			.saturating_add(Weight::from_parts(790_271, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:0)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::NotedChange` (r:0 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn propose_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `971 + n * (316 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 41_297_000 picoseconds.
		Weight::from_parts(42_125_690, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 5_612
			.saturating_add(Weight::from_parts(468_060, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PoolSystem::NotedChange` (r:1 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:0)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn apply_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37404 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 121_217_000 picoseconds.
		Weight::from_parts(123_415_693, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 16_707
			.saturating_add(Weight::from_parts(571_387, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Loans::CreatedLoan` (r:1 w:0)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:0)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(182), added: 2657, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ClosedLoan` (r:0 w:1)
	/// Proof: `Loans::ClosedLoan` (`max_values`: None, `max_size`: Some(281), added: 2756, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:2)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(105), added: 2580, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn close(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37264 + n * (373 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 134_531_000 picoseconds.
		Weight::from_parts(142_641_726, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 26_601
			.saturating_add(Weight::from_parts(601_280, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::NotedChange` (r:0 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	fn propose_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `4278`
		// Minimum execution time: 99_716_000 picoseconds.
		Weight::from_parts(100_969_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PoolSystem::NotedChange` (r:1 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Loans::WriteOffPolicy` (r:0 w:1)
	/// Proof: `Loans::WriteOffPolicy` (`max_values`: None, `max_size`: Some(5126), added: 7601, mode: `MaxEncodedLen`)
	fn apply_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4854`
		//  Estimated: `8649`
		// Minimum execution time: 131_546_000 picoseconds.
		Weight::from_parts(132_908_000, 0)
			.saturating_add(Weight::from_parts(0, 8649))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:0)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::Collection` (r:1 w:0)
	/// Proof: `OraclePriceCollection::Collection` (`max_values`: None, `max_size`: Some(7542), added: 10017, mode: `MaxEncodedLen`)
	/// Storage: `OraclePriceCollection::CollectionInfo` (r:1 w:0)
	/// Proof: `OraclePriceCollection::CollectionInfo` (`max_values`: None, `max_size`: Some(3058), added: 5533, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:0)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:0 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 10]`.
	fn update_portfolio_valuation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36953 + n * (353 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 110_657_000 picoseconds.
		Weight::from_parts(80_744_391, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 40_449
			.saturating_add(Weight::from_parts(34_591_694, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Loans::PortfolioValuation` (r:1 w:0)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:0)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:0)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::CreatedLoan` (r:1 w:0)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::NotedChange` (r:0 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 8]`.
	fn propose_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37071 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 306_813_000 picoseconds.
		Weight::from_parts(320_875_517, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 85_096
			.saturating_add(Weight::from_parts(1_049_642, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `PoolSystem::NotedChange` (r:1 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 8]`.
	fn apply_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37732 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 317_713_000 picoseconds.
		Weight::from_parts(330_762_786, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 82_499
			.saturating_add(Weight::from_parts(726_621, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(36002), added: 36497, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn increase_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36731 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 200_694_000 picoseconds.
		Weight::from_parts(213_050_296, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 68_417
			.saturating_add(Weight::from_parts(887_808, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
