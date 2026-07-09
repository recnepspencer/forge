#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S4IntegrityHandoffDenialKind {
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
pub struct S4IntegrityHandoffDenial {
    kind: S4IntegrityHandoffDenialKind,
}

impl S4IntegrityHandoffDenial {
    pub const fn new(kind: S4IntegrityHandoffDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> S4IntegrityHandoffDenialKind {
        self.kind
    }
}
