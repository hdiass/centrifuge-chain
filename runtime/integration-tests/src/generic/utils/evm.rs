use std::{
	collections::HashMap,
	fs,
	path::{Path, PathBuf},
};

use cfg_utils::vec_to_fixed_array;
use ethabi::{ethereum_types::H160, Contract};
use ethereum::ReceiptV3;
use frame_support::traits::{fungible::Mutate, OriginTrait};
use pallet_evm::FeeCalculator;
use runtime_common::account_conversion::AccountConverter;
use sp_runtime::traits::Get;

use crate::generic::{config::Runtime, utils::ESSENTIAL};

/// Liquidity-Pool solidity artifacts generated by build-script.
/// All needed contracts can be loaded from here.
///
/// This panics if the solidity contracts were not built properly. This can
/// happen if the submodule was not pulled or the forge cli has not been
/// installed locally.
pub const LP_SOL_SOURCES: &str = env!("LP_SOL_SOURCES", "Build script failed to populate environment variable LP_SOL_SOURCES pointing to missing solidity source files in the 'target/*/build/integration-tests*/out' directory required for EVM integration tests.\n\nPlease check if you have pulled the 'liquidity-pools' submodule via `git pull --recurse-submodules` and if you have installed the forge cli, e.g. check `forge -V`.");

#[derive(Clone, Debug, PartialEq)]
pub struct DeployedContractInfo {
	pub contract: Contract,
	pub deployed_bytecode: Vec<u8>,
	pub address: H160,
}

impl DeployedContractInfo {
	pub fn new(contract: Contract, deployed_bytecode: Vec<u8>, address: H160) -> Self {
		Self {
			address,
			contract,
			deployed_bytecode,
		}
	}

	pub fn address(&self) -> H160 {
		H160::from(self.address)
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

fn is_sol_directory(dir_entry: &std::fs::DirEntry) -> bool {
	dir_entry
		.path()
		.parent()
		.expect(ESSENTIAL)
		.extension()
		.map(|s| s.to_str().expect(ESSENTIAL))
		== Some("sol")
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
				if is_sol_directory(dir_entry) {
					files.push(dir_entry.path())
				}
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
						.trim_start_matches("0x"),
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
							.trim_start_matches("0x"),
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

pub fn receipt_ok(receipt: ReceiptV3) -> bool {
	let inner = match receipt {
		ReceiptV3::Legacy(inner) | ReceiptV3::EIP1559(inner) | ReceiptV3::EIP2930(inner) => inner,
	};

	inner.status_code == 1
}

/// TEST COMMENT
pub fn mint_balance_into_derived_account<T: Runtime>(address: impl AsRef<[u8]>, balance: u128) {
	let chain_id = pallet_evm_chain_id::Pallet::<T>::get();
	let derived_account =
		AccountConverter::convert_evm_address(chain_id, vec_to_fixed_array(address));
	pallet_balances::Pallet::<T>::mint_into(&derived_account.into(), balance)
		.expect("Minting into derived EVM accounf failed.");
}

pub fn deploy_contract<T: Runtime>(address: impl AsRef<[u8]> + Clone, code: Vec<u8>) -> H160 {
	let transaction_create_cost = <T as pallet_evm::Config>::config().gas_transaction_create;
	let (base_fee, _) = <T as pallet_evm::Config>::FeeCalculator::min_gas_price();

	pallet_evm::Pallet::<T>::create(
		T::RuntimeOriginExt::root(),
		H160::from(vec_to_fixed_array(address)),
		code,
		sp_core::U256::from(0),
		transaction_create_cost * 10,
		sp_core::U256::from(base_fee + 10),
		None,
		None,
		Vec::new(),
	)
	.unwrap();

	match frame_system::Pallet::<T>::events()
		.pop()
		.expect("Event is deposited lastly there if create does not fail.")
		.event
		.try_into()
		.map_err(|_| ())
		.expect("Last event is coming from pallet-evm")
	{
		pallet_evm::Event::<T>::Created { address } => H160::from(address.0),
		_ => panic!("Last event is Created event. qed"),
	}
}
