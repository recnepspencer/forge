use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::artifact_evidence::BridgeSubscriptionSourceArtifactKind;
use super::artifact_record::BridgeSubscriptionSourceArtifactRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BridgeSubscriptionSourceArtifactKindIndexBasis {
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    record_evidence: Arc<[BridgeSubscriptionSourceArtifactKindRecordEvidence]>,
}

impl BridgeSubscriptionSourceArtifactKindIndexBasis {
    pub(super) fn from_records_for_kind(
        records: &[BridgeSubscriptionSourceArtifactRecord],
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
    ) -> Self {
        let record_evidence = records
            .iter()
            .filter(|record| record.artifact_kind() == artifact_kind)
            .map(BridgeSubscriptionSourceArtifactKindRecordEvidence::from_record)
            .collect::<Vec<_>>();
        Self {
            artifact_kind,
            record_evidence: Arc::from(record_evidence),
        }
    }

    pub(super) fn digest(&self) -> Arc<str> {
        let canonical_basis = self.canonical_basis();
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Arc::from(format!(
            "bridge-subscription-source-artifact-kind-index:sha256:{digest:x}"
        ))
    }

    fn canonical_basis(&self) -> Arc<str> {
        Arc::from(format!(
            "bridge-subscription-source-artifact-kind-index|kind={}|records={}",
            self.artifact_kind.as_str(),
            self.record_digest_evidence_basis(),
        ))
    }

    fn record_digest_evidence_basis(&self) -> String {
        let mut basis = String::new();
        for evidence in self.record_evidence.iter() {
            if !basis.is_empty() {
                basis.push(',');
            }
            basis.push_str(evidence.as_str());
        }
        basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeSubscriptionSourceArtifactKindRecordEvidence {
    digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactKindRecordEvidence {
    fn from_record(record: &BridgeSubscriptionSourceArtifactRecord) -> Self {
        Self {
            digest: Arc::from(record.digest()),
        }
    }

    fn as_str(&self) -> &str {
        self.digest.as_ref()
    }
}
