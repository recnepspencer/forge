use serde::Serialize;

use crate::runtime::summaries::RunSummary;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerCommittedTransactionEnvelope {
    pub envelope_family: &'static str,
    pub deployment_posture: &'static str,
    pub runtime_authority: &'static str,
    pub branch_id: u64,
    pub committed_truth_digest: String,
    pub run_summary: RunSummary,
}

impl WorkerCommittedTransactionEnvelope {
    pub(in crate::runtime::worker_host) fn from_committed_worker_transaction(
        branch_id: u64,
        committed_truth_digest: String,
        run_summary: RunSummary,
    ) -> Self {
        Self {
            envelope_family: "transactionResult",
            deployment_posture: "workerFirst",
            runtime_authority: "workerOwnedRuntime",
            branch_id,
            committed_truth_digest,
            run_summary,
        }
    }
}
