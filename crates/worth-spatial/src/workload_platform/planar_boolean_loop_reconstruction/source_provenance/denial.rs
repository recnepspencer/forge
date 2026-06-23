use super::counters::PlanarBooleanLoopSourceProvenanceCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopSourceProvenanceDenialKind {
    ForeignSplitLedgerReceipt,
    ForeignSplitRequestLineage,
    ForeignSourceCarrierLineage,
    ForeignFragmentLineage,
    ForeignOverlapChainLineage,
    MissingLedgerChainCarrier,
    MissingRecoveredSourceCarrier,
    MissingLedgerFragment,
    MissingLedgerOverlapChain,
    OverlapChainMemberMissingFragmentMembership,
    DuplicateFragmentIdentity,
    DuplicateOverlapChainIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSourceProvenanceDenial {
    kind: PlanarBooleanLoopSourceProvenanceDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanLoopSourceProvenanceCounters,
    human_reason: &'static str,
}

impl PlanarBooleanLoopSourceProvenanceDenial {
    pub(crate) fn new(
        kind: PlanarBooleanLoopSourceProvenanceDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanLoopSourceProvenanceCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopSourceProvenanceDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanLoopSourceProvenanceCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
