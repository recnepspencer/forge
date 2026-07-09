use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBranchTruthEnvelope {
    pub envelope_family: &'static str,
    pub lifecycle_artifact: &'static str,
    pub deployment_posture: &'static str,
    pub runtime_authority: &'static str,
    pub branch_id: u64,
    pub branch_name: String,
    pub snapshot_id: Option<u64>,
    pub committed_truth_digest: String,
}

impl WorkerBranchTruthEnvelope {
    pub(in crate::runtime::worker_host) fn from_worker_branch(
        branch_id: u64,
        branch_name: String,
        snapshot_id: Option<u64>,
        committed_truth_digest: String,
    ) -> Self {
        Self {
            envelope_family: "lifecycleControl",
            lifecycle_artifact: "branchTruth",
            deployment_posture: "workerFirst",
            runtime_authority: "workerOwnedRuntime",
            branch_id,
            branch_name,
            snapshot_id,
            committed_truth_digest,
        }
    }
}
