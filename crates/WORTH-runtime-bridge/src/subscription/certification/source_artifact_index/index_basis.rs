use std::sync::Arc;

use super::artifact_record::BridgeSubscriptionSourceArtifactRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BridgeSubscriptionSourceArtifactIndexBasis {
    record_evidence: Arc<[BridgeSubscriptionSourceArtifactIndexRecordEvidence]>,
}

impl BridgeSubscriptionSourceArtifactIndexBasis {
    pub(super) fn from_records(records: &[BridgeSubscriptionSourceArtifactRecord]) -> Self {
        let record_evidence = records
            .iter()
            .map(BridgeSubscriptionSourceArtifactIndexRecordEvidence::from_record)
            .collect::<Vec<_>>();
        Self {
            record_evidence: Arc::from(record_evidence),
        }
    }

    pub(super) fn canonical_basis(&self) -> Arc<str> {
        Arc::from(format!(
            "bridge-subscription-source-artifact-index|records={}",
            self.record_digest_evidence_basis()
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
struct BridgeSubscriptionSourceArtifactIndexRecordEvidence {
    digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactIndexRecordEvidence {
    fn from_record(record: &BridgeSubscriptionSourceArtifactRecord) -> Self {
        Self {
            digest: Arc::from(record.digest()),
        }
    }

    fn as_str(&self) -> &str {
        self.digest.as_ref()
    }
}
