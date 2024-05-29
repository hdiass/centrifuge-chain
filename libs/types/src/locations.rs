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
use frame_support::RuntimeDebugNoBound;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use staging_xcm::VersionedLocation;

use crate::domain_address::DomainAddress;
/// Location types for destinations that can receive restricted transfers
#[derive(Clone, RuntimeDebugNoBound, Encode, Decode, Eq, PartialEq, MaxEncodedLen, TypeInfo)]
pub enum RestrictedTransferLocation {
	/// Local chain account sending destination.
	Local(AccountId),
	/// XCM MultiLocation sending destinations.
	/// Using hash value here as Multilocation is large -- v1 is 512 bytes, but
	/// next largest is only 40 bytes other values aren't hashed as we have
	/// blake2 hashing on storage map keys, and we don't want the extra overhead
	XCM(H256),
	/// DomainAddress sending location from a liquidity pools' instance
	Address(DomainAddress),
}

impl From<AccountId32> for RestrictedTransferLocation {
	fn from(value: AccountId32) -> Self {
		Self::Local(value)
	}
}

impl From<VersionedLocation> for RestrictedTransferLocation {
	fn from(vml: VersionedLocation) -> Self {
		// using hash here as multilocation is significantly larger than any other enum
		// type here -- 592 bytes, vs 40 bytes for domain address (next largest)
		Self::XCM(BlakeTwo256::hash(&vml.encode()))

		// TODO-1.7: I'm afraid of locations translated from v3 to v4 will
		// generate a different hash here. How this affect our current chain
		// state?
	}
}

impl From<DomainAddress> for RestrictedTransferLocation {
	fn from(da: DomainAddress) -> Self {
		Self::Address(da)
	}
}

#[cfg(test)]
mod test {

	use hex::FromHex;
	use staging_xcm::v4::Location;

	use super::*;

	#[test]
	fn from_xcm_versioned_address_works() {
		let xa = VersionedLocation::V4(Location::default());
		let l = RestrictedTransferLocation::from(xa.clone());
		assert_eq!(
			l,
			RestrictedTransferLocation::XCM(sp_core::H256(
				<[u8; 32]>::from_hex(
					"a943e30c855a123a9506e69e678dc65ae9f5b70149cb6b26eb2ed58a59b4bf77"
				)
				.unwrap()
			))
		);
	}

	#[test]
	fn from_domain_address_works() {
		let da = DomainAddress::EVM(
			1284,
			<[u8; 20]>::from_hex("1231231231231231231231231231231231231231").unwrap(),
		);
		let l = RestrictedTransferLocation::from(da.clone());

		assert_eq!(l, RestrictedTransferLocation::Address(da))
	}
}
