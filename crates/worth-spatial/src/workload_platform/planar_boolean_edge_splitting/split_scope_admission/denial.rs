use super::policy_outcome::PlanarBooleanEdgeSplitPolicyOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitScopeAdmissionDenialKind {
    UnsupportedEmptySourceCarrierScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitScopeAdmissionDenial {
    kind: PlanarBooleanEdgeSplitScopeAdmissionDenialKind,
    split_request_identity: String,
    policy_outcome: PlanarBooleanEdgeSplitPolicyOutcome,
    human_reason: String,
}

impl PlanarBooleanEdgeSplitScopeAdmissionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitScopeAdmissionDenialKind,
        split_request_identity: impl Into<String>,
        policy_outcome: PlanarBooleanEdgeSplitPolicyOutcome,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            split_request_identity: split_request_identity.into(),
            policy_outcome,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitScopeAdmissionDenialKind {
        self.kind
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }

    pub fn policy_outcome(&self) -> &PlanarBooleanEdgeSplitPolicyOutcome {
        &self.policy_outcome
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
