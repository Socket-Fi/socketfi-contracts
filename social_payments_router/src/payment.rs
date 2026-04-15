use socketfi_access::access::read_registry;
use socketfi_shared::{tokens::send_asset, utils::userid_payment_key, ContractError};
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal, String, Symbol, Val, Vec};

use crate::data::{DataKey, PaymentStatus, PendingPayment};

pub fn now(e: &Env) -> u64 {
    e.ledger().timestamp()
}

/// Deterministic payment id generator.
pub fn generate_payment_id(
    e: &Env,
    sender: Address,
    asset: Address,
    amount: i128,
    platform: String,
    user_id: String,
    nonce: u64,
) -> BytesN<32> {
    let mut data = Bytes::new(e);
    data.append(&String::from_str(e, "social_payment").to_xdr(e));
    data.append(&sender.to_xdr(e));
    data.append(&asset.to_xdr(e));
    data.append(&amount.to_xdr(e));
    data.append(&platform.to_xdr(e));
    data.append(&user_id.to_xdr(e));
    data.append(&nonce.to_xdr(e));

    e.crypto().sha256(&data).into()
}

pub fn write_payment(
    e: &Env,
    payment_id: &BytesN<32>,
    payment: &PendingPayment,
) -> Result<(), ContractError> {
    e.storage()
        .persistent()
        .set(&DataKey::Payment(payment_id.clone()), &payment);
    Ok(())
}

pub fn read_payment(e: &Env, payment_id: &BytesN<32>) -> Result<PendingPayment, ContractError> {
    e.storage()
        .persistent()
        .get(&DataKey::Payment(payment_id.clone()))
        .ok_or(ContractError::PaymentNotFound)
}

pub fn append_identity_payment(
    e: &Env,
    platform: String,
    user_id: String,
    payment_id: BytesN<32>,
) -> Result<(), ContractError> {
    let id_key = userid_payment_key(e, platform, user_id)?;
    let storage_key = DataKey::IdentityPayments(id_key);

    let mut ids = e
        .storage()
        .persistent()
        .get::<_, Vec<BytesN<32>>>(&storage_key)
        .unwrap_or_else(|| Vec::new(e));

    ids.push_back(payment_id);
    e.storage().persistent().set(&storage_key, &ids);

    Ok(())
}

pub fn append_sender_payment(
    e: &Env,
    from: Address,
    payment_id: BytesN<32>,
) -> Result<(), ContractError> {
    let storage_key = DataKey::SenderPayments(from);

    let mut ids = e
        .storage()
        .persistent()
        .get::<_, Vec<BytesN<32>>>(&storage_key)
        .unwrap_or_else(|| Vec::new(e));

    ids.push_back(payment_id);
    e.storage().persistent().set(&storage_key, &ids);

    Ok(())
}

/// Claims a pending payment.
pub fn claim_one(e: &Env, claimer: &Address, payment_id: &BytesN<32>) -> Result<(), ContractError> {
    let mut payment = read_payment(e, payment_id)?;

    if !matches!(payment.status, PaymentStatus::Pending) {
        return Err(ContractError::PaymentNotClaimable);
    }

    if now(e) >= payment.expires_at {
        return Err(ContractError::PaymentExpired);
    }

    let args: Vec<Val> = Vec::from_array(
        e,
        [
            payment.platform.clone().into_val(e),
            payment.user_id.clone().into_val(e),
        ],
    );

    let resolved: Option<Address> = e.invoke_contract(
        &read_registry(e)?,
        &Symbol::new(e, "get_wallet_by_userid"),
        args,
    );

    if resolved != Some(claimer.clone()) {
        return Err(ContractError::UnauthorizedClaim);
    }

    payment.status = PaymentStatus::Claimed;
    payment.claimed_by = Some(claimer.clone());
    write_payment(e, payment_id, &payment)?;

    send_asset(e, claimer, &payment.asset, payment.amount);

    Ok(())
}

/// Refunds an expired pending payment.
pub fn refund_one(e: &Env, sender: &Address, payment_id: &BytesN<32>) -> Result<(), ContractError> {
    let mut payment = read_payment(e, payment_id)?;

    if !matches!(payment.status, PaymentStatus::Pending) {
        return Err(ContractError::PaymentNotRefundable);
    }

    if payment.sender != *sender {
        return Err(ContractError::NotPaymentSender);
    }

    if now(e) < payment.expires_at {
        return Err(ContractError::PaymentNotExpired);
    }

    payment.status = PaymentStatus::Refunded;
    write_payment(e, payment_id, &payment)?;

    send_asset(e, sender, &payment.asset, payment.amount);

    Ok(())
}

pub fn read_identity_payment_ids(
    e: &Env,
    platform: String,
    user_id: String,
) -> Result<Vec<BytesN<32>>, ContractError> {
    let id_key = userid_payment_key(e, platform, user_id)?;
    let storage_key = DataKey::IdentityPayments(id_key);

    Ok(e.storage()
        .persistent()
        .get::<_, Vec<BytesN<32>>>(&storage_key)
        .unwrap_or_else(|| Vec::new(e)))
}

pub fn read_sender_payment_ids(e: &Env, sender: Address) -> Result<Vec<BytesN<32>>, ContractError> {
    let storage_key = DataKey::SenderPayments(sender);

    Ok(e.storage()
        .persistent()
        .get::<_, Vec<BytesN<32>>>(&storage_key)
        .unwrap_or_else(|| Vec::new(e)))
}
