#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityHandoffDenialKind {
    EvidenceIsNotAuthoritativeCurrent,
    EvidenceIsNotReceiptEvidence,
    EvidenceIsNotIntactPhysicalBoundary,
    ReceiptScopeMismatch,
    ReceiptCounterMismatch,
    ReceiptBasisMismatch,
    InspectionEnvelopeExceeded,
    ChecksumBasisMismatch,
    DamageMapSourceMismatch,
    UnresolvedAuthorityDamageRequiresAuthorityClassification,
    DamagedInputRequiresBlockingEvidence,
    MissingRootManifestRecord,
    MissingSegmentManifestRecord,
    MissingPageFrameRecord,
    MissingWalFrame,
    MissingCheckpointRecord,
    MissingInspectionEnvelopeEvidence,
    MissingS3ProtectedViewCapability,
    MissingS3InspectionLifetimeLaw,
    MissingS3NoMaterializationWitness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityHandoffDenial {
    kind: IntegrityHandoffDenialKind,
}

impl IntegrityHandoffDenial {
    pub const fn new(kind: IntegrityHandoffDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> IntegrityHandoffDenialKind {
        self.kind
    }
}
