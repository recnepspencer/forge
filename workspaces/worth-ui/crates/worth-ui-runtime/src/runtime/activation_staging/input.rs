use crate::runtime::{
    WorthUiDurableStateReconciliationPlan, WorthUiNodeReplacementPlan, WorthUiQueryLiveRebindPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPendingExecutionPlanLoweringInput {
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    node_classification_count: usize,
    reconciliation_receipt_count: usize,
    query_rebind_entry_count: usize,
}

impl WorthUiPendingExecutionPlanLoweringInput {
    pub(crate) fn from_staged_plans(
        node_plan: &WorthUiNodeReplacementPlan,
        reconciliation_plan: &WorthUiDurableStateReconciliationPlan,
        query_rebind_plan: &WorthUiQueryLiveRebindPlan,
    ) -> Self {
        Self {
            active_artifact_digest: node_plan.active_artifact_digest(),
            candidate_artifact_digest: node_plan.candidate_artifact_digest(),
            node_classification_count: node_plan.classifications().len(),
            reconciliation_receipt_count: reconciliation_plan.receipts().len(),
            query_rebind_entry_count: query_rebind_plan.entries().len(),
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn node_classification_count(&self) -> usize {
        self.node_classification_count
    }

    pub fn reconciliation_receipt_count(&self) -> usize {
        self.reconciliation_receipt_count
    }

    pub fn query_rebind_entry_count(&self) -> usize {
        self.query_rebind_entry_count
    }
}
