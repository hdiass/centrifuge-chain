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

use frame_support::weights::Weight;

pub trait WeightInfo {
	fn register(n: u32, m: u32) -> Weight;
	fn update_no_execution(n: u32, m: u32) -> Weight;
	fn update_and_execute(n: u32, m: u32) -> Weight;
	fn execute_update(n: u32, m: u32) -> Weight;
	fn set_metadata(n: u32, m: u32) -> Weight;
}

impl WeightInfo for () {
	fn register(_n: u32, _m: u32) -> Weight {
		Weight::zero()
	}

	fn update_no_execution(_n: u32, _m: u32) -> Weight {
		Weight::zero()
	}

	fn update_and_execute(_n: u32, _m: u32) -> Weight {
		Weight::zero()
	}

	fn execute_update(_n: u32, _m: u32) -> Weight {
		Weight::zero()
	}

	fn set_metadata(_n: u32, _m: u32) -> Weight {
		Weight::zero()
	}
}
