#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarLocalRebuildParityDenialKind {
    MissingLocalNeighborhood,
    BroadSearchNotAllowed,
    MissingRebindingContinuity,
    CorrespondenceOnlyRebinding,
    MissingPlanarReceipt,
    MismatchedRetainedProjectionBasis,
    MismatchedStructuralIdentityBasis,
    MismatchedMotionPostureBasis,
    MismatchedTopologyBasis,
    RecoveryReclassifiedTruth,
    DiagnosticReclassifiedTruth,
    KernelSummaryNotAuthority,
    ProjectionConsumedIdentityRecomputed,
    MismatchedRebindingNeighborhood,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildParityDenial {
    kind: PlanarLocalRebuildParityDenialKind,
    reason: &'static str,
    counters: super::PlanarLocalRebuildParityCounters,
}

impl PlanarLocalRebuildParityDenial {
    pub(crate) fn new(kind: PlanarLocalRebuildParityDenialKind, reason: &'static str) -> Self {
        Self {
            kind,
            reason,
            counters: super::PlanarLocalRebuildParityCounters::certified(0, 0, 0, 0),
        }
    }

    pub fn kind(&self) -> PlanarLocalRebuildParityDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        self.reason
    }

    pub fn counters(&self) -> super::PlanarLocalRebuildParityCounters {
        self.counters
    }
}
