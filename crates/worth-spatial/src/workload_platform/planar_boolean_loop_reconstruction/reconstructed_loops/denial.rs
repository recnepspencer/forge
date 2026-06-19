use super::counters::PlanarBooleanReconstructedLoopBoundaryCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanReconstructedLoopBoundaryDenialKind {
    ContradictoryIslandOwnership,
    FragmentLeakageAcrossIslands,
    UnsupportedMultiSourceBornAncestry,
    UntrackedBornLoopEmergence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReconstructedLoopBoundaryDenial {
    kind: PlanarBooleanReconstructedLoopBoundaryDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanReconstructedLoopBoundaryCounters,
    human_reason: &'static str,
}

impl PlanarBooleanReconstructedLoopBoundaryDenial {
    pub(crate) fn new(
        kind: PlanarBooleanReconstructedLoopBoundaryDenialKind,
        rejected_identity: String,
        counters: PlanarBooleanReconstructedLoopBoundaryCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity,
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanReconstructedLoopBoundaryDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanReconstructedLoopBoundaryCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
