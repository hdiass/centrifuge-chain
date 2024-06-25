
//! Autogenerated weights for `pallet_loans`
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
// --pallet=pallet_loans
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_loans.rs

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
		//  Measured:  `1195`
		//  Estimated: `4278`
		// Minimum execution time: 75_201_000 picoseconds.
		Weight::from_parts(78_006_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
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
		//  Measured:  `12846 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 186_288_000 picoseconds.
		Weight::from_parts(190_716_513, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 19_435
			.saturating_add(Weight::from_parts(587_402, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
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
		//  Measured:  `13032 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 165_729_000 picoseconds.
		Weight::from_parts(167_843_939, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 22_614
			.saturating_add(Weight::from_parts(691_747, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15833 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 196_697_000 picoseconds.
		Weight::from_parts(200_116_785, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 22_793
			.saturating_add(Weight::from_parts(426_786, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16084 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 232_884_000 picoseconds.
		Weight::from_parts(236_836_129, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 23_620
			.saturating_add(Weight::from_parts(527_055, 0).saturating_mul(n.into()))
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
		//  Measured:  `938 + n * (316 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 40_355_000 picoseconds.
		Weight::from_parts(40_780_970, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 5_395
			.saturating_add(Weight::from_parts(448_479, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn apply_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12169 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 106_780_000 picoseconds.
		Weight::from_parts(108_199_468, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 17_211
			.saturating_add(Weight::from_parts(729_848, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
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
		//  Measured:  `12029 + n * (373 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 92_222_000 picoseconds.
		Weight::from_parts(93_656_922, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 13_832
			.saturating_add(Weight::from_parts(650_957, 0).saturating_mul(n.into()))
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
		// Minimum execution time: 103_723_000 picoseconds.
		Weight::from_parts(105_928_000, 0)
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
		// Minimum execution time: 137_526_000 picoseconds.
		Weight::from_parts(140_211_000, 0)
			.saturating_add(Weight::from_parts(0, 8649))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:0)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
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
		//  Measured:  `11718 + n * (353 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 98_765_000 picoseconds.
		Weight::from_parts(69_862_207, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 26_661
			.saturating_add(Weight::from_parts(31_411_063, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::CreatedLoan` (r:1 w:0)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::NotedChange` (r:0 w:1)
	/// Proof: `PoolSystem::NotedChange` (`max_values`: None, `max_size`: Some(5184), added: 7659, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 8]`.
	fn propose_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11836 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 211_504_000 picoseconds.
		Weight::from_parts(216_578_752, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 34_460
			.saturating_add(Weight::from_parts(821_413, 0).saturating_mul(n.into()))
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
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 8]`.
	fn apply_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12497 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 222_114_000 picoseconds.
		Weight::from_parts(225_316_304, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 34_163
			.saturating_add(Weight::from_parts(948_074, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Loans::CreatedLoan` (r:1 w:1)
	/// Proof: `Loans::CreatedLoan` (`max_values`: None, `max_size`: Some(245), added: 2720, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::Rates` (r:1 w:1)
	/// Proof: `InterestAccrual::Rates` (`max_values`: Some(1), `max_size`: Some(10802), added: 11297, mode: `MaxEncodedLen`)
	/// Storage: `InterestAccrual::LastUpdated` (r:1 w:0)
	/// Proof: `InterestAccrual::LastUpdated` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Loans::PortfolioValuation` (r:1 w:1)
	/// Proof: `Loans::PortfolioValuation` (`max_values`: None, `max_size`: Some(24050), added: 26525, mode: `MaxEncodedLen`)
	/// Storage: `Loans::ActiveLoans` (r:1 w:1)
	/// Proof: `Loans::ActiveLoans` (`max_values`: None, `max_size`: Some(373026), added: 375501, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 9]`.
	fn increase_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11496 + n * (340 ±0)`
		//  Estimated: `376491`
		// Minimum execution time: 128_950_000 picoseconds.
		Weight::from_parts(131_986_528, 0)
			.saturating_add(Weight::from_parts(0, 376491))
			// Standard Error: 15_405
			.saturating_add(Weight::from_parts(538_412, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
