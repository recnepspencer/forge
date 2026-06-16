use crate::runtime::{WorthUiIdentityMatchNodeKind, WorthUiNodeLifecycleTransition};
use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiNodeReplacementClassification {
    identity_basis: String,
    transition: WorthUiNodeLifecycleTransition,
    active_kind: Option<WorthUiIdentityMatchNodeKind>,
    candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
    active_handle: Option<WorthUiArtifactHandle>,
    candidate_handle: Option<WorthUiArtifactHandle>,
    active_durable_state_eligible: bool,
    candidate_durable_state_eligible: bool,
}

impl WorthUiNodeReplacementClassification {
    pub(crate) fn new(
        identity_basis: String,
        transition: WorthUiNodeLifecycleTransition,
        active_kind: Option<WorthUiIdentityMatchNodeKind>,
        candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
        active_durable_state_eligible: bool,
        candidate_durable_state_eligible: bool,
    ) -> Self {
        Self {
            identity_basis,
            transition,
            active_kind,
            candidate_kind,
            active_handle: None,
            candidate_handle: None,
            active_durable_state_eligible,
            candidate_durable_state_eligible,
        }
    }

    pub(crate) fn with_artifact_handles(
        mut self,
        active_handle: Option<WorthUiArtifactHandle>,
        candidate_handle: Option<WorthUiArtifactHandle>,
    ) -> Self {
        self.active_handle = active_handle;
        self.candidate_handle = candidate_handle;
        self
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn transition(&self) -> WorthUiNodeLifecycleTransition {
        self.transition
    }

    pub fn active_kind(&self) -> Option<WorthUiIdentityMatchNodeKind> {
        self.active_kind
    }

    pub fn candidate_kind(&self) -> Option<WorthUiIdentityMatchNodeKind> {
        self.candidate_kind
    }

    pub(crate) fn candidate_handle(&self) -> Option<&WorthUiArtifactHandle> {
        self.candidate_handle.as_ref()
    }

    pub fn active_durable_state_eligible(&self) -> bool {
        self.active_durable_state_eligible
    }

    pub fn candidate_durable_state_eligible(&self) -> bool {
        self.candidate_durable_state_eligible
    }

    pub fn unrestored_durable_state_carry_permitted(&self) -> bool {
        matches!(
            self.transition,
            WorthUiNodeLifecycleTransition::Preserve
                | WorthUiNodeLifecycleTransition::Move
                | WorthUiNodeLifecycleTransition::Rebind
        ) && self.active_durable_state_eligible
            && self.candidate_durable_state_eligible
    }
}
