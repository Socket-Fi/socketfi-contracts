use soroban_sdk::{xdr::ToXdr, Bytes, Env, String};

use crate::{constants::MAX_LEN, ContractError};

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