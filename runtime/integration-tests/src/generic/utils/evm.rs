use std::{
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use ethabi::{ethereum_types::H160, Contract};
use pallet_evm::CreateInfo;

use crate::generic::utils::ESSENTIAL;

/// Liquidity-Pool solidity artifacts generated by build-script.
/// All needed contracts can be loaded from here
pub const LP_SOL_SOURCES: &str = env!("LP_SOL_SOURCES", "Build script failed to populate environment variable LP_SOL_SOURCES pointing to solidity source files.");

pub const IRREGULAR: &str = "__$a334d32fe50f4079904586988e51d312a5$__";

#[derive(Clone, Debug, PartialEq)]
pub struct DeployedContractInfo {
	pub contract: Contract,
	pub deployed_bytecode: Vec<u8>,
	pub create_info: CreateInfo,
}

impl DeployedContractInfo {
	pub fn new(contract: Contract, deployed_bytecode: Vec<u8>, create_info: CreateInfo) -> Self {
		Self {
			create_info,
			contract,
			deployed_bytecode,
		}
	}

	pub fn address(&self) -> H160 {
		H160::from(self.create_info.value.0)
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContractInfo {
	pub contract: Contract,
	pub bytecode: Vec<u8>,
	pub deployed_bytecode: Vec<u8>,
}

impl ContractInfo {
	pub fn new(contract: Contract, bytecode: Vec<u8>, deployed_bytecode: Vec<u8>) -> Self {
		Self {
			contract,
			bytecode,
			deployed_bytecode,
		}
	}
}

fn traversal(path: impl AsRef<Path>, files: &mut Vec<PathBuf>) {
	for path in fs::read_dir(path).expect("Submodules directory must exist for integration-tests") {
		if let Ok(dir_entry) = path.as_ref() {
			if dir_entry
				.metadata()
				.map(|meta| meta.is_dir())
				.unwrap_or(false)
			{
				traversal(
					fs::canonicalize(dir_entry.path()).expect("Failed to find absolute path."),
					files,
				)
			} else if dir_entry
				.metadata()
				.map(|meta| meta.is_file())
				.unwrap_or(false)
			{
				files.push(dir_entry.path())
			}
		}
	}
}

pub fn fetch_contracts() -> HashMap<String, ContractInfo> {
	let mut contracts = HashMap::new();
	let mut files = Vec::new();
	traversal(LP_SOL_SOURCES, &mut files);
	files.iter().for_each(|path| {
		let file_name = path
			.file_name()
			.expect("Only files here. qed.")
			.to_str()
			.expect(".sol files are valid unicode. qed")
			.split(".")
			.collect::<Vec<_>>();

		let contract_name = file_name
			.first()
			.expect("Files are all x.json.qed")
			.to_string();

		let contract_json: serde_json::Value =
			serde_json::from_reader(fs::File::open(path).expect(ESSENTIAL)).expect(ESSENTIAL);
		let abi = contract_json.get("abi").expect(ESSENTIAL);
		let _ = Contract::load(&mut serde_json::to_string(abi).expect(ESSENTIAL).as_bytes())
			.map_err(|e| {
				println!(
					"Error: Failed loading contract {}. Error: {}",
					contract_name, e
				)
			})
			.map(|contract| {
				// NOTE: We do not care of the code is invalid for now.
				let _ = hex::decode(
					contract_json
						.get("bytecode")
						.expect(ESSENTIAL)
						.get("object")
						.expect(ESSENTIAL)
						.as_str()
						.expect(ESSENTIAL)
						.trim_start_matches("0x")
						.replace(IRREGULAR, ""),
				)
				.map_err(|e| {
					println!(
						"Error: Failed decoding contract code {}. Error: {}",
						contract_name, e,
					)
				})
				.and_then(|code| {
					// NOTE: We do not care of the code is invalid for now.
					let deployed = hex::decode(
						contract_json
							.get("deployedBytecode")
							.expect(ESSENTIAL)
							.get("object")
							.expect(ESSENTIAL)
							.as_str()
							.expect(ESSENTIAL)
							.trim_start_matches("0x")
							.replace(IRREGULAR, ""),
					)
					.map_err(|e| {
						println!(
							"Error: Failed decoding deployed contract code {}. Error: {}",
							contract_name, e,
						)
					})?;

					Ok((code, deployed))
				})
				.map(|(code, deployed)| {
					// NOTE: There are some overlapping contract names in the LP codebase atm, but
					//       non that we care about for now. If we do care, we need to have some
					//       prefix or sort here.
					//
					//       For now: Use latest contract.
					contracts.insert(contract_name, ContractInfo::new(contract, code, deployed));
				});
			});
	});

	contracts
}
