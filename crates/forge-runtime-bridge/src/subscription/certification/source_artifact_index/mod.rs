mod artifact_evidence;
mod artifact_record;
mod index_basis;
mod kind_index_basis;

use std::sync::Arc;

use sha2::{Digest, Sha256};

pub use artifact_evidence::{
    BridgeSubscriptionSourceArtifactEvidence, BridgeSubscriptionSourceArtifactKind,
    BridgeSubscriptionSourceArtifactRole, BridgeSubscriptionSourceArtifactScenario,
};
pub use artifact_record::{
    BridgeSubscriptionSourceArtifactInput, BridgeSubscriptionSourceArtifactRecord,
};

use super::BridgeSubscriptionCertificationCounterSnapshot;

use artifact_record::source_artifact_record_ordering;
use index_basis::BridgeSubscriptionSourceArtifactIndexBasis;
use kind_index_basis::BridgeSubscriptionSourceArtifactKindIndexBasis;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSourceArtifactIndex {
    records: Vec<BridgeSubscriptionSourceArtifactRecord>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactIndex {
    pub(crate) fn build(inputs: Vec<BridgeSubscriptionSourceArtifactInput>) -> Self {
        let scanned_input_count = inputs.len();
        let records = materialize_source_artifact_records(inputs);
        let index_basis = BridgeSubscriptionSourceArtifactIndexBasis::from_records(&records);
        let canonical_basis = index_basis.canonical_basis();
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_source_artifact_index(
                records.len(),
                scanned_input_count,
            ),
            records,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-source-artifact-index:sha256:{digest:x}"
            )),
        }
    }

    pub fn records(&self) -> &[BridgeSubscriptionSourceArtifactRecord] {
        &self.records
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub(crate) fn artifact_kind_digest(
        &self,
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
    ) -> Arc<str> {
        BridgeSubscriptionSourceArtifactKindIndexBasis::from_records_for_kind(
            self.records(),
            artifact_kind,
        )
        .digest()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn materialize_source_artifact_records(
    inputs: Vec<BridgeSubscriptionSourceArtifactInput>,
) -> Vec<BridgeSubscriptionSourceArtifactRecord> {
    let mut records: Vec<_> = inputs
        .into_iter()
        .map(BridgeSubscriptionSourceArtifactRecord::from_input)
        .collect();
    records.sort_by(source_artifact_record_ordering);
    records.dedup_by(BridgeSubscriptionSourceArtifactRecord::same_source_artifact_identity);
    records
}
