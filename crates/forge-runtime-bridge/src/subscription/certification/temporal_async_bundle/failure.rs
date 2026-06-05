use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::diagnostics::BridgeTemporalAsyncOfflineDiagnosisBundleSealed;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationFailureSection {
    bridge_owner: Arc<str>,
    semantic_failure_rows: Arc<[Arc<str>]>,
    localized_failure_count: usize,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationFailureSection {
    pub(crate) fn collect(
        failure_bundle: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
    ) -> Self {
        let semantic_failure_rows = failure_bundle
            .localized_failures()
            .iter()
            .map(|failure| {
                Arc::from(format!(
                    "{}:{}",
                    failure.failure_class().as_str(),
                    failure.subcode().as_str()
                ))
            })
            .collect::<Vec<_>>();
        let semantic_basis = format!(
            "bridge-temporal-async-certification-failure-section|rows={}",
            semantic_failure_rows
                .iter()
                .map(|row: &Arc<str>| row.as_ref())
                .collect::<Vec<_>>()
                .join(","),
        );
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        let digest = Sha256::digest(
            format!(
                "{semantic_basis}|failure-bundle={}|bridge-owner=forge-runtime-bridge",
                failure_bundle.digest(),
            )
            .as_bytes(),
        );
        Self {
            bridge_owner: Arc::from("forge-runtime-bridge"),
            localized_failure_count: semantic_failure_rows.len(),
            semantic_failure_rows: semantic_failure_rows.into(),
            semantic_digest: Arc::from(format!(
                "bridge-temporal-async-certification-failure-section-semantic:sha256:{semantic_digest:x}"
            )),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-failure-section:sha256:{digest:x}"
            )),
        }
    }

    pub fn bridge_owner(&self) -> &str {
        self.bridge_owner.as_ref()
    }

    pub fn localized_failure_count(&self) -> usize {
        self.localized_failure_count
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
