use forge_store_contracts::{PhysicalIntegrityReadinessDenial, PhysicalIntegrityReadinessDenialKind};

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

impl From<PhysicalIntegrityReadinessDenial> for IntegrityEntryDenial {
    fn from(denial: PhysicalIntegrityReadinessDenial) -> Self {
        Self::new(match denial.kind() {
            PhysicalIntegrityReadinessDenialKind::S2ReadinessNotSealed => {
                IntegrityEntryDenialKind::S2ReadinessNotSealed
            }
            PhysicalIntegrityReadinessDenialKind::MissingProtectedViewCapability => {
                IntegrityEntryDenialKind::MissingProtectedViewCapability
            }
            PhysicalIntegrityReadinessDenialKind::MissingVerifierResidentEnvelope => {
                IntegrityEntryDenialKind::MissingVerifierResidentEnvelope
            }
            PhysicalIntegrityReadinessDenialKind::MissingScrubAllocationEnvelope => {
                IntegrityEntryDenialKind::MissingScrubAllocationEnvelope
            }
            PhysicalIntegrityReadinessDenialKind::MissingInspectionLifetimeLaw => {
                IntegrityEntryDenialKind::MissingInspectionLifetimeLaw
            }
            PhysicalIntegrityReadinessDenialKind::MissingNoMaterializationWitness => {
                IntegrityEntryDenialKind::MissingNoMaterializationWitness
            }
            PhysicalIntegrityReadinessDenialKind::MissingCounterRecap => {
                IntegrityEntryDenialKind::MissingCounterRecap
            }
            PhysicalIntegrityReadinessDenialKind::MissingDenialBehavior => {
                IntegrityEntryDenialKind::MissingDenialBehavior
            }
            PhysicalIntegrityReadinessDenialKind::MissingPhysicalAuthorityRecap => {
                IntegrityEntryDenialKind::MissingPhysicalAuthorityRecap
            }
            PhysicalIntegrityReadinessDenialKind::PhysicalAuthorityRecapMismatch => {
                IntegrityEntryDenialKind::PhysicalAuthorityRecapMismatch
            }
            PhysicalIntegrityReadinessDenialKind::MissingBufferPoolAuthorityRecap => {
                IntegrityEntryDenialKind::MissingBufferPoolAuthorityRecap
            }
            PhysicalIntegrityReadinessDenialKind::LaterSequenceSemanticClaimed => {
                IntegrityEntryDenialKind::LaterSequenceSemanticClaimed
            }
        })
    }
}
