use crate::intent_admission::WorthQueryIntentAdmissionExecutionSeam;
use crate::runtime::{WorthQueryAuthorityLane, WorthQueryIntentSourceLane};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSupportEligibility {
    Admitted,
    Deferred(&'static str),
    Unsupported(&'static str),
}

impl WorthQueryIntentAdmissionSupportEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Deferred(_) => "deferred",
            Self::Unsupported(_) => "unsupported",
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::Admitted => None,
            Self::Deferred(detail) | Self::Unsupported(detail) => Some(detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionCapabilityEligibility {
    Admitted,
    Violation {
        stage: &'static str,
        detail: &'static str,
    },
}

impl WorthQueryIntentAdmissionCapabilityEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Violation { .. } => "violation",
        }
    }

    pub fn violation_detail(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Admitted => None,
            Self::Violation { stage, detail } => Some((stage, detail)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionPolicyEligibility {
    NotApplicableForRuntimeIntentFloor,
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionPolicyEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => "not-applicable-runtime-intent-floor",
            Self::DeferredNeighbor(_) => "deferred-neighbor",
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => None,
            Self::DeferredNeighbor(detail) => Some(detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionBasisEligibility {
    NotApplicableForRuntimeIntentFloor,
    ObservationLifecycleAdmitted,
    ObservationLifecycleViolation(&'static str),
    ReadExecutionCurrentRuntimeAdmitted,
    ReadExecutionBasisContextAdmitted,
    ReadExecutionBasisContextViolation(&'static str),
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionBasisEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => "not-applicable-runtime-intent-floor",
            Self::ObservationLifecycleAdmitted => "basis-observation-lifecycle-admitted",
            Self::ObservationLifecycleViolation(_) => "basis-observation-lifecycle-violation",
            Self::ReadExecutionCurrentRuntimeAdmitted => "read-execution-current-runtime-admitted",
            Self::ReadExecutionBasisContextAdmitted => "read-execution-basis-context-admitted",
            Self::ReadExecutionBasisContextViolation(_) => "read-execution-basis-context-violation",
            Self::DeferredNeighbor(_) => "deferred-neighbor",
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => None,
            Self::ObservationLifecycleAdmitted => None,
            Self::ObservationLifecycleViolation(detail) => Some(detail),
            Self::ReadExecutionCurrentRuntimeAdmitted => None,
            Self::ReadExecutionBasisContextAdmitted => None,
            Self::ReadExecutionBasisContextViolation(detail) => Some(detail),
            Self::DeferredNeighbor(detail) => Some(detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionInvariantEligibility {
    PreExecutionAuthorityRequired,
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionInvariantEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PreExecutionAuthorityRequired => "pre-execution-authority-required",
            Self::DeferredNeighbor(_) => "deferred-neighbor",
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::PreExecutionAuthorityRequired => None,
            Self::DeferredNeighbor(detail) => Some(detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionProjectionSourceEligibility {
    NotApplicableForRuntimeIntentFloor,
    ProjectionConsumptionAdmitted,
    ProjectionConsumptionAdmittedWithWarnings(&'static str),
    ProjectionConsumptionViolation(&'static str),
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionProjectionSourceEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => "not-applicable-runtime-intent-floor",
            Self::ProjectionConsumptionAdmitted => "projection-consumption-admitted",
            Self::ProjectionConsumptionAdmittedWithWarnings(_) => {
                "projection-consumption-admitted-with-warnings"
            }
            Self::ProjectionConsumptionViolation(_) => "projection-consumption-violation",
            Self::DeferredNeighbor(_) => "deferred-neighbor",
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::NotApplicableForRuntimeIntentFloor => None,
            Self::ProjectionConsumptionAdmitted => None,
            Self::ProjectionConsumptionAdmittedWithWarnings(detail)
            | Self::ProjectionConsumptionViolation(detail) => Some(detail),
            Self::DeferredNeighbor(detail) => Some(detail),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionRoutingSupportEligibility {
    CoveredExecutionSeam(WorthQueryIntentAdmissionExecutionSeam),
    NoExecutionHandoff(&'static str),
    DeferredNeighbor(&'static str),
    Unsupported(&'static str),
}

impl WorthQueryIntentAdmissionRoutingSupportEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredExecutionSeam(seam) => seam.as_str(),
            Self::NoExecutionHandoff(reason)
            | Self::DeferredNeighbor(reason)
            | Self::Unsupported(reason) => reason,
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::CoveredExecutionSeam(_) => None,
            Self::NoExecutionHandoff(reason)
            | Self::DeferredNeighbor(reason)
            | Self::Unsupported(reason) => Some(reason),
        }
    }

    pub fn covered_execution_seam(self) -> Option<WorthQueryIntentAdmissionExecutionSeam> {
        match self {
            Self::CoveredExecutionSeam(seam) => Some(seam),
            Self::NoExecutionHandoff(_) | Self::DeferredNeighbor(_) | Self::Unsupported(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionSourceLaneEligibility {
    MatchesExpected(WorthQueryIntentSourceLane),
    Mismatch {
        expected: WorthQueryIntentSourceLane,
        actual: WorthQueryIntentSourceLane,
    },
    NotApplicableNonRuntimeFamily,
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionSourceLaneEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MatchesExpected(expected) => expected.as_str(),
            Self::Mismatch { actual, .. } => actual.as_str(),
            Self::NotApplicableNonRuntimeFamily => "not-applicable-non-runtime-family",
            Self::DeferredNeighbor(reason) => reason,
        }
    }

    pub fn detail(self) -> Option<String> {
        match self {
            Self::MatchesExpected(_) => None,
            Self::Mismatch { expected, actual } => Some(format!(
                "expected:{} actual:{}",
                expected.as_str(),
                actual.as_str()
            )),
            Self::NotApplicableNonRuntimeFamily => None,
            Self::DeferredNeighbor(reason) => Some(reason.to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentAdmissionAuthorityLaneEligibility {
    MatchesExpected(WorthQueryAuthorityLane),
    Mismatch {
        expected: WorthQueryAuthorityLane,
        actual: WorthQueryAuthorityLane,
    },
    NotApplicableNonRuntimeFamily,
    DeferredNeighbor(&'static str),
}

impl WorthQueryIntentAdmissionAuthorityLaneEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MatchesExpected(expected) => expected.as_str(),
            Self::Mismatch { actual, .. } => actual.as_str(),
            Self::NotApplicableNonRuntimeFamily => "not-applicable-non-runtime-family",
            Self::DeferredNeighbor(reason) => reason,
        }
    }

    pub fn detail(self) -> Option<String> {
        match self {
            Self::MatchesExpected(_) => None,
            Self::Mismatch { expected, actual } => Some(format!(
                "expected:{} actual:{}",
                expected.as_str(),
                actual.as_str()
            )),
            Self::NotApplicableNonRuntimeFamily => None,
            Self::DeferredNeighbor(reason) => Some(reason.to_string()),
        }
    }
}
