
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_identity.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 17_984_000 picoseconds.
		Weight::from_parts(19_196_076, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 2_244
			.saturating_add(Weight::from_parts(93_804, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(_: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:100 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `11003 + s * (2589 ±0)`
		// Minimum execution time: 13_385_000 picoseconds.
		Weight::from_parts(35_012_175, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 5_579
			.saturating_add(Weight::from_parts(5_033_322, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193 + p * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 13_555_000 picoseconds.
		Weight::from_parts(34_449_259, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 4_793
			.saturating_add(Weight::from_parts(2_021_801, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(_: u32, _: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(_: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(_: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 10_800_000 picoseconds.
		Weight::from_parts(11_560_340, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_436
			.saturating_add(Weight::from_parts(65_728, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 10_149_000 picoseconds.
		Weight::from_parts(10_749_756, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_211
			.saturating_add(Weight::from_parts(73_627, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:1)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 9_828_000 picoseconds.
		Weight::from_parts(10_486_472, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_079
			.saturating_add(Weight::from_parts(56_775, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity Registrars (r:1 w:0)
	/// Proof: Identity Registrars (max_values: Some(1), max_size: Some(1141), added: 1636, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(_: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: Identity IdentityOf (r:1 w:1)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:0 w:100)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(_: u32, _: u32) -> Weight {
        Weight::default()
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474 + s * (36 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 44_634_000 picoseconds.
		Weight::from_parts(49_922_675, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 1_300
			.saturating_add(Weight::from_parts(66_943, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `590 + s * (3 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 18_374_000 picoseconds.
		Weight::from_parts(20_536_491, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 565
			.saturating_add(Weight::from_parts(18_611, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Identity IdentityOf (r:1 w:0)
	/// Proof: Identity IdentityOf (max_values: None, max_size: Some(7538), added: 10013, mode: MaxEncodedLen)
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `637 + s * (35 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 48_681_000 picoseconds.
		Weight::from_parts(51_302_602, 0)
			.saturating_add(Weight::from_parts(0, 11003))
			// Standard Error: 890
			.saturating_add(Weight::from_parts(56_097, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Identity SuperOf (r:1 w:1)
	/// Proof: Identity SuperOf (max_values: None, max_size: Some(114), added: 2589, mode: MaxEncodedLen)
	/// Storage: Identity SubsOf (r:1 w:1)
	/// Proof: Identity SubsOf (max_values: None, max_size: Some(3258), added: 5733, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `770 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 35_296_000 picoseconds.
		Weight::from_parts(37_766_079, 0)
			.saturating_add(Weight::from_parts(0, 6723))
			// Standard Error: 809
			.saturating_add(Weight::from_parts(61_292, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}

    fn add_username_authority() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn remove_username_authority() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn set_username_for() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn accept_username() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn remove_expired_approval() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn set_primary_username() -> cumulus_primitives_core::Weight {
        Weight::default()
    }

    fn remove_dangling_username() -> cumulus_primitives_core::Weight {
        Weight::default()
    }
}
