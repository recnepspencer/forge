use crate::runtime::WorthUiDurableStateReconciliationPlan;
use std::collections::BTreeMap;

/// Durable semantic truth emitted from an admitted reconciliation plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationDurableSemanticState {
    reconciliation: WorthUiDurableStateReconciliationPlan,
    committed_resize_by_identity: BTreeMap<u64, crate::runtime::UiResizeAllocationPlanningBasis>,
}

impl UiAllocationDurableSemanticState {
    pub(crate) fn from_reconciliation(
        reconciliation: WorthUiDurableStateReconciliationPlan,
        _: crate::runtime::reconciliation::UiAllocationDurableSemanticStateMintAuthority,
    ) -> Self {
        Self {
            reconciliation,
            committed_resize_by_identity: BTreeMap::new(),
        }
    }

    pub fn reconciliation(&self) -> &WorthUiDurableStateReconciliationPlan {
        &self.reconciliation
    }

    pub fn truth_category(&self) -> crate::evidence::allocation::UiAllocationTruthCategory {
        crate::evidence::allocation::UiAllocationTruthCategory::DurableSemanticState
    }
    pub fn committed_resize(
        &self,
        identity_digest: u64,
    ) -> Option<&crate::runtime::UiResizeAllocationPlanningBasis> {
        self.committed_resize_by_identity.get(&identity_digest)
    }
    pub(crate) fn commit_resize(
        &mut self,
        basis: crate::runtime::UiResizeAllocationPlanningBasis,
    ) -> bool {
        let identity = basis.durable_identity_digest();
        if self.committed_resize_by_identity.get(&identity) == Some(&basis) {
            return false;
        }
        self.committed_resize_by_identity.insert(identity, basis);
        true
    }
}
