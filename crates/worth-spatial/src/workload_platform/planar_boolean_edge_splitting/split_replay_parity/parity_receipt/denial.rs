#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitReplayParityDenialKind {
    MissingRetainedReplayReceipt,
    MissingSplitRequestRetainedReplay,
    MissingSplitLedgerReceipt,
    MissingDecisionLogReceipt,
    MissingOperationalTruthDigest,
    ReplaySplitRequestMismatch,
    ReplaySplitRequestLedgerMismatch,
    ForeignRetainedReplayReceipt,
    ReplayLedgerMismatch,
    ReplayDecisionLogMismatch,
    ReplayOperationalTruthMismatch,
    ReplayFragmentMismatch,
    ReplayOverlapChainMismatch,
    ReplayPersistentNamingMismatch,
    ReplayProductNotQueryOwned,
    CheckpointParityMismatch,
    OrientationCanonicalizationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayParityDenial {
    kind: PlanarBooleanEdgeSplitReplayParityDenialKind,
    rejected_identity: String,
    expected_identity: String,
    observed_identity: String,
    human_reason: String,
}

impl PlanarBooleanEdgeSplitReplayParityDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitReplayParityDenialKind,
        rejected_identity: impl Into<String>,
        expected_identity: impl Into<String>,
        observed_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            expected_identity: expected_identity.into(),
            observed_identity: observed_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitReplayParityDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn expected_identity(&self) -> &str {
        &self.expected_identity
    }

    pub fn observed_identity(&self) -> &str {
        &self.observed_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
