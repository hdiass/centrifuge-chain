#![feature(stmt_expr_attributes)]
// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
#![cfg(test)]
#![allow(unused)]

mod generic;
mod rewards;
mod utils;

/// Re-exports the correct runtimes that we run the integration tests with
/// This allows all other modules to use the import of `crate::chain::{...}`
/// in order to get the right stuff from the respective runtime.
mod chain {
	pub mod centrifuge {
		pub use development::*;

		pub mod development {
			pub use development_runtime::*;
			pub const PARA_ID: u32 = 2000;
		}
	}

	pub mod relay {
		pub use rococo_runtime::*;
	}
}
