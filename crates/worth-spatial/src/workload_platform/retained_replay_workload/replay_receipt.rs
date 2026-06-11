use crate::workload_platform::vocabulary::{RetainedReplayWorkloadReceipt, WorkloadStageIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayWorkloadCounters {
    retained_artifact_rows: usize,
    replay_evidence_rows: usize,
    replay_rows: usize,
    projection_consumed_rows: usize,
}

impl ReplayWorkloadCounters {
    pub(crate) fn new(
        retained_artifact_rows: usize,
        replay_evidence_rows: usize,
        replay_rows: usize,
        projection_consumed_rows: usize,
    ) -> Self {
        Self {
            retained_artifact_rows,
            replay_evidence_rows,
            replay_rows,
            projection_consumed_rows,
        }
    }

    pub fn retained_artifact_rows(self) -> usize {
        self.retained_artifact_rows
    }

    pub fn replay_evidence_rows(self) -> usize {
        self.replay_evidence_rows
    }

    pub fn replay_rows(self) -> usize {
        self.replay_rows
    }

    pub fn projection_consumed_rows(self) -> usize {
        self.projection_consumed_rows
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayReceiptSet {
    stage_receipt: RetainedReplayWorkloadReceipt,
    transformed_workload_identity: String,
    retained_artifact_identity: String,
    retained_artifact_capture_identity: String,
    retained_basis_identity: String,
    replay_checkpoint_identity: String,
    replay_evidence_identity: String,
    counters: ReplayWorkloadCounters,
}

impl ReplayReceiptSet {
    pub(crate) fn new(
        stage_receipt: RetainedReplayWorkloadReceipt,
        transformed_workload_identity: impl Into<String>,
        retained_artifact_identity: impl Into<String>,
        retained_artifact_capture_identity: impl Into<String>,
        retained_basis_identity: impl Into<String>,
        replay_checkpoint_identity: impl Into<String>,
        replay_evidence_identity: impl Into<String>,
        counters: ReplayWorkloadCounters,
    ) -> Self {
        Self {
            stage_receipt,
            transformed_workload_identity: transformed_workload_identity.into(),
            retained_artifact_identity: retained_artifact_identity.into(),
            retained_artifact_capture_identity: retained_artifact_capture_identity.into(),
            retained_basis_identity: retained_basis_identity.into(),
            replay_checkpoint_identity: replay_checkpoint_identity.into(),
            replay_evidence_identity: replay_evidence_identity.into(),
            counters,
        }
    }

    pub fn stage_identity(&self) -> &WorkloadStageIdentity {
        self.stage_receipt.identity()
    }

    pub fn stage_receipt(&self) -> &RetainedReplayWorkloadReceipt {
        &self.stage_receipt
    }

    pub fn transformed_workload_identity(&self) -> &str {
        &self.transformed_workload_identity
    }

    pub fn retained_artifact_identity(&self) -> &str {
        &self.retained_artifact_identity
    }

    pub fn retained_artifact_capture_identity(&self) -> &str {
        &self.retained_artifact_capture_identity
    }

    pub fn retained_basis_identity(&self) -> &str {
        &self.retained_basis_identity
    }

    pub fn replay_checkpoint_identity(&self) -> &str {
        &self.replay_checkpoint_identity
    }

    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }

    pub fn counters(&self) -> ReplayWorkloadCounters {
        self.counters
    }
}
