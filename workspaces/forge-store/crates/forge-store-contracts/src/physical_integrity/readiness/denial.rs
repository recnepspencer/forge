#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalIntegrityReadinessDenial {
    kind: PhysicalIntegrityReadinessDenialKind,
}

impl PhysicalIntegrityReadinessDenial {
    pub const fn new(kind: PhysicalIntegrityReadinessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> PhysicalIntegrityReadinessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIntegrityReadinessDenialKind {
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
