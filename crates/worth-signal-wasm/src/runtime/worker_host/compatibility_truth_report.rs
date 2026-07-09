use serde::Serialize;

use super::WorkerCommittedTransactionEnvelope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCompatibilityTruthReport {
    pub worker_first_truth_digest: String,
    pub compatibility_mode_truth_digest: String,
    pub committed_truth_matches: bool,
    pub worker_envelope_family: &'static str,
}

impl WorkerCompatibilityTruthReport {
    pub fn compare(
        worker_envelope: &WorkerCommittedTransactionEnvelope,
        compatibility_mode_truth_digest: String,
    ) -> Self {
        Self {
            worker_first_truth_digest: worker_envelope.committed_truth_digest.clone(),
            committed_truth_matches: worker_envelope.committed_truth_digest
                == compatibility_mode_truth_digest,
            compatibility_mode_truth_digest,
            worker_envelope_family: worker_envelope.envelope_family,
        }
    }
}
