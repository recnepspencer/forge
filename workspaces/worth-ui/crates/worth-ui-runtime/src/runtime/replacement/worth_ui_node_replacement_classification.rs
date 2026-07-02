use crate::runtime::{WorthUiIdentityMatchNodeKind, WorthUiNodeLifecycleTransition};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiNodeReplacementClassification {
    identity_basis: String,
    authored_provenance_digest: Option<u64>,
    transition: WorthUiNodeLifecycleTransition,
    active_kind: Option<WorthUiIdentityMatchNodeKind>,
    candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
    active_durable_state_eligible: bool,
    candidate_durable_state_eligible: bool,
}

impl WorthUiNodeReplacementClassification {
    pub(crate) fn new(
        identity_basis: String,
        authored_provenance_digest: Option<u64>,
        transition: WorthUiNodeLifecycleTransition,
        active_kind: Option<WorthUiIdentityMatchNodeKind>,
        candidate_kind: Option<WorthUiIdentityMatchNodeKind>,
        active_durable_state_eligible: bool,
        candidate_durable_state_eligible: bool,
    ) -> Self {
        Self {
            identity_basis,
            authored_provenance_digest,
            transition,
            active_kind,
            candidate_kind,
            active_durable_state_eligible,
            candidate_durable_state_eligible,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn authored_provenance_digest(&self) -> Option<u64> {
        self.authored_provenance_digest
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
