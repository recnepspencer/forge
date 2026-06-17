#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitEdgeFragmentDenialKind {
    MismatchedSplitVertexScheduleSet,
    MissingSplitVertexSchedule,
    ForeignSplitVertexSchedule,
    ForeignIntervalSubdivisionSchedule,
    AmbiguousFragmentBasis,
    NonFiniteFragmentBoundary,
    UnorderedFragmentBoundary,
    CollapsedSplitFragment,
    GapInSourceEdgeCoverage,
    OverlappingFragmentRange,
    MissingFragmentProvenance,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeFragmentDenial {
    kind: PlanarBooleanSplitEdgeFragmentDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanSplitEdgeFragmentDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitEdgeFragmentDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitEdgeFragmentDenialKind {
        self.kind
    }
    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
