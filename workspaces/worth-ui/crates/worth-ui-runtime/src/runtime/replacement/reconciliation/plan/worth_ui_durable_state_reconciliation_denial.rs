use crate::runtime::WorthUiDurableStateReconciliationCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDurableStateReconciliationDenial {
    AmbiguousNodeReplacementPlan {
        counters: Box<WorthUiDurableStateReconciliationCounters>,
    },
    InventoryDigestMismatch {
        plan_active_artifact_digest: u64,
        inventory_active_artifact_digest: u64,
        plan_candidate_artifact_digest: u64,
        inventory_candidate_artifact_digest: u64,
        counters: Box<WorthUiDurableStateReconciliationCounters>,
    },
}
