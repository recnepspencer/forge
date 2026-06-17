use super::counters::PlanarBooleanSegmentPairEnumerationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSegmentPairEnumerationDenialKind {
    CandidateEnvelopeInvalid,
    EmittedPairBreadthMismatch,
    OperandSideMismatch,
    PairBreadthOverflow,
    QueryIndexNotAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSegmentPairEnumerationDenial {
    kind: PlanarBooleanSegmentPairEnumerationDenialKind,
    canonical_segment_set_identity: String,
    counters: PlanarBooleanSegmentPairEnumerationCounters,
    human_reason: String,
}

impl PlanarBooleanSegmentPairEnumerationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSegmentPairEnumerationDenialKind,
        canonical_segment_set_identity: impl Into<String>,
        counters: PlanarBooleanSegmentPairEnumerationCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            canonical_segment_set_identity: canonical_segment_set_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSegmentPairEnumerationDenialKind {
        self.kind
    }

    pub fn canonical_segment_set_identity(&self) -> &str {
        &self.canonical_segment_set_identity
    }

    pub fn counters(&self) -> PlanarBooleanSegmentPairEnumerationCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
