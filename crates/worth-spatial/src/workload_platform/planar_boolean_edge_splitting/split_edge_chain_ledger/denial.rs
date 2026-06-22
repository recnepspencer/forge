use super::counters::PlanarBooleanSplitEdgeChainLedgerCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitEdgeChainLedgerDenialKind {
    EmptyQueryDeclarationIdentity,
    ForeignScheduleProduct,
    ForeignSplitArtifactProduct,
    ForeignValidationReceipt,
    ForeignPersistentNamingReceipt,
    ForeignDecisionLogReceipt,
    MissingFragmentValidationCoverage,
    MissingScheduleBinding,
    MissingDecisionLogReceipt,
    MissingPersistentNameBinding,
    MissingDecisionBinding,
    DuplicateLedgerChainIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedgerDenial {
    kind: PlanarBooleanSplitEdgeChainLedgerDenialKind,
    artifact_identity: String,
    counters: PlanarBooleanSplitEdgeChainLedgerCounters,
    message: String,
}

impl PlanarBooleanSplitEdgeChainLedgerDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitEdgeChainLedgerDenialKind,
        artifact_identity: impl Into<String>,
        counters: PlanarBooleanSplitEdgeChainLedgerCounters,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            artifact_identity: artifact_identity.into(),
            counters,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitEdgeChainLedgerDenialKind {
        self.kind
    }
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }
    pub fn counters(&self) -> PlanarBooleanSplitEdgeChainLedgerCounters {
        self.counters
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
