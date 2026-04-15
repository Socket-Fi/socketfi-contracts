use soroban_sdk::{Address, BytesN, Env, String, Vec};

use socketfi_shared::ContractError;

use crate::data::{PaymentResult, PendingPayment};

pub trait SocialPaymentsTrait {
    // ---------------------------------------------------------------------
    // Initialization
    // ---------------------------------------------------------------------

    fn __constructor(e: Env, admin: Address, registry: Address) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Payments
    // ---------------------------------------------------------------------

    fn pay_to_social(
        e: Env,
        sender: Address,
        platform_str: String,
        user_id: String,
        asset: Address,
        amount: i128,
        duration: u64,
    ) -> Result<PaymentResult, ContractError>;

    fn claim_payment(e: Env, claimer: Address, payment_id: BytesN<32>)
        -> Result<(), ContractError>;

    fn claim_payments(
        e: Env,
        claimer: Address,
        payment_ids: Vec<BytesN<32>>,
    ) -> Result<(), ContractError>;

    fn refund_payment(e: Env, sender: Address, payment_id: BytesN<32>)
        -> Result<(), ContractError>;

    fn refund_payments(
        e: Env,
        sender: Address,
        payment_ids: Vec<BytesN<32>>,
    ) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Queries
    // ---------------------------------------------------------------------

    fn get_identity_payments(
        e: Env,
        platform: String,
        user_id: String,
    ) -> Result<Vec<BytesN<32>>, ContractError>;

    fn get_sender_payments(e: Env, sender: Address) -> Result<Vec<BytesN<32>>, ContractError>;

    fn get_claimable_total(
        e: Env,
        platform: String,
        user_id: String,
        asset: Address,
    ) -> Result<i128, ContractError>;

    fn get_payment(e: Env, payment_id: BytesN<32>) -> Result<PendingPayment, ContractError>;

    fn get_nonce(e: Env) -> u64;

    fn get_supported_assets(e: Env) -> Vec<Address>;

    // ---------------------------------------------------------------------
    // Admin
    // ---------------------------------------------------------------------

    fn add_supported_asset(e: Env, asset: Address) -> Result<(), ContractError>;

    fn remove_supported_asset(e: Env, asset: Address) -> Result<(), ContractError>;

    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    // ---------------------------------------------------------------------
    // Upgrade Governance
    // ---------------------------------------------------------------------

    fn propose_upgrade(
        e: Env,
        proposal_type: String,
        new_wasm_hash: BytesN<32>,
    ) -> Result<(), ContractError>;

    fn add_voter(e: Env, voter: Address) -> Result<(), ContractError>;

    fn cast_vote(e: Env, voter: Address, wasm_hash: BytesN<32>) -> Result<(), ContractError>;

    fn cancel_proposal(e: Env) -> Result<(), ContractError>;

    fn apply_upgrade(e: Env) -> Result<BytesN<32>, ContractError>;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}
