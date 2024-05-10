// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_mocks::pallet_mock_fees;
use frame_support::{
	derive_impl,
	traits::{ConstU64, ConstU8},
};
use sp_runtime::{
	traits::{ConstBool, ConstU32},
	BuildStorage,
};

use crate::{self as pallet_anchors, Config};

pub const COMMIT_FEE_KEY: u8 = 1;
pub const COMMIT_FEE_VALUE: Balance = 23;

pub const PRE_COMMIT_FEE_KEY: u8 = 2;
pub const PRE_COMMIT_FEE_VALUE: Balance = 42;

type Balance = u64;

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Aura: pallet_aura,
		MockFees: pallet_mock_fees,
		Anchors: pallet_anchors,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountData = pallet_balances::AccountData<Balance>;
	type Block = frame_system::mocking::MockBlock<Runtime>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Runtime {
	type AccountStore = System;
}

impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = ConstU64<6000>;
	type Moment = u64;
	type OnTimestampSet = ();
	type WeightInfo = ();
}

impl pallet_mock_fees::Config for Runtime {
	type Balance = Balance;
	type FeeKey = u8;
}

impl pallet_aura::Config for Runtime {
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
	type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<32>;
}

impl Config for Runtime {
	type CommitAnchorFeeKey = ConstU8<COMMIT_FEE_KEY>;
	type Currency = Balances;
	type Fees = MockFees;
	type PreCommitDepositFeeKey = ConstU8<PRE_COMMIT_FEE_KEY>;
	type WeightInfo = ();
}

impl Runtime {
	pub fn test_document_hashes() -> (
		<Runtime as frame_system::Config>::Hash,
		<Runtime as frame_system::Config>::Hash,
		<Runtime as frame_system::Config>::Hash,
	) {
		// first is the hash of concatenated last two in sorted order
		(
			// doc_root
			[
				238, 250, 118, 84, 35, 55, 212, 193, 69, 104, 25, 244, 240, 31, 54, 36, 85, 171,
				12, 71, 247, 81, 74, 10, 127, 127, 185, 158, 253, 100, 206, 130,
			]
			.into(),
			// signing root
			[
				63, 39, 76, 249, 122, 12, 22, 110, 110, 63, 161, 193, 10, 51, 83, 226, 96, 179,
				203, 22, 42, 255, 135, 63, 160, 26, 73, 222, 175, 198, 94, 200,
			]
			.into(),
			// proof hash
			[
				192, 195, 141, 209, 99, 91, 39, 154, 243, 6, 188, 4, 144, 5, 89, 252, 52, 105, 112,
				173, 143, 101, 65, 6, 191, 206, 210, 2, 176, 103, 161, 14,
			]
			.into(),
		)
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap();

	// pre-fill balances
	// 100 is the block author
	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(1, 100000), (2, 100000), (100, 100)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		MockFees::mock_fee_value(|key| match key {
			COMMIT_FEE_KEY => COMMIT_FEE_VALUE,
			PRE_COMMIT_FEE_KEY => PRE_COMMIT_FEE_VALUE,
			_ => panic!("No valid fee key"),
		});
		MockFees::mock_fee_to_author(|_, _| Ok(()));
	});
	ext
}
