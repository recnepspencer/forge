use super::counters::PlanarBooleanOverlapAdjacencyIndexCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapAdjacencyIndexDenialKind {
    UnindexedOverlapNeighborhoodDiscoveryDenied,
    IncidentalIterationOrderTieBreakDenied,
    DanglingAdjacencyLineageDenied,
    ContradictoryAdjacencyNeighborhoodDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapAdjacencyIndexDenial {
    kind: PlanarBooleanOverlapAdjacencyIndexDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapAdjacencyIndexCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapAdjacencyIndexDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapAdjacencyIndexDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapAdjacencyIndexCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapAdjacencyIndexDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapAdjacencyIndexCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
