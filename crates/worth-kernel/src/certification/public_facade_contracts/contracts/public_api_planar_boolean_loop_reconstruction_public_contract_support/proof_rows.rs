#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanLoopPublicContractProofRow {
    kind: PlanarBooleanLoopPublicContractProofRowKind,
    identity: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopPublicContractProofRowKind {
    LoopLedgerReceipt,
    LoopEvidenceReceipt,
    RuntimeRegistrationProof,
    WorkloadStageIndex,
    DownstreamLoopConsumption,
    AntiTheatreGuard,
    AntiTheatreFence,
}

impl PlanarBooleanLoopPublicContractProofRow {
    pub(crate) fn new(kind: PlanarBooleanLoopPublicContractProofRowKind, identity: String) -> Self {
        Self { kind, identity }
    }

    pub(crate) fn kind(&self) -> PlanarBooleanLoopPublicContractProofRowKind {
        self.kind
    }

    pub(crate) fn identity(&self) -> &str {
        &self.identity
    }
}
