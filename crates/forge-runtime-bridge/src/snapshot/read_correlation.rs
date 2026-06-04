use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SnapshotReadCorrelationId {
    value: Arc<str>,
}

impl SnapshotReadCorrelationId {
    pub(super) fn from_native_request_basis(canonical_basis: &str) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            value: Arc::from(format!("snapshot-read-correlation:sha256:{digest:x}")),
        }
    }

    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }
}
