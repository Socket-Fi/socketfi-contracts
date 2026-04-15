use soroban_sdk::Env;

use crate::data::DataKey;

/// Reads current payment nonce.
/// Returns 0 if not initialized.
pub fn read_payment_nonce(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::PaymentNonce)
        .unwrap_or(0)
}

/// Writes updated nonce.
pub fn write_payment_nonce(e: &Env, nonce: u64) {
    e.storage().instance().set(&DataKey::PaymentNonce, &nonce);
}
