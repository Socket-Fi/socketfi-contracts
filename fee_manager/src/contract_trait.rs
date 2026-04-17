use soroban_sdk::{Address, BytesN, Env, Vec};

use crate::errors::ContractError;
use socketfi_shared::fee_types::FeeDecision;

pub trait FeeManagerTrait {
    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------
    fn __constructor(
        e: Env,
        admin: Address,
        base_fee: i128,
        max_deferred_fee: i128,
    ) -> Result<(), ContractError>;

    // -------------------------------------------------------------------------
    // Admin Management
    // -------------------------------------------------------------------------
    fn get_admin(e: Env) -> Option<Address>;
    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    // -------------------------------------------------------------------------
    // Base Fee Configuration
    // -------------------------------------------------------------------------
    fn set_base_fee(e: Env, fee: i128) -> Result<(), ContractError>;
    fn get_base_fee(e: Env) -> Option<i128>;

    fn set_max_deferred_fee(e: Env, fee: i128) -> Result<(), ContractError>;
    fn get_max_deferred_fee(e: Env) -> Result<i128, ContractError>;

    // -------------------------------------------------------------------------
    // Supported Fee Assets
    // -------------------------------------------------------------------------
    fn add_supported_fee_asset(e: Env, asset: Address, rate: i128) -> Result<(), ContractError>;
    fn remove_supported_fee_asset(e: Env, asset: Address) -> Result<(), ContractError>;

    fn set_fee_asset_rate(e: Env, asset: Address, rate: i128) -> Result<(), ContractError>;

    fn is_supported_fee_asset(e: Env, asset: Address) -> bool;
    fn get_supported_fee_assets(e: Env) -> Vec<Address>;
    fn get_fee_asset_rate(e: Env, asset: Address) -> Result<i128, ContractError>;

    // -------------------------------------------------------------------------
    // Deferred Fee State
    // -------------------------------------------------------------------------
    fn get_deferred_fee(e: Env, user: Address) -> Result<i128, ContractError>;

    // -------------------------------------------------------------------------
    // Fee Logic (Core)
    // -------------------------------------------------------------------------
    // Determines whether to collect fee immediately or defer
    fn quote_transaction_fee(
        e: Env,
        user: Address,
        tx_asset: Address,
        tx_amount: i128,
    ) -> FeeDecision;

    // Applies the result of quote_transaction_fee
    fn apply_transaction_fee(e: Env, wallet: Address, decision: FeeDecision);

    // -------------------------------------------------------------------------
    // Contract Upgrade
    // -------------------------------------------------------------------------
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}
