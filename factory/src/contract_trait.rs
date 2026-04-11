use soroban_sdk::{Address, BytesN, Env, Vec};

use socketfi_shared::ContractError;

pub trait FactoryTrait {
    fn __constructor(
        e: Env,
        admin: Address,
        registry: Address,
        wasm: BytesN<32>,
    ) -> Result<(), ContractError>;
    fn create_wallet(
        e: Env,
        passkey: BytesN<77>,
        bls_keys: Vec<BytesN<96>>,
    ) -> Result<Address, ContractError>;

    fn get_latest_version(e: Env) -> Result<BytesN<32>, ContractError>;
    fn get_admin(e: Env) -> Result<Address, ContractError>;
    fn get_registry(e: Env) -> Result<Address, ContractError>;
    fn set_latest_wallet(e: Env, wasm: BytesN<32>) -> Result<(), ContractError>;
    fn update_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;
    fn update_registry(e: Env, registry: Address) -> Result<(), ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}
