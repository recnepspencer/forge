use super::counters::PlanarBooleanSplitChainValidationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitChainValidationDenialKind {
    ForeignOverlapChainSet,
    EmptyFragmentSchedule,
    FragmentGap,
    FragmentOverlap,
    MalformedFragmentRange,
    DuplicateFragmentIdentity,
    DanglingOverlapFragmentReference,
    MismatchedOverlapFragmentAuthority,
    MismatchedOverlapIntervalBasis,
    MalformedOverlapIntervalBasis,
    OverlapFragmentOutsideSourceInterval,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitChainValidationDenial {
    kind: PlanarBooleanSplitChainValidationDenialKind,
    evidence_identity: String,
    counters: PlanarBooleanSplitChainValidationCounters,
    human_reason: String,
}

impl PlanarBooleanSplitChainValidationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitChainValidationDenialKind,
        evidence_identity: impl Into<String>,
        counters: PlanarBooleanSplitChainValidationCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitChainValidationDenialKind {
        self.kind
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub fn counters(&self) -> PlanarBooleanSplitChainValidationCounters {
        self.counters
    }
    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
