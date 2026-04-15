use soroban_sdk::{contracttype, Address, BytesN, String};

/// Result of a payment attempt.
///
/// Notes:
/// - `Direct` means funds were immediately transferred to a resolved wallet.
/// - `Pending` means funds are escrowed and require a later claim.
#[derive(Clone)]
#[contracttype]
pub enum PaymentResult {
    Direct(Address),
    Pending(BytesN<32>),
}

/// Represents a stored pending payment.
#[derive(Clone)]
#[contracttype]
pub struct PendingPayment {
    pub payment_id: BytesN<32>,
    pub sender: Address,
    pub asset: Address,
    pub amount: i128,
    pub platform: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub status: PaymentStatus,
    pub claimed_by: Option<Address>,
}

/// Lifecycle state of a payment.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentStatus {
    Pending,
    Claimed,
    Refunded,
}

/// Storage keys for contract state.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    /// Mapping: identity_hash -> Vec<payment_id>
    IdentityPayments(BytesN<32>),
    /// Mapping: sender -> Vec<payment_id>
    SenderPayments(Address),
    /// Validator set: Map<BytesN<32>, ()>
    Validators,
    /// Global payment nonce (monotonic counter)
    PaymentNonce,
    /// Mapping: payment_id -> PendingPayment
    Payment(BytesN<32>),
    /// Supported asset set
    SupportedTokens,
}
