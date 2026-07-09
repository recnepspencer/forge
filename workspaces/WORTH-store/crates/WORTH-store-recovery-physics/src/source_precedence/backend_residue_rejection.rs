use super::RecoveryCandidateDiscoveryTrace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackendResidueKind {
    StalePageImage,
    OrphanedCheckpointManifest,
    BackendDirectoryResidue,
    InvalidCompactionProduct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendResidueRejection {
    kind: BackendResidueKind,
    trace: RecoveryCandidateDiscoveryTrace,
}

impl BackendResidueRejection {
    pub fn new(kind: BackendResidueKind, trace: RecoveryCandidateDiscoveryTrace) -> Self {
        Self { kind, trace }
    }

    pub const fn kind(&self) -> BackendResidueKind {
        self.kind
    }

    pub const fn trace(&self) -> &RecoveryCandidateDiscoveryTrace {
        &self.trace
    }
}
