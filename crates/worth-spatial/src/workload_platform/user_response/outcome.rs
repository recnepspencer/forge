use super::{
    HumanReadableResponse, WorthPolicyDecision, WorthUserOutcomeCause, WorthUserOutcomeCauseKind,
    WorthUserResponseEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUserOutcomeKind {
    Admitted,
    PolicyRequired,
    Unsupported,
    Denied,
    PredicateUncertain,
    IntegrityMismatch,
    NoOptions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUserOutcome {
    kind: WorthUserOutcomeKind,
    cause: Option<WorthUserOutcomeCause>,
    human_response: HumanReadableResponse,
    evidence: WorthUserResponseEvidence,
    choices: Vec<WorthPolicyDecision>,
}

impl WorthUserOutcome {
    pub(crate) fn admitted(
        human_response: HumanReadableResponse,
        evidence: WorthUserResponseEvidence,
    ) -> Self {
        Self {
            kind: WorthUserOutcomeKind::Admitted,
            cause: None,
            human_response,
            evidence,
            choices: Vec::new(),
        }
    }

    pub(crate) fn policy_required(
        cause: WorthUserOutcomeCause,
        human_response: HumanReadableResponse,
        evidence: WorthUserResponseEvidence,
        choices: Vec<WorthPolicyDecision>,
    ) -> Self {
        Self {
            kind: WorthUserOutcomeKind::PolicyRequired,
            cause: Some(cause),
            human_response,
            evidence,
            choices,
        }
    }

    pub(crate) fn no_options(
        cause: WorthUserOutcomeCause,
        human_response: HumanReadableResponse,
        evidence: WorthUserResponseEvidence,
    ) -> Self {
        Self {
            kind: no_options_kind(cause.kind()),
            cause: Some(cause),
            human_response,
            evidence,
            choices: Vec::new(),
        }
    }

    pub fn kind(&self) -> WorthUserOutcomeKind {
        self.kind
    }

    pub fn cause(&self) -> Option<&WorthUserOutcomeCause> {
        self.cause.as_ref()
    }

    pub fn human_response(&self) -> &HumanReadableResponse {
        &self.human_response
    }

    pub fn evidence(&self) -> &WorthUserResponseEvidence {
        &self.evidence
    }

    pub fn choices(&self) -> &[WorthPolicyDecision] {
        &self.choices
    }
}

fn no_options_kind(cause: WorthUserOutcomeCauseKind) -> WorthUserOutcomeKind {
    match cause {
        WorthUserOutcomeCauseKind::UnsupportedInput => WorthUserOutcomeKind::Unsupported,
        WorthUserOutcomeCauseKind::DeniedMovementOrRotation
        | WorthUserOutcomeCauseKind::OverlapDenied => WorthUserOutcomeKind::Denied,
        WorthUserOutcomeCauseKind::PredicateUncertain
        | WorthUserOutcomeCauseKind::PredicateEvaluationFailed
        | WorthUserOutcomeCauseKind::PredicateAuthorityNotBound => {
            WorthUserOutcomeKind::PredicateUncertain
        }
        WorthUserOutcomeCauseKind::IntegrityMismatch => WorthUserOutcomeKind::IntegrityMismatch,
        WorthUserOutcomeCauseKind::PolicyRequired
        | WorthUserOutcomeCauseKind::DirtyInput
        | WorthUserOutcomeCauseKind::MissingEvidence => WorthUserOutcomeKind::NoOptions,
    }
}
