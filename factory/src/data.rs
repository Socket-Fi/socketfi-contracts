use soroban_sdk::{contracttype, BytesN};

#[derive(Clone)]
#[contracttype]
pub struct BlsKeyWithProof {
    pub key: BytesN<96>,
    pub sig: BytesN<192>,
}
