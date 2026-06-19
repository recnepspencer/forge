use super::counters::PlanarBooleanLoopReplayParityCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReplayParityDenialKind {
    LoopEvidenceMismatch,
    ReconstructedLoopMismatch,
    BornLoopMismatch,
    IslandPartitionMismatch,
    SplitAttributionMismatch,
    RoleOutcomeMismatch,
    DegenerateOutcomeMismatch,
    DecisionLogMismatch,
    LoopLedgerMismatch,
    CheckpointAuthorityMismatch,
    CheckpointParityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReplayParityDenial {
    kind: PlanarBooleanLoopReplayParityDenialKind,
    expected: String,
    actual: String,
    counters: PlanarBooleanLoopReplayParityCounters,
}

impl PlanarBooleanLoopReplayParityDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopReplayParityDenialKind,
        expected: impl Into<String>,
        actual: impl Into<String>,
        counters: PlanarBooleanLoopReplayParityCounters,
    ) -> Self {
        Self {
            kind,
            expected: expected.into(),
            actual: actual.into(),
            counters,
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReplayParityDenialKind {
        self.kind
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn actual(&self) -> &str {
        &self.actual
    }

    pub fn counters(&self) -> PlanarBooleanLoopReplayParityCounters {
        self.counters
    }
}
