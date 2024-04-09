
//! Autogenerated weights for `pallet_loans`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_loans
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/development/src/weights/pallet_loans.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_loans`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_loans::WeightInfo for WeightInfo<T> {
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Loans LastLoanId (r:1 w:1)
	/// Proof: Loans LastLoanId (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:0 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1261`
		//  Estimated: `4278`
		// Minimum execution time: 80_350_000 picoseconds.
		Weight::from_parts(81_984_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn borrow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38651 + n * (340 ±0)`
		//  Estimated: `375491 + n * (340 ±0)`
		// Minimum execution time: 258_965_000 picoseconds.
		Weight::from_parts(273_923_939, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 57_114
			.saturating_add(Weight::from_parts(607_600, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn repay(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `38771 + n * (340 ±0)`
		//  Estimated: `375491 + n * (340 ±0)`
		// Minimum execution time: 191_809_000 picoseconds.
		Weight::from_parts(197_140_156, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 31_149
			.saturating_add(Weight::from_parts(875_000, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(535), added: 3010, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37360 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 240_039_000 picoseconds.
		Weight::from_parts(253_671_302, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 52_481
			.saturating_add(Weight::from_parts(736_804, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(535), added: 3010, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37611 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 251_641_000 picoseconds.
		Weight::from_parts(269_631_279, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 49_436
			.saturating_add(Weight::from_parts(898_913, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn propose_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `971 + n * (316 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 45_165_000 picoseconds.
		Weight::from_parts(45_298_209, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 12_320
			.saturating_add(Weight::from_parts(714_558, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn apply_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37477 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 99_717_000 picoseconds.
		Weight::from_parts(101_939_910, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 18_626
			.saturating_add(Weight::from_parts(640_848, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Loans ClosedLoan (r:0 w:1)
	/// Proof: Loans ClosedLoan (max_values: None, max_size: Some(280), added: 2755, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37370 + n * (373 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 142_897_000 picoseconds.
		Weight::from_parts(152_648_479, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 36_308
			.saturating_add(Weight::from_parts(464_350, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	fn propose_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `478`
		//  Estimated: `4278`
		// Minimum execution time: 44_153_000 picoseconds.
		Weight::from_parts(44_754_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:0 w:1)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(535), added: 3010, mode: MaxEncodedLen)
	fn apply_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1073`
		//  Estimated: `6581`
		// Minimum execution time: 43_742_000 picoseconds.
		Weight::from_parts(44_454_000, 0)
			.saturating_add(Weight::from_parts(0, 6581))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: OraclePriceCollection Collection (r:1 w:0)
	/// Proof: OraclePriceCollection Collection (max_values: None, max_size: Some(7542), added: 10017, mode: MaxEncodedLen)
	/// Storage: OraclePriceCollection CollectionInfo (r:1 w:0)
	/// Proof: OraclePriceCollection CollectionInfo (max_values: None, max_size: Some(3058), added: 5533, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:0 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn update_portfolio_valuation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37026 + n * (353 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 91_371_000 picoseconds.
		Weight::from_parts(86_053_471, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 42_303
			.saturating_add(Weight::from_parts(10_607_794, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn propose_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37144 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 283_661_000 picoseconds.
		Weight::from_parts(295_272_087, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 88_073
			.saturating_add(Weight::from_parts(1_367_999, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(3116), added: 5591, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn apply_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `37805 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 286_618_000 picoseconds.
		Weight::from_parts(301_381_602, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 88_453
			.saturating_add(Weight::from_parts(956_537, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(244), added: 2719, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(36002), added: 36497, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(372026), added: 374501, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn increase_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `36804 + n * (340 ±0)`
		//  Estimated: `375491`
		// Minimum execution time: 187_943_000 picoseconds.
		Weight::from_parts(195_618_379, 0)
			.saturating_add(Weight::from_parts(0, 375491))
			// Standard Error: 44_252
			.saturating_add(Weight::from_parts(1_090_526, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
