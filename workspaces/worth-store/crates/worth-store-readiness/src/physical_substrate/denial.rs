#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateReadinessDenial {
    kind: PhysicalSubstrateReadinessDenialKind,
}

impl PhysicalSubstrateReadinessDenial {
    pub(crate) const fn new(kind: PhysicalSubstrateReadinessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> PhysicalSubstrateReadinessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateReadinessDenialKind {
    WrongRoadmapScope,
    PhysicalSubstrateProofRejected,
    MissingPhysicalReferences,
    MissingHeaderDecodeWitnesses,
    MissingPayloadAdmissionWitnesses,
    MissingManifestLayoutEvidence,
    MissingNoMaterializationWitness,
    MissingCounterEvidence,
}
