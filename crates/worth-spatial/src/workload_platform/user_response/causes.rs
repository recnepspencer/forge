use super::validation::normalize_human_text;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUserOutcomeCauseKind {
    PolicyRequired,
    UnsupportedInput,
    DeniedMovementOrRotation,
    PredicateUncertain,
    PredicateEvaluationFailed,
    PredicateAuthorityNotBound,
    OverlapDenied,
    IntegrityMismatch,
    DirtyInput,
    MissingEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUserOutcomeCause {
    kind: WorthUserOutcomeCauseKind,
    human_reason: String,
}

impl WorthUserOutcomeCause {
    pub(crate) fn new(kind: WorthUserOutcomeCauseKind, human_reason: impl Into<String>) -> Self {
        Self {
            kind,
            human_reason: normalize_human_text(human_reason),
        }
    }

    pub fn kind(&self) -> WorthUserOutcomeCauseKind {
        self.kind
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn no_options_cause(&self) -> Option<WorthNoOptionsCause> {
        match self.kind {
            WorthUserOutcomeCauseKind::DirtyInput => Some(WorthNoOptionsCause::DirtyInput),
            WorthUserOutcomeCauseKind::MissingEvidence => {
                Some(WorthNoOptionsCause::MissingEvidence)
            }
            _ => None,
        }
    }

    pub fn unsupported_cause(&self) -> Option<WorthUnsupportedCause> {
        match self.kind {
            WorthUserOutcomeCauseKind::UnsupportedInput => {
                Some(WorthUnsupportedCause::UnsupportedInput)
            }
            _ => None,
        }
    }

    pub fn denied_cause(&self) -> Option<WorthDeniedCause> {
        match self.kind {
            WorthUserOutcomeCauseKind::DeniedMovementOrRotation => {
                Some(WorthDeniedCause::DeniedMovementOrRotation)
            }
            WorthUserOutcomeCauseKind::OverlapDenied => Some(WorthDeniedCause::OverlapDenied),
            _ => None,
        }
    }

    pub fn integrity_mismatch_cause(&self) -> Option<WorthIntegrityMismatchCause> {
        match self.kind {
            WorthUserOutcomeCauseKind::IntegrityMismatch => {
                Some(WorthIntegrityMismatchCause::RetainedReplayProjectionDrift)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthNoOptionsCause {
    DirtyInput,
    MissingEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUnsupportedCause {
    UnsupportedInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthDeniedCause {
    DeniedMovementOrRotation,
    OverlapDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthIntegrityMismatchCause {
    RetainedReplayProjectionDrift,
}
