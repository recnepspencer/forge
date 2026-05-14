use crate::intent_admission::ForgeQueryIntentAdmissionExecutionSeam;
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryIntentSourceLane};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSupportEligibility {
    Admitted,
    Deferred(&'static str),
    Unsupported(&'static str),
}

impl ForgeQueryIntentAdmissionSupportEligibility {
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
pub enum ForgeQueryIntentAdmissionCapabilityEligibility {
    Admitted,
    Violation {
        stage: &'static str,
        detail: &'static str,
    },
}

impl ForgeQueryIntentAdmissionCapabilityEligibility {
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
pub enum ForgeQueryIntentAdmissionPolicyEligibility {
    NotApplicableForRuntimeIntentFloor,
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionPolicyEligibility {
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
pub enum ForgeQueryIntentAdmissionBasisEligibility {
    NotApplicableForRuntimeIntentFloor,
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionBasisEligibility {
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
pub enum ForgeQueryIntentAdmissionInvariantEligibility {
    PreExecutionAuthorityRequired,
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionInvariantEligibility {
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
pub enum ForgeQueryIntentAdmissionProjectionSourceEligibility {
    NotApplicableForRuntimeIntentFloor,
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionProjectionSourceEligibility {
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
pub enum ForgeQueryIntentAdmissionRoutingSupportEligibility {
    CoveredExecutionSeam(ForgeQueryIntentAdmissionExecutionSeam),
    DeferredNeighbor(&'static str),
    Unsupported(&'static str),
}

impl ForgeQueryIntentAdmissionRoutingSupportEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredExecutionSeam(seam) => seam.as_str(),
            Self::DeferredNeighbor(reason) | Self::Unsupported(reason) => reason,
        }
    }

    pub fn detail(self) -> Option<&'static str> {
        match self {
            Self::CoveredExecutionSeam(_) => None,
            Self::DeferredNeighbor(reason) | Self::Unsupported(reason) => Some(reason),
        }
    }

    pub fn covered_execution_seam(self) -> Option<ForgeQueryIntentAdmissionExecutionSeam> {
        match self {
            Self::CoveredExecutionSeam(seam) => Some(seam),
            Self::DeferredNeighbor(_) | Self::Unsupported(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionSourceLaneEligibility {
    MatchesExpected(ForgeQueryIntentSourceLane),
    Mismatch {
        expected: ForgeQueryIntentSourceLane,
        actual: ForgeQueryIntentSourceLane,
    },
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionSourceLaneEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MatchesExpected(expected) => expected.as_str(),
            Self::Mismatch { actual, .. } => actual.as_str(),
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
            Self::DeferredNeighbor(reason) => Some(reason.to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentAdmissionAuthorityLaneEligibility {
    MatchesExpected(ForgeQueryAuthorityLane),
    Mismatch {
        expected: ForgeQueryAuthorityLane,
        actual: ForgeQueryAuthorityLane,
    },
    DeferredNeighbor(&'static str),
}

impl ForgeQueryIntentAdmissionAuthorityLaneEligibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MatchesExpected(expected) => expected.as_str(),
            Self::Mismatch { actual, .. } => actual.as_str(),
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
            Self::DeferredNeighbor(reason) => Some(reason.to_string()),
        }
    }
}
