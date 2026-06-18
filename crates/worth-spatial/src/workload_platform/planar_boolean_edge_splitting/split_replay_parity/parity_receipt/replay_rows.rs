#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitReplayParityRowKind {
    SplitRequest,
    SplitLedgerReceipt,
    DownstreamConsumption,
    DecisionLogReceipt,
    OperationalTruthDigest,
    FragmentSet,
    OverlapChainSet,
    PersistentNamingReceipt,
    RetainedReplayCheckpoint,
    ReversedSourceSenseCanonicalization,
    ReplayProduct,
    ReplayClosureManifest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayParityRow {
    kind: PlanarBooleanEdgeSplitReplayParityRowKind,
    parity_row_identity: String,
    original_identity: String,
    replayed_identity: String,
}

impl PlanarBooleanEdgeSplitReplayParityRow {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitReplayParityRowKind,
        original_identity: impl Into<String>,
        replayed_identity: impl Into<String>,
    ) -> Self {
        let original_identity = original_identity.into();
        let replayed_identity = replayed_identity.into();
        let parity_row_identity = format!(
            "edge-split-replay-parity-row:{kind:?}:{original_identity}:{replayed_identity}"
        );
        Self {
            kind,
            parity_row_identity,
            original_identity,
            replayed_identity,
        }
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitReplayParityRowKind {
        self.kind
    }

    pub fn parity_row_identity(&self) -> &str {
        &self.parity_row_identity
    }

    pub fn original_identity(&self) -> &str {
        &self.original_identity
    }

    pub fn replayed_identity(&self) -> &str {
        &self.replayed_identity
    }

    pub fn certifies_match(&self) -> bool {
        self.original_identity == self.replayed_identity
    }
}
