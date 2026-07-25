use crate::runtime::WorthUiDurableStateReconciliationOutcome;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiDurableStateReconciliationCounters {
    reconciled_family_count: usize,
    reconciled_node_count: usize,
    receipt_count: usize,
    carry_forward_count: usize,
    replacement_count: usize,
    drop_count: usize,
    recreate_count: usize,
    orphan_removal_count: usize,
    incompatible_shape_count: usize,
    query_posture_required_count: usize,
    rejected_reconciliation_count: usize,
    initial_artifact_node_visit_count: usize,
    initialized_resize_input_count: usize,
}

impl WorthUiDurableStateReconciliationCounters {
    pub(crate) fn record_family(&mut self) {
        self.reconciled_family_count += 1;
    }

    pub(crate) fn record_node(&mut self) {
        self.reconciled_node_count += 1;
    }

    pub(crate) fn record_receipt(&mut self, outcome: WorthUiDurableStateReconciliationOutcome) {
        self.receipt_count += 1;
        match outcome {
            WorthUiDurableStateReconciliationOutcome::CarryForward => self.carry_forward_count += 1,
            WorthUiDurableStateReconciliationOutcome::Replace => self.replacement_count += 1,
            WorthUiDurableStateReconciliationOutcome::Drop => self.drop_count += 1,
            WorthUiDurableStateReconciliationOutcome::Recreate => self.recreate_count += 1,
        }
    }

    pub(crate) fn record_orphan_removal(&mut self) {
        self.orphan_removal_count += 1;
    }

    pub(crate) fn record_incompatible_shape(&mut self) {
        self.incompatible_shape_count += 1;
    }

    pub(crate) fn record_query_posture_required(&mut self) {
        self.query_posture_required_count += 1;
    }

    pub(crate) fn record_rejected_reconciliation(&mut self) {
        self.rejected_reconciliation_count += 1;
    }

    pub(crate) fn record_initial_artifact_nodes(&mut self, count: usize) {
        self.initial_artifact_node_visit_count += count;
    }

    pub(crate) fn record_initialized_resize_input(&mut self) {
        self.initialized_resize_input_count += 1;
    }

    pub fn reconciled_family_count(&self) -> usize {
        self.reconciled_family_count
    }

    pub fn reconciled_node_count(&self) -> usize {
        self.reconciled_node_count
    }

    pub fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub fn carry_forward_count(&self) -> usize {
        self.carry_forward_count
    }

    pub fn replacement_count(&self) -> usize {
        self.replacement_count
    }

    pub fn drop_count(&self) -> usize {
        self.drop_count
    }

    pub fn recreate_count(&self) -> usize {
        self.recreate_count
    }

    pub fn orphan_removal_count(&self) -> usize {
        self.orphan_removal_count
    }

    pub fn incompatible_shape_count(&self) -> usize {
        self.incompatible_shape_count
    }

    pub fn query_posture_required_count(&self) -> usize {
        self.query_posture_required_count
    }

    pub fn rejected_reconciliation_count(&self) -> usize {
        self.rejected_reconciliation_count
    }

    pub fn initial_artifact_node_visit_count(&self) -> usize {
        self.initial_artifact_node_visit_count
    }

    pub fn initialized_resize_input_count(&self) -> usize {
        self.initialized_resize_input_count
    }
}
