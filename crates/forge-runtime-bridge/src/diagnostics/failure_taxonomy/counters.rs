use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncFailureCounters {
    localized_failure_count: usize,
    bundle_entry_count: usize,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncFailureCounters {
    pub fn localized() -> Self {
        Self::new(1, 0)
    }

    pub fn bundled(bundle_entry_count: usize) -> Self {
        Self::new(bundle_entry_count, bundle_entry_count)
    }

    fn new(localized_failure_count: usize, bundle_entry_count: usize) -> Self {
        let canonical_basis = format!(
            "bridge-temporal-async-failure-counters|localized={localized_failure_count}|bundle={bundle_entry_count}"
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            localized_failure_count,
            bundle_entry_count,
            digest: Arc::from(format!(
                "bridge-temporal-async-failure-counters:sha256:{digest:x}"
            )),
        }
    }

    pub fn localized_failure_count(&self) -> usize {
        self.localized_failure_count
    }

    pub fn bundle_entry_count(&self) -> usize {
        self.bundle_entry_count
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
