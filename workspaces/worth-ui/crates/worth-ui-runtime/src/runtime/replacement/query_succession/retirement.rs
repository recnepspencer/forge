use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRetirementReason {
    CandidateRemovedQueryBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingRetirement {
    identity: WorthUiQueryBindingIdentity,
    active_ui_requirements: WorthUiQueryBindingUiRequirements,
    reason: WorthUiQueryBindingRetirementReason,
}

impl WorthUiQueryBindingRetirement {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_ui_requirements: WorthUiQueryBindingUiRequirements,
        reason: WorthUiQueryBindingRetirementReason,
    ) -> Self {
        Self {
            identity,
            active_ui_requirements,
            reason,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_ui_requirements(&self) -> &WorthUiQueryBindingUiRequirements {
        &self.active_ui_requirements
    }

    pub fn reason(&self) -> WorthUiQueryBindingRetirementReason {
        self.reason
    }
}
