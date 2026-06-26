use crate::source::{WorthUiArtifactDigest, WorthUiArtifactEquivalenceBasis};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementCandidateBasis {
    artifact_digest: WorthUiArtifactDigest,
    artifact_equivalence_basis: WorthUiArtifactEquivalenceBasis,
    dependency_metadata_digest: u64,
    lowering_basis_digest: u64,
}

impl WorthUiReplacementCandidateBasis {
    pub(crate) fn new(
        artifact_digest: WorthUiArtifactDigest,
        dependency_metadata_digest: u64,
        lowering_basis_digest: u64,
    ) -> Self {
        Self {
            artifact_digest,
            artifact_equivalence_basis: artifact_digest.basis(),
            dependency_metadata_digest,
            lowering_basis_digest,
        }
    }

    pub(crate) fn artifact_digest(self) -> WorthUiArtifactDigest {
        self.artifact_digest
    }

    pub(crate) fn artifact_equivalence_basis(self) -> WorthUiArtifactEquivalenceBasis {
        self.artifact_equivalence_basis
    }

    pub(crate) fn dependency_metadata_digest(self) -> u64 {
        self.dependency_metadata_digest
    }

    pub fn lowering_basis_digest(self) -> u64 {
        self.lowering_basis_digest
    }
}
