use super::RetainedPlanarFactsCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetainedPlanarFactsDenialKind {
    MissingBooleanReadinessReceipt,
    MissingStructuralIdentityReceipt,
    MissingMotionPostureReceipt,
    MissingTopologyContractReceipt,
    MissingRetainedFamilyRows,
    MissingPlanarClassificationRetention,
    MismatchedBooleanReadinessBasis,
    MismatchedMotionPosture,
    MismatchedTopologyContract,
    TruncatedRetainedBasis,
    UnsupportedBranchBasis,
}

impl RetainedPlanarFactsDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingBooleanReadinessReceipt => "missing-boolean-readiness-receipt",
            Self::MissingStructuralIdentityReceipt => "missing-structural-identity-receipt",
            Self::MissingMotionPostureReceipt => "missing-motion-posture-receipt",
            Self::MissingTopologyContractReceipt => "missing-topology-contract-receipt",
            Self::MissingRetainedFamilyRows => "missing-retained-family-rows",
            Self::MissingPlanarClassificationRetention => "missing-planar-classification-retention",
            Self::MismatchedBooleanReadinessBasis => "mismatched-boolean-readiness-basis",
            Self::MismatchedMotionPosture => "mismatched-motion-posture",
            Self::MismatchedTopologyContract => "mismatched-topology-contract",
            Self::TruncatedRetainedBasis => "truncated-retained-basis",
            Self::UnsupportedBranchBasis => "unsupported-branch-basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetainedPlanarFactsDenial {
    kind: RetainedPlanarFactsDenialKind,
    reason: String,
    counters: RetainedPlanarFactsCounters,
}

impl RetainedPlanarFactsDenial {
    pub(crate) fn new(kind: RetainedPlanarFactsDenialKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
            counters: RetainedPlanarFactsCounters::rejected(),
        }
    }

    pub fn kind(&self) -> RetainedPlanarFactsDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> RetainedPlanarFactsCounters {
        self.counters
    }
}
