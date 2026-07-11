use forge_store_contracts::{S3ReadinessDenial, S3ReadinessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryDenial {
    kind: IntegrityEntryDenialKind,
}

impl IntegrityEntryDenial {
    pub const fn new(kind: IntegrityEntryDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(self) -> IntegrityEntryDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEntryDenialKind {
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
    MissingProtectedPhysicalByteView,
}

impl From<S3ReadinessDenial> for IntegrityEntryDenial {
    fn from(denial: S3ReadinessDenial) -> Self {
        Self::new(match denial.kind() {
            S3ReadinessDenialKind::S2ReadinessNotSealed => {
                IntegrityEntryDenialKind::S2ReadinessNotSealed
            }
            S3ReadinessDenialKind::MissingProtectedViewCapability => {
                IntegrityEntryDenialKind::MissingProtectedViewCapability
            }
            S3ReadinessDenialKind::MissingVerifierResidentEnvelope => {
                IntegrityEntryDenialKind::MissingVerifierResidentEnvelope
            }
            S3ReadinessDenialKind::MissingScrubAllocationEnvelope => {
                IntegrityEntryDenialKind::MissingScrubAllocationEnvelope
            }
            S3ReadinessDenialKind::MissingInspectionLifetimeLaw => {
                IntegrityEntryDenialKind::MissingInspectionLifetimeLaw
            }
            S3ReadinessDenialKind::MissingNoMaterializationWitness => {
                IntegrityEntryDenialKind::MissingNoMaterializationWitness
            }
            S3ReadinessDenialKind::MissingCounterRecap => {
                IntegrityEntryDenialKind::MissingCounterRecap
            }
            S3ReadinessDenialKind::MissingDenialBehavior => {
                IntegrityEntryDenialKind::MissingDenialBehavior
            }
            S3ReadinessDenialKind::MissingPhysicalAuthorityRecap => {
                IntegrityEntryDenialKind::MissingPhysicalAuthorityRecap
            }
            S3ReadinessDenialKind::PhysicalAuthorityRecapMismatch => {
                IntegrityEntryDenialKind::PhysicalAuthorityRecapMismatch
            }
            S3ReadinessDenialKind::MissingBufferPoolAuthorityRecap => {
                IntegrityEntryDenialKind::MissingBufferPoolAuthorityRecap
            }
            S3ReadinessDenialKind::LaterSequenceSemanticClaimed => {
                IntegrityEntryDenialKind::LaterSequenceSemanticClaimed
            }
        })
    }
}
