use soroban_sdk::{xdr::ToXdr, Bytes, Env, String};

use crate::error::ContractError;

const MAX_LEN: u32 = 256;

pub fn string_to_bytes(e: &Env, string: String) -> Result<Bytes, ContractError> {
    let string_xdr = string.clone().to_xdr(e);
    let len = string_xdr.len();
    if len > MAX_LEN {
        return Err(ContractError::MaxLengthExceeded);
    }
    for i in 0..len {
        let v = string_xdr.get_unchecked(i);

        if v >= 65 && v <= 90 {
            return Err(ContractError::UpperNotAlloyed);
        }
    }

    Ok(string.clone().to_xdr(e))
}
