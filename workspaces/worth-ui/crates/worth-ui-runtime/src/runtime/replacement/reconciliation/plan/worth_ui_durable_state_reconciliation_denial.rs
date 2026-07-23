use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationCounters,
    WorthUiNodeLifecycleTransition,
};

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
    MissingInventoryFamily {
        family_id: WorthUiDurableStateFamilyId,
        counters: Box<WorthUiDurableStateReconciliationCounters>,
    },
    UnsupportedCustomTransition {
        identity_basis: String,
        family_id: WorthUiDurableStateFamilyId,
        transition: WorthUiNodeLifecycleTransition,
        counters: Box<WorthUiDurableStateReconciliationCounters>,
    },
}
