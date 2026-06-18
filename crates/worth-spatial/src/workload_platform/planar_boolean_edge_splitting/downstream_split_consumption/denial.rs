use super::counters::PlanarBooleanDownstreamSplitConsumptionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanDownstreamSplitConsumptionDenialKind {
    MissingSplitLedgerReceipt,
    MissingDecisionLogReceipt,
    MissingPersistentNamingReceipt,
    MissingValidationReceipt,
    MissingReplayParityReceipt,
    MissingWorkloadStageIndex,
    ForeignDecisionLogReceipt,
    ForeignPersistentNamingReceipt,
    ForeignValidationReceipt,
    ForeignReplayParityReceipt,
    ForeignWorkloadStageIndex,
    NonReceiptBackedBooleanSplitEvidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDownstreamSplitConsumptionDenial {
    kind: PlanarBooleanDownstreamSplitConsumptionDenialKind,
    rejected_identity: String,
    expected_identity: String,
    observed_identity: String,
    counters: PlanarBooleanDownstreamSplitConsumptionCounters,
    human_reason: &'static str,
}

impl PlanarBooleanDownstreamSplitConsumptionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanDownstreamSplitConsumptionDenialKind,
        rejected_identity: impl Into<String>,
        expected_identity: impl Into<String>,
        observed_identity: impl Into<String>,
        counters: PlanarBooleanDownstreamSplitConsumptionCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            expected_identity: expected_identity.into(),
            observed_identity: observed_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanDownstreamSplitConsumptionDenialKind {
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

    pub fn counters(&self) -> PlanarBooleanDownstreamSplitConsumptionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
