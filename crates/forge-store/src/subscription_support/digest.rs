use crate::failure::StoreError;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) fn stable_digest<T: Serialize>(value: &T) -> Result<String, StoreError> {
    let canonical = serde_json::to_vec(value)?;
    let mut hasher = Sha256::new();
    hasher.update(canonical);
    Ok(format!("{:x}", hasher.finalize()))
}
