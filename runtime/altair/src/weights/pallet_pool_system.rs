
//! Autogenerated weights for `pallet_pool_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_pool_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_pool_system.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_pool_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_system::WeightInfo for WeightInfo<T> {
	/// Storage: Permissions Permission (r:1 w:0)
	/// Proof: Permissions Permission (max_values: None, max_size: Some(228), added: 2703, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	fn set_max_reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `5991`
		// Minimum execution time: 30_998 nanoseconds.
		Weight::from_parts(31_799_000, 5991)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:0)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:5 w:0)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_no_orders(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `907 + n * (133 ±0)`
		//  Estimated: `33561 + n * (20298 ±0)`
		// Minimum execution time: 130_624 nanoseconds.
		Weight::from_parts(53_072_261, 33561)
			// Standard Error: 49_992
			.saturating_add(Weight::from_ref_time(79_902_734).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(20298).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:0)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_no_execution(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1073 + n * (133 ±0)`
		//  Estimated: `33561 + n * (17694 ±0)`
		// Minimum execution time: 87_393 nanoseconds.
		Weight::from_parts(56_041_104, 33561)
			// Standard Error: 29_589
			.saturating_add(Weight::from_ref_time(33_508_964).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(17694).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: PoolSystem EpochExecution (r:1 w:0)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans PortfolioValuation (r:1 w:0)
	/// Proof: Loans PortfolioValuation (max_values: None, max_size: Some(24050), added: 26525, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:5 w:1)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:5)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:5)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:7 w:3)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn close_epoch_execute(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1591 + n * (133 ±0)`
		//  Estimated: `43975 + n * (20298 ±0)`
		// Minimum execution time: 221_963 nanoseconds.
		Weight::from_parts(143_410_171, 43975)
			// Standard Error: 52_280
			.saturating_add(Weight::from_ref_time(81_679_027).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(20298).saturating_mul(n.into()))
	}
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:0)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn submit_solution(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493 + n * (249 ±0)`
		//  Estimated: `6533`
		// Minimum execution time: 31_979 nanoseconds.
		Weight::from_parts(32_038_410, 6533)
			// Standard Error: 11_202
			.saturating_add(Weight::from_ref_time(928_430).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PoolSystem EpochExecution (r:1 w:1)
	/// Proof: PoolSystem EpochExecution (max_values: None, max_size: Some(770), added: 3245, mode: MaxEncodedLen)
	/// Storage: PoolSystem Pool (r:1 w:1)
	/// Proof: PoolSystem Pool (max_values: None, max_size: Some(813), added: 3288, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingInvestOrders (r:5 w:5)
	/// Proof: Investments InProcessingInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrmlTokens Accounts (r:7 w:3)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrmlTokens TotalIssuance (r:1 w:1)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Investments InvestOrderId (r:5 w:0)
	/// Proof: Investments InvestOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveInvestOrders (r:5 w:5)
	/// Proof: Investments ActiveInvestOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments InProcessingRedeemOrders (r:5 w:5)
	/// Proof: Investments InProcessingRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Investments RedeemOrderId (r:5 w:0)
	/// Proof: Investments RedeemOrderId (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Investments ActiveRedeemOrders (r:5 w:5)
	/// Proof: Investments ActiveRedeemOrders (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Investments ClearedInvestOrders (r:0 w:5)
	/// Proof: Investments ClearedInvestOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// Storage: Investments ClearedRedeemOrders (r:0 w:5)
	/// Proof: Investments ClearedRedeemOrders (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn execute_epoch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1503 + n * (605 ±0)`
		//  Estimated: `19974 + n * (17774 ±0)`
		// Minimum execution time: 184_965 nanoseconds.
		Weight::from_parts(131_175_488, 19974)
			// Standard Error: 50_811
			.saturating_add(Weight::from_ref_time(58_136_652).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(8))
			.saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_proof_size(17774).saturating_mul(n.into()))
	}
}
