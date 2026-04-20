use crate::failure::StoreError;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::model::{DurableMutationId, WalRecordFamily, WalRecordPayload};

#[derive(Serialize)]
pub(super) struct WalRecordDigestBasis<'a> {
    pub family: WalRecordFamily,
    pub durable_mutation_id: DurableMutationId,
    pub runtime_session_id: &'a str,
    pub wal_version: u32,
    pub payload: &'a WalRecordPayload,
}

pub(super) fn stable_digest<T: Serialize>(value: &T) -> Result<String, StoreError> {
    let bytes = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}
