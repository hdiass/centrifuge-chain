// Copyright 2024 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_mocks::pallet_mock_liquidity_pools_gateway;
use cfg_primitives::LPGatewayQueueMessageNonce;
use cfg_types::domain_address::Domain;
use frame_support::derive_impl;
use sp_runtime::traits::ConstU128;

use crate::{self as pallet_liquidity_pools_gateway_queue, Config};

frame_support::construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		Balances: pallet_balances,
		LPGatewayMock: pallet_mock_liquidity_pools_gateway,
		LPGatewayQueue: pallet_liquidity_pools_gateway_queue,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountData = pallet_balances::AccountData<u128>;
	type Block = frame_system::mocking::MockBlock<Runtime>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Runtime {
	type AccountStore = System;
	type Balance = u128;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type RuntimeHoldReason = ();
}

impl pallet_mock_liquidity_pools_gateway::Config for Runtime {
	type Destination = Domain;
	type Message = ();
}

impl Config for Runtime {
	type Message = ();
	type MessageNonce = LPGatewayQueueMessageNonce;
	type MessageProcessor = LPGatewayMock;
	type RuntimeEvent = RuntimeEvent;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	System::externalities()
}
