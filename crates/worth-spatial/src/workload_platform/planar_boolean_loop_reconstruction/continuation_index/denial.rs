use super::counters::PlanarBooleanFragmentContinuationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanFragmentContinuationDenialKind {
    ForeignRequestLineage,
    ForeignSourceProvenanceBundle,
    ForeignSplitVertexSet,
    ForeignFragmentSet,
    ForeignOverlapChainSet,
    MissingFragmentMembership,
    MissingSplitVertexBinding,
    MissingOverlapChainBinding,
    DuplicateSplitVertexIdentity,
    DuplicateContinuationSlot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanFragmentContinuationDenial {
    kind: PlanarBooleanFragmentContinuationDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanFragmentContinuationCounters,
    human_reason: &'static str,
}

impl PlanarBooleanFragmentContinuationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanFragmentContinuationDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanFragmentContinuationCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanFragmentContinuationDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanFragmentContinuationCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
