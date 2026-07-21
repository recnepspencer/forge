use crate::runtime::WorthUiRuntimeFrameEpoch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPlanLoweringBasis {
    prior_artifact_digest: Option<u64>,
    candidate_artifact_digest: u64,
    frame_epoch: WorthUiRuntimeFrameEpoch,
    candidate_node_input_count: usize,
    reconciliation_receipt_count: usize,
    query_binding_input_count: usize,
}

impl WorthUiPlanLoweringBasis {
    pub(crate) fn new(
        prior_artifact_digest: Option<u64>,
        candidate_artifact_digest: u64,
        frame_epoch: WorthUiRuntimeFrameEpoch,
        candidate_node_input_count: usize,
        reconciliation_receipt_count: usize,
        query_binding_input_count: usize,
    ) -> Self {
        Self {
            prior_artifact_digest,
            candidate_artifact_digest,
            frame_epoch,
            candidate_node_input_count,
            reconciliation_receipt_count,
            query_binding_input_count,
        }
    }

    pub fn prior_artifact_digest(&self) -> Option<u64> {
        self.prior_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn frame_epoch(&self) -> WorthUiRuntimeFrameEpoch {
        self.frame_epoch
    }

    pub fn candidate_node_input_count(&self) -> usize {
        self.candidate_node_input_count
    }

    pub fn reconciliation_receipt_count(&self) -> usize {
        self.reconciliation_receipt_count
    }

    pub fn query_binding_input_count(&self) -> usize {
        self.query_binding_input_count
    }
}
