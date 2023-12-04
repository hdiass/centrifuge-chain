
//! Autogenerated weights for `pallet_loans`
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
// --pallet=pallet_loans
// --extrinsic=*
// --execution=wasm
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
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1196`
		//  Estimated: `4278`
		// Minimum execution time: 83_537_000 picoseconds.
		Weight::from_parts(84_167_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
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
		//  Measured:  `13046 + n * (340 ±0)`
		//  Estimated: `101291 + n * (340 ±0)`
		// Minimum execution time: 192_330_000 picoseconds.
		Weight::from_parts(196_048_493, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 23_666
			.saturating_add(Weight::from_parts(773_616, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
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
		//  Measured:  `13199 + n * (340 ±0)`
		//  Estimated: `101291 + n * (340 ±0)`
		// Minimum execution time: 167_504_000 picoseconds.
		Weight::from_parts(170_514_539, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 18_046
			.saturating_add(Weight::from_parts(571_046, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(Weight::from_parts(0, 340).saturating_mul(n.into()))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15906 + n * (340 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 259_507_000 picoseconds.
		Weight::from_parts(263_279_069, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 22_186
			.saturating_add(Weight::from_parts(733_536, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:1 w:0)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn admin_write_off(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `16157 + n * (340 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 274_774_000 picoseconds.
		Weight::from_parts(280_686_651, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 25_836
			.saturating_add(Weight::from_parts(630_561, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn propose_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `938 + n * (316 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 47_289_000 picoseconds.
		Weight::from_parts(48_738_445, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 11_107
			.saturating_add(Weight::from_parts(459_257, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn apply_loan_mutation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12243 + n * (340 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 94_126_000 picoseconds.
		Weight::from_parts(96_466_083, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 15_592
			.saturating_add(Weight::from_parts(579_892, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(182), added: 2657, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: Loans ClosedLoan (r:0 w:1)
	/// Proof: Loans ClosedLoan (max_values: None, max_size: Some(264), added: 2739, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(104), added: 2579, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(105), added: 2580, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12069 + n * (373 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 103_524_000 picoseconds.
		Weight::from_parts(106_067_643, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 13_218
			.saturating_add(Weight::from_parts(686_571, 0).saturating_mul(n.into()))
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
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	fn propose_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `512`
		//  Estimated: `4278`
		// Minimum execution time: 105_848_000 picoseconds.
		Weight::from_parts(106_509_000, 0)
			.saturating_add(Weight::from_parts(0, 4278))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans WriteOffPolicy (r:0 w:1)
	/// Proof: Loans WriteOffPolicy (max_values: None, max_size: Some(5126), added: 7601, mode: MaxEncodedLen)
	fn apply_write_off_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4887`
		//  Estimated: `8649`
		// Minimum execution time: 113_994_000 picoseconds.
		Weight::from_parts(116_558_000, 0)
			.saturating_add(Weight::from_parts(0, 8649))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: PriceCollector Collection (r:1 w:0)
	/// Proof: PriceCollector Collection (max_values: None, max_size: Some(11126), added: 13601, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:0 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 10]`.
	fn update_portfolio_valuation(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11696 + n * (316 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 78_487_000 picoseconds.
		Weight::from_parts(71_413_633, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 13_011
			.saturating_add(Weight::from_parts(9_365_622, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:0)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:0)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:0)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem NotedChange (r:0 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn propose_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11909 + n * (340 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 191_729_000 picoseconds.
		Weight::from_parts(193_274_892, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 27_330
			.saturating_add(Weight::from_parts(1_223_402, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem NotedChange (r:1 w:1)
	/// Proof: PoolSystem NotedChange (max_values: None, max_size: Some(5184), added: 7659, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:1)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(7250), added: 9725, mode: MaxEncodedLen)
	/// Storage: Loans ActiveLoans (r:1 w:1)
	/// Proof: Loans ActiveLoans (max_values: None, max_size: Some(97826), added: 100301, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: InterestAccrual Rates (r:1 w:1)
	/// Proof: InterestAccrual Rates (max_values: Some(1), max_size: Some(10802), added: 11297, mode: MaxEncodedLen)
	/// Storage: InterestAccrual LastUpdated (r:1 w:0)
	/// Proof: InterestAccrual LastUpdated (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans CreatedLoan (r:1 w:1)
	/// Proof: Loans CreatedLoan (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// The range of component `n` is `[2, 8]`.
	fn apply_transfer_debt(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12571 + n * (340 ±0)`
		//  Estimated: `101291`
		// Minimum execution time: 200_095_000 picoseconds.
		Weight::from_parts(202_119_702, 0)
			.saturating_add(Weight::from_parts(0, 101291))
			// Standard Error: 29_607
			.saturating_add(Weight::from_parts(1_227_309, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
