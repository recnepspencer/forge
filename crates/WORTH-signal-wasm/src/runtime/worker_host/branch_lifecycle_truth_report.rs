use serde::Serialize;

use super::WorkerBranchTruthEnvelope;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBranchLifecycleTruthReport {
    pub worker_first_truth_digest: String,
    pub compatibility_mode_truth_digest: String,
    pub branch_truth_matches: bool,
    pub worker_envelope_family: &'static str,
}

impl WorkerBranchLifecycleTruthReport {
    pub fn compare(
        worker_branch: &WorkerBranchTruthEnvelope,
        compatibility_mode_truth_digest: String,
    ) -> Self {
        Self {
            worker_first_truth_digest: worker_branch.committed_truth_digest.clone(),
            branch_truth_matches: worker_branch.committed_truth_digest
                == compatibility_mode_truth_digest,
            compatibility_mode_truth_digest,
            worker_envelope_family: worker_branch.envelope_family,
        }
    }
}
