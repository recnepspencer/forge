#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitSummumBonumCloseoutDenialKind {
    CandidateIndexNotProduction,
    CandidateIndexCountersDoNotReconcile,
    CandidateRowsMissingProofIdentity,
    SplitLineageIncomplete,
    DecisionLogNotQueryNative,
    DecisionRowsNotLocalized,
    PersistentNamingNotQueryNative,
    SplitLedgerNotCertified,
    ReplayParityNotCertified,
    DownstreamConsumptionNotCertified,
    LoopReconstructionConsumptionNotCertified,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitSummumBonumCloseoutDenial {
    kind: PlanarBooleanEdgeSplitSummumBonumCloseoutDenialKind,
    message: String,
}

impl PlanarBooleanEdgeSplitSummumBonumCloseoutDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitSummumBonumCloseoutDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitSummumBonumCloseoutDenialKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
