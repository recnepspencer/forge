use crate::runtime::{
    WorthUiActivationGateReceipt, WorthUiAtomicPlanSwapCounters, WorthUiPriorValidPlanObservation,
    WorthUiRuntimeFrameEpoch,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPlanSwapReceipt {
    previous_active_artifact_digest: u64,
    previous_active_plan_digest: u64,
    previous_active_snapshot_digest: u64,
    next_active_artifact_digest: u64,
    next_active_plan_digest: u64,
    next_active_snapshot_digest: u64,
    activation_gate_receipt: WorthUiActivationGateReceipt,
    prior_valid_plan: WorthUiPriorValidPlanObservation,
    readiness_frame_epoch: WorthUiRuntimeFrameEpoch,
    boundary_frame_epoch: WorthUiRuntimeFrameEpoch,
    reconciliation_basis_digest: u64,
    reconciliation_receipt_count: usize,
    query_rebind_basis_digest: u64,
    query_rebind_entry_count: usize,
    query_rebind_denied_count: usize,
    lane_parity_semantic_reference_digest: Option<u64>,
    counters: WorthUiAtomicPlanSwapCounters,
}

pub(crate) struct WorthUiPlanSwapReceiptParts {
    pub(crate) previous_active_artifact_digest: u64,
    pub(crate) previous_active_plan_digest: u64,
    pub(crate) previous_active_snapshot_digest: u64,
    pub(crate) next_active_artifact_digest: u64,
    pub(crate) next_active_plan_digest: u64,
    pub(crate) next_active_snapshot_digest: u64,
    pub(crate) activation_gate_receipt: WorthUiActivationGateReceipt,
    pub(crate) prior_valid_plan: WorthUiPriorValidPlanObservation,
    pub(crate) counters: WorthUiAtomicPlanSwapCounters,
}

impl WorthUiPlanSwapReceipt {
    pub(crate) fn new(parts: WorthUiPlanSwapReceiptParts) -> Self {
        Self {
            previous_active_artifact_digest: parts.previous_active_artifact_digest,
            previous_active_plan_digest: parts.previous_active_plan_digest,
            previous_active_snapshot_digest: parts.previous_active_snapshot_digest,
            next_active_artifact_digest: parts.next_active_artifact_digest,
            next_active_plan_digest: parts.next_active_plan_digest,
            next_active_snapshot_digest: parts.next_active_snapshot_digest,
            readiness_frame_epoch: parts.activation_gate_receipt.readiness_frame_epoch(),
            boundary_frame_epoch: parts.activation_gate_receipt.boundary_frame_epoch(),
            reconciliation_basis_digest: parts
                .activation_gate_receipt
                .reconciliation_basis_digest(),
            reconciliation_receipt_count: parts
                .activation_gate_receipt
                .reconciliation_receipt_count(),
            query_rebind_basis_digest: parts.activation_gate_receipt.query_rebind_basis_digest(),
            query_rebind_entry_count: parts.activation_gate_receipt.query_rebind_entry_count(),
            query_rebind_denied_count: parts.activation_gate_receipt.query_rebind_denied_count(),
            lane_parity_semantic_reference_digest: parts
                .activation_gate_receipt
                .lane_parity_semantic_reference_digest(),
            activation_gate_receipt: parts.activation_gate_receipt,
            prior_valid_plan: parts.prior_valid_plan,
            counters: parts.counters,
        }
    }

    pub fn previous_active_artifact_digest(self) -> u64 {
        self.previous_active_artifact_digest
    }

    pub fn previous_active_plan_digest(self) -> u64 {
        self.previous_active_plan_digest
    }

    pub fn previous_active_snapshot_digest(self) -> u64 {
        self.previous_active_snapshot_digest
    }

    pub fn next_active_artifact_digest(self) -> u64 {
        self.next_active_artifact_digest
    }

    pub fn next_active_plan_digest(self) -> u64 {
        self.next_active_plan_digest
    }

    pub fn next_active_snapshot_digest(self) -> u64 {
        self.next_active_snapshot_digest
    }

    pub fn activation_gate_receipt(self) -> WorthUiActivationGateReceipt {
        self.activation_gate_receipt
    }

    pub fn prior_valid_plan(self) -> WorthUiPriorValidPlanObservation {
        self.prior_valid_plan
    }

    pub fn readiness_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.readiness_frame_epoch
    }

    pub fn boundary_frame_epoch(self) -> WorthUiRuntimeFrameEpoch {
        self.boundary_frame_epoch
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

    pub fn counters(self) -> WorthUiAtomicPlanSwapCounters {
        self.counters
    }
}
