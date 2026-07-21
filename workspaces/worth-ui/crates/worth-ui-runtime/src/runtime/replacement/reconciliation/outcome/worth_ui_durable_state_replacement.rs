use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationOutcome,
    WorthUiNodeLifecycleTransition,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateReplacement {
    identity_basis: String,
    family_id: WorthUiDurableStateFamilyId,
    transition: WorthUiNodeLifecycleTransition,
    outcome: WorthUiDurableStateReconciliationOutcome,
    reason: &'static str,
}

impl WorthUiDurableStateReplacement {
    pub(crate) fn new(
        identity_basis: String,
        family_id: WorthUiDurableStateFamilyId,
        transition: WorthUiNodeLifecycleTransition,
        outcome: WorthUiDurableStateReconciliationOutcome,
        reason: &'static str,
    ) -> Self {
        Self {
            identity_basis,
            family_id,
            transition,
            outcome,
            reason,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn family_id(&self) -> &WorthUiDurableStateFamilyId {
        &self.family_id
    }

    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }

    pub fn outcome(&self) -> WorthUiDurableStateReconciliationOutcome {
        self.outcome
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
