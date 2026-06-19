#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReplayParityRowKind {
    LoopEvidenceReceipt,
    ReconstructedLoopSet,
    BornLoopSet,
    IslandPartition,
    SplitAttribution,
    RoleOutcomeSet,
    DegenerateOutcomeSet,
    DecisionLog,
    LoopLedgerReceipt,
    DownstreamConsumption,
    RetainedReplayCheckpoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReplayParityRow {
    kind: PlanarBooleanLoopReplayParityRowKind,
    original_identity: String,
    replayed_identity: String,
}

impl PlanarBooleanLoopReplayParityRow {
    pub(crate) fn new(
        kind: PlanarBooleanLoopReplayParityRowKind,
        original_identity: impl Into<String>,
        replayed_identity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            original_identity: original_identity.into(),
            replayed_identity: replayed_identity.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReplayParityRowKind {
        self.kind
    }

    pub fn original_identity(&self) -> &str {
        &self.original_identity
    }

    pub fn replayed_identity(&self) -> &str {
        &self.replayed_identity
    }
}
