use soroban_sdk::{xdr::ToXdr, Bytes, Env, String, IntoVal, Val};

use crate::{constants::{MAX_LEN, DAY_IN_LEDGERS}, ContractError};

pub fn string_to_bytes(e: &Env, string: String) -> Result<Bytes, ContractError> {
    let string_xdr = string.to_xdr(e);
    let len = string_xdr.len();

    if len > MAX_LEN {
        return Err(ContractError::MaxLengthExceeded);
    }

    for i in 0..len {
        let v = string_xdr.get_unchecked(i);

        if v >= 65 && v <= 90 {
            return Err(ContractError::UpperNotAllowed);
        }
    }

    Ok(string_xdr)
}

pub fn check_lower(e: &Env, string: String) -> Result<(), ContractError> {
    let string_xdr = string.clone().to_xdr(e);
    let len = string_xdr.len();
    if len > MAX_LEN {
        return Err(ContractError::MaxLengthExceeded);
    }
    for i in 0..len {
        let v = string_xdr.get_unchecked(i);

        if v >= 65 && v <= 90 {
            return Err(ContractError::UpperNotAllowed);
        }
    }

    Ok(())
}

pub fn bump_instance(e: &Env) {
    let max_ttl = e.storage().max_ttl();
    e.storage()
        .instance()
        .extend_ttl(max_ttl - DAY_IN_LEDGERS, max_ttl);
}

pub fn bump_persistent<K>(e: &Env, key: &K)
where
    K: IntoVal<Env, Val>,
{
    let max_ttl = e.storage().max_ttl();
    e.storage()
        .persistent()
        .extend_ttl(key, max_ttl - DAY_IN_LEDGERS, max_ttl);
}