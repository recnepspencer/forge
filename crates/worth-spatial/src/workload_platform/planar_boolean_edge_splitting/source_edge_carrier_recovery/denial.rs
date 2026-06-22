#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind {
    ScopeLedgerIdentityMismatch,
    ScopeCarrierSetIdentityMismatch,
    MissingCarrierRows,
    MissingSourceFaceIdentity,
    MissingSourceLoopIdentity,
    MissingSourceEdgeIdentity,
    MissingCarrierIdentity,
    MissingLocalFrameIdentity,
    MissingProjectionStageIdentity,
    MissingPrecisionBasisIdentity,
    MissingEndpointSourceIdentity,
    MissingProjectedEndpointFactIdentity,
    UnknownPointEventCarrierReference,
    UnknownIntervalEventCarrierReference,
    UnknownGroupedCarrierReference,
    DuplicateCarrierIdentityWithConflictingSourceBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial {
    kind: PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial {
    pub(crate) fn new(
        kind: PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
