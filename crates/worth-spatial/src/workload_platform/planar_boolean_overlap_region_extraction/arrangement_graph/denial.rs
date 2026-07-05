use super::counters::PlanarBooleanOverlapArrangementGraphCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapArrangementGraphDenialKind {
    ArrangementOrderingBasisMismatchDenied,
    AmbiguousArrangementSegmentOrderingDenied,
    ContradictoryArrangementNeighborhoodDenied,
    DisconnectedArrangementNeighborhoodDenied,
    NoConcreteCellSubstrateDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementGraphDenial {
    kind: PlanarBooleanOverlapArrangementGraphDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapArrangementGraphCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapArrangementGraphDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapArrangementGraphDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapArrangementGraphCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapArrangementGraphDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapArrangementGraphCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        self.human_reason
    }
}
