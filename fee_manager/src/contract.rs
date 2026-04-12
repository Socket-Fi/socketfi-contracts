use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};

use crate::{
    access::{authenticate_admin, has_admin, write_admin, write_factory},
    contract_trait::RegistryTrait,
    registry::{read_is_smart_wallet, read_passkey_wallet_map, read_username_wallet_map},
};
use socketfi_shared::ContractError;

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryTrait for RegistryContract {
    ///Initialize Contract
    fn __constructor(e: Env, admin: Address, factory: Address) -> Result<(), ContractError> {
        if has_admin(&e) {
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        write_factory(&e, &factory);

        Ok(())
    }

    ///Set a wallet version

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool {
        read_is_smart_wallet(&e, wallet_id)
    }

    fn get_is_smart_wallet_no_return(e: Env, wallet_id: Address) {
        read_is_smart_wallet(&e, wallet_id);
    }

    fn get_wallet_by_username(
        e: Env,
        platform: String,
        username: String,
    ) -> Result<Address, ContractError> {
        let wallet = read_username_wallet_map(&e, platform, username)?;

        Ok(wallet)
    }

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError> {
        read_passkey_wallet_map(&e, passkey)
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
