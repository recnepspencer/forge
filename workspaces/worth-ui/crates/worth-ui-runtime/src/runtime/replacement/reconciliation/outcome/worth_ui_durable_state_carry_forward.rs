use crate::runtime::{WorthUiDurableStateFamilyId, WorthUiNodeLifecycleTransition};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDurableStateCarryForward {
    identity_basis: String,
    family_id: WorthUiDurableStateFamilyId,
    transition: WorthUiNodeLifecycleTransition,
}

impl WorthUiDurableStateCarryForward {
    pub(crate) fn new(
        identity_basis: String,
        family_id: WorthUiDurableStateFamilyId,
        transition: WorthUiNodeLifecycleTransition,
    ) -> Self {
        Self {
            identity_basis,
            family_id,
            transition,
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
}
