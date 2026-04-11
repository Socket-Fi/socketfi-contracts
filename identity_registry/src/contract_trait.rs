use soroban_sdk::{Address, BytesN, Env, String, Vec};

use crate::error::ContractError;

pub trait MasterContractTrait {
    fn __constructor(e: Env, admin: Address, wasm: BytesN<32>) -> Result<(), ContractError>;
    fn create_wallet(
        e: Env,
        username: String,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError>;

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool;
    fn get_is_smart_wallet_no_return(e: Env, wallet_id: Address);

    fn get_wallet_by_username(e: Env, username: String) -> Result<Address, ContractError>;

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError>;

    fn get_latest_version(e: Env) -> Result<BytesN<32>, ContractError>;
    fn set_latest_wallet_v(e: Env, wasm: BytesN<32>) -> Result<(), ContractError>;
    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}
