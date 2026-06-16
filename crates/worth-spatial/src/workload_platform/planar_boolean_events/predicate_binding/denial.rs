use super::counters::PlanarBooleanEventPredicateBindingCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventPredicateBindingDenialKind {
    MissingReducedPairIdentity,
    EmptyPairWorklist,
    SegmentContractCountMismatch,
    MissingSegmentContractForPair,
    DuplicateSegmentContractForPair,
    SegmentContractIdentityMismatch,
    SegmentContractLocalFrameMismatch,
    SegmentContractPrecisionBasisMismatch,
    PredicateConsumptionSegmentSetMismatch,
    PredicateConsumptionMissingNoSecondEngineProof,
    PredicateConsumptionRowCountMismatch,
    PredicateConsumptionLocalFrameMismatch,
    PredicateConsumptionPrecisionBasisMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventPredicateBindingDenial {
    kind: PlanarBooleanEventPredicateBindingDenialKind,
    reduced_pair_identity: String,
    segment_pair_identity: String,
    counters: PlanarBooleanEventPredicateBindingCounters,
    human_reason: String,
}

impl PlanarBooleanEventPredicateBindingDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEventPredicateBindingDenialKind,
        reduced_pair_identity: impl Into<String>,
        segment_pair_identity: impl Into<String>,
        counters: PlanarBooleanEventPredicateBindingCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reduced_pair_identity: reduced_pair_identity.into(),
            segment_pair_identity: segment_pair_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEventPredicateBindingDenialKind {
        self.kind
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn counters(&self) -> PlanarBooleanEventPredicateBindingCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
