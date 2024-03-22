// Copyright 2023 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::AccountId;
use pallet_evm::AddressMapping;
use polkadot_parachain_primitives::primitives::Sibling;
use sp_core::{crypto::AccountId32, Get, H160};
use sp_runtime::traits::AccountIdConversion;

use crate::account_conversion::AccountMapping;

pub fn get_gateway_account<T: pallet_evm_chain_id::Config + parachain_info::Config>() -> AccountId {
	let sender_account: AccountId =
		Sibling::from(parachain_info::Pallet::<T>::get()).into_account_truncating();

	let truncated_sender_account =
		H160::from_slice(&<AccountId32 as AsRef<[u8; 32]>>::as_ref(&sender_account)[0..20]);

	AccountMapping::<T>::into_account_id(truncated_sender_account)
}
