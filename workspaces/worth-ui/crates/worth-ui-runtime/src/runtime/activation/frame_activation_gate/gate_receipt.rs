use crate::runtime::{WorthUiActivationGateCounters, WorthUiRuntimeFrameEpoch};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiActivationGateReceipt {
    active_artifact_digest: u64,
    active_plan_digest: u64,
    active_snapshot_digest: u64,
    candidate_artifact_digest: u64,
    candidate_execution_plan_digest: u64,
    handle_allocation_basis_digest: u64,
    node_classification_count: usize,
    lane_changed_node_count: usize,
    reconciliation_basis_digest: u64,
    reconciliation_receipt_count: usize,
    query_rebind_basis_digest: u64,
    query_rebind_entry_count: usize,
    query_rebind_denied_count: usize,
    lane_parity_semantic_reference_digest: Option<u64>,
    readiness_frame_epoch: WorthUiRuntimeFrameEpoch,
    boundary_frame_epoch: WorthUiRuntimeFrameEpoch,
    counters: WorthUiActivationGateCounters,
}

pub(crate) struct WorthUiActivationGateReceiptParts {
    pub(crate) active_artifact_digest: u64,
    pub(crate) active_plan_digest: u64,
    pub(crate) active_snapshot_digest: u64,
    pub(crate) candidate_artifact_digest: u64,
    pub(crate) candidate_execution_plan_digest: u64,
    pub(crate) handle_allocation_basis_digest: u64,
    pub(crate) node_classification_count: usize,
    pub(crate) lane_changed_node_count: usize,
    pub(crate) reconciliation_basis_digest: u64,
    pub(crate) reconciliation_receipt_count: usize,
    pub(crate) query_rebind_basis_digest: u64,
    pub(crate) query_rebind_entry_count: usize,
    pub(crate) query_rebind_denied_count: usize,
    pub(crate) lane_parity_semantic_reference_digest: Option<u64>,
    pub(crate) readiness_frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) boundary_frame_epoch: WorthUiRuntimeFrameEpoch,
    pub(crate) counters: WorthUiActivationGateCounters,
}

impl WorthUiActivationGateReceipt {
    pub(crate) fn new(parts: WorthUiActivationGateReceiptParts) -> Self {
        Self {
            active_artifact_digest: parts.active_artifact_digest,
            active_plan_digest: parts.active_plan_digest,
            active_snapshot_digest: parts.active_snapshot_digest,
            candidate_artifact_digest: parts.candidate_artifact_digest,
            candidate_execution_plan_digest: parts.candidate_execution_plan_digest,
            handle_allocation_basis_digest: parts.handle_allocation_basis_digest,
            node_classification_count: parts.node_classification_count,
            lane_changed_node_count: parts.lane_changed_node_count,
            reconciliation_basis_digest: parts.reconciliation_basis_digest,
            reconciliation_receipt_count: parts.reconciliation_receipt_count,
            query_rebind_basis_digest: parts.query_rebind_basis_digest,
            query_rebind_entry_count: parts.query_rebind_entry_count,
            query_rebind_denied_count: parts.query_rebind_denied_count,
            lane_parity_semantic_reference_digest: parts.lane_parity_semantic_reference_digest,
            readiness_frame_epoch: parts.readiness_frame_epoch,
            boundary_frame_epoch: parts.boundary_frame_epoch,
            counters: parts.counters,
        }
    }

    pub fn active_artifact_digest(self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(self) -> u64 {
        self.active_plan_digest
    }

    pub fn active_snapshot_digest(self) -> u64 {
        self.active_snapshot_digest
    }

    pub fn candidate_artifact_digest(self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn candidate_execution_plan_digest(self) -> u64 {
        self.candidate_execution_plan_digest
    }

    pub fn handle_allocation_basis_digest(self) -> u64 {
        self.handle_allocation_basis_digest
    }

    pub fn node_classification_count(self) -> usize {
        self.node_classification_count
    }

    pub fn lane_changed_node_count(self) -> usize {
        self.lane_changed_node_count
    }

    pub fn reconciliation_basis_digest(self) -> u64 {
        self.reconciliation_basis_digest
    }

    pub fn reconciliation_receipt_count(self) -> usize {
        self.reconciliation_receipt_count
    }

    pub fn query_rebind_basis_digest(self) -> u64 {
        self.query_rebind_basis_digest
    }

    pub fn query_rebind_entry_count(self) -> usize {
        self.query_rebind_entry_count
    }

    pub fn query_rebind_denied_count(self) -> usize {
        self.query_rebind_denied_count
    }

    pub fn lane_parity_semantic_reference_digest(self) -> Option<u64> {
        self.lane_parity_semantic_reference_digest
    }

    pub fn readiness_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.readiness_frame_epoch
    }

    pub fn boundary_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.boundary_frame_epoch
    }

    pub fn counters(self) -> WorthUiActivationGateCounters {
        self.counters
    }
}
