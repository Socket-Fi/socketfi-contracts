use soroban_sdk::{Address, BytesN, Env, String, Vec};

use socketfi_shared::ContractError;

pub trait RegistryTrait {
    fn __constructor(e: Env, admin: Address, factory: Address) -> Result<(), ContractError>;

    fn get_is_smart_wallet(e: Env, wallet_id: Address) -> bool;
    fn get_is_smart_wallet_no_return(e: Env, wallet_id: Address);

    fn get_wallet_by_username(
        e: Env,
        platform: String,
        username: String,
    ) -> Result<Address, ContractError>;

    fn get_wallet_by_passkey(e: Env, passkey: BytesN<77>) -> Result<Address, ContractError>;

    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}
