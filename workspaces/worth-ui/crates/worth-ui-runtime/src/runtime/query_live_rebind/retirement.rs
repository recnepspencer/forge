use crate::runtime::query_binding::{WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRetirementReason {
    CandidateRemovedQueryBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingRetirement {
    identity: WorthUiQueryBindingIdentity,
    active_posture: WorthUiQueryBindingPosture,
    reason: WorthUiQueryBindingRetirementReason,
}

impl WorthUiQueryBindingRetirement {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_posture: WorthUiQueryBindingPosture,
        reason: WorthUiQueryBindingRetirementReason,
    ) -> Self {
        Self {
            identity,
            active_posture,
            reason,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_posture(&self) -> &WorthUiQueryBindingPosture {
        &self.active_posture
    }

    pub fn reason(&self) -> WorthUiQueryBindingRetirementReason {
        self.reason
    }
}
