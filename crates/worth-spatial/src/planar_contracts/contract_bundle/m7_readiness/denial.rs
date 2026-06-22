use super::PlanarM7ReadinessCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarM7ReadinessDenialKind {
    MissingCloseoutFamily,
    MismatchedBooleanReadinessRoot,
    MismatchedStructuralIdentity,
    MismatchedMotionPosture,
    MismatchedRetainedFacts,
    MismatchedProjectionConsumption,
    MismatchedRecoveryPosture,
    MismatchedDiagnostics,
    MissingSupportPosture,
    BooleanExecutionAlreadyPresent,
    QueryBoundaryMismatch,
}

impl PlanarM7ReadinessDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingCloseoutFamily => "missing-closeout-family",
            Self::MismatchedBooleanReadinessRoot => "mismatched-boolean-readiness-root",
            Self::MismatchedStructuralIdentity => "mismatched-structural-identity",
            Self::MismatchedMotionPosture => "mismatched-motion-posture",
            Self::MismatchedRetainedFacts => "mismatched-retained-facts",
            Self::MismatchedProjectionConsumption => "mismatched-projection-consumption",
            Self::MismatchedRecoveryPosture => "mismatched-recovery-posture",
            Self::MismatchedDiagnostics => "mismatched-diagnostics",
            Self::MissingSupportPosture => "missing-support-posture",
            Self::BooleanExecutionAlreadyPresent => "boolean-execution-already-present",
            Self::QueryBoundaryMismatch => "query-boundary-mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarM7ReadinessDenial {
    kind: PlanarM7ReadinessDenialKind,
    reason: String,
    counters: PlanarM7ReadinessCounters,
}

impl PlanarM7ReadinessDenial {
    pub(crate) fn new(kind: PlanarM7ReadinessDenialKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
            counters: PlanarM7ReadinessCounters::rejected(),
        }
    }

    pub fn kind(&self) -> PlanarM7ReadinessDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PlanarM7ReadinessCounters {
        self.counters
    }
}
