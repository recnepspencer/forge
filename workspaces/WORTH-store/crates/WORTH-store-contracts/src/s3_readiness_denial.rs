#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S3ReadinessDenial {
    kind: S3ReadinessDenialKind,
}

impl S3ReadinessDenial {
    pub const fn new(kind: S3ReadinessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> S3ReadinessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S3ReadinessDenialKind {
    S2ReadinessNotSealed,
    MissingProtectedViewCapability,
    MissingVerifierResidentEnvelope,
    MissingScrubAllocationEnvelope,
    MissingInspectionLifetimeLaw,
    MissingNoMaterializationWitness,
    MissingCounterRecap,
    MissingDenialBehavior,
    MissingPhysicalAuthorityRecap,
    PhysicalAuthorityRecapMismatch,
    MissingBufferPoolAuthorityRecap,
    LaterSequenceSemanticClaimed,
}
