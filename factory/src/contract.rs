use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Vec};

use crate::{
    access::{
        authenticate_admin, has_admin, read_admin, read_registry, write_admin, write_registry,
    },
    contract_trait::FactoryTrait,
    wallet_factory::{read_latest_version, write_create_wallet, write_latest_version},
};
use socketfi_shared::ContractError;

#[contract]
pub struct FactoryContract;

#[contractimpl]
impl FactoryTrait for FactoryContract {
    ///Initialize Contract and set wallet wasm
    fn __constructor(
        e: Env,
        admin: Address,
        registry: Address,
        wasm: BytesN<32>,
    ) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        write_registry(&e, &registry);
        write_latest_version(&e, wasm);

        Ok(())
    }

    fn create_wallet(
        e: Env,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError> {
        let wallet_address = write_create_wallet(&e, &passkey.clone(), bls_keys)?;
        Ok(wallet_address)
    }

    fn get_latest_version(e: Env) -> Result<BytesN<32>, ContractError> {
        let version = read_latest_version(&e).ok_or(ContractError::VersionNotFound)?;

        Ok(version)
    }
    fn get_admin(e: Env) -> Result<Address, ContractError> {
        let admin = read_admin(&e)?;

        Ok(admin)
    }
    fn get_registry(e: Env) -> Result<Address, ContractError> {
        let registry = read_registry(&e)?;

        Ok(registry)
    }

    fn set_latest_wallet(e: Env, wasm: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_latest_version(&e, wasm);
        Ok(())
    }
    fn update_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_admin(&e, &new_admin);
        Ok(())
    }
    fn update_registry(e: Env, registry: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_registry(&e, &registry);
        Ok(())
    }
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
