#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2ReadinessDenial {
    kind: S2ReadinessDenialKind,
}

impl S2ReadinessDenial {
    pub(crate) const fn new(kind: S2ReadinessDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> S2ReadinessDenialKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S2ReadinessDenialKind {
    WrongRoadmapScope,
    S1PhysicalSubstrateProofRejected,
    MissingPhysicalReferences,
    MissingHeaderDecodeWitnesses,
    MissingPayloadAdmissionWitnesses,
    MissingManifestLayoutEvidence,
    MissingNoMaterializationWitness,
    MissingCounterEvidence,
}
