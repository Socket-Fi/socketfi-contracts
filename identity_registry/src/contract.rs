use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

use crate::{
    access::{authenticate_admin, has_admin, write_admin},
    contract_trait::MasterContractTrait,
    error::ContractError,
    platforms::{
        read_is_smart_wallet, read_passkey_wallet_map, read_username_wallet_map,
        write_is_smart_wallet, write_passkey_wallet_map, write_username_wallet_map,
    },
    wallet_factory::{read_latest_version, write_create_wallet, write_latest_version},
};

#[contract]
pub struct MasterContract;

#[contractimpl]
impl MasterContractTrait for MasterContract {
    ///Initialize Contract
    fn __constructor(e: Env, admin: Address, wasm: BytesN<32>) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        write_latest_version(&e, wasm);

        Ok(())
    }

    fn create_wallet(
        e: Env,
        username: String,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError> {
        let wasm = read_latest_version(&e).ok_or(ContractError::VersionNotFound)?;
        let wallet_address = write_create_wallet(
            &e,
            username.clone(),
            &passkey.clone(),
            bls_keys,
            wasm.clone(),
        )?;

        write_username_wallet_map(&e, username, wallet_address.clone())?;
        write_passkey_wallet_map(&e, passkey, wallet_address.clone())?;
        write_is_smart_wallet(&e, wallet_address.clone());

        Ok(wallet_address)
    }

    ///Set a wallet version

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool {
        read_is_smart_wallet(&e, wallet_id)
    }

    fn get_is_smart_wallet_no_return(e: Env, wallet_id: Address) {
        read_is_smart_wallet(&e, wallet_id);
    }

    fn get_wallet_by_username(e: Env, username: String) -> Result<Address, ContractError> {
        let wallet = read_username_wallet_map(&e, username)?;

        Ok(wallet)
    }

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
        read_passkey_wallet_map(&e, passkey)
    }
    fn get_latest_version(e: Env) -> Result<BytesN<32>, ContractError> {
        let version = read_latest_version(&e).ok_or(ContractError::VersionNotFound)?;

        Ok(version)
    }

    fn set_latest_wallet_v(e: Env, wasm: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_latest_version(&e, wasm);
        Ok(())
    }
    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        write_admin(&e, &new_admin);
        Ok(())
    }
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        authenticate_admin(&e)?;
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}
