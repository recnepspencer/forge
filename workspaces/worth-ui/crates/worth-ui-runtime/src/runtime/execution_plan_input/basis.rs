use crate::runtime::WorthUiRuntimeFrameEpoch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanLoweringBasis {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    staged_node_classification_count: usize,
    staged_reconciliation_receipt_count: usize,
    staged_query_rebind_entry_count: usize,
}

impl WorthUiPlanLoweringBasis {
    pub(crate) fn new(
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        staged_node_classification_count: usize,
        staged_reconciliation_receipt_count: usize,
        staged_query_rebind_entry_count: usize,
    ) -> Self {
        Self {
            active_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            staged_node_classification_count,
            staged_reconciliation_receipt_count,
            staged_query_rebind_entry_count,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn staged_node_classification_count(&self) -> usize {
        self.staged_node_classification_count
    }

    pub fn staged_reconciliation_receipt_count(&self) -> usize {
        self.staged_reconciliation_receipt_count
    }

    pub fn staged_query_rebind_entry_count(&self) -> usize {
        self.staged_query_rebind_entry_count
    }
}
