use forge_foundational::{CanonicalBasisEntry, CanonicalBasisReadyArtifact};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalMergeProofPacketCanonicalBasis {
    artifact: CanonicalBasisReadyArtifact,
}

impl RelationalMergeProofPacketCanonicalBasis {
    pub(crate) fn new(artifact: CanonicalBasisReadyArtifact) -> Self {
        Self { artifact }
    }

    pub fn artifact(&self) -> &CanonicalBasisReadyArtifact {
        &self.artifact
    }

    pub fn entries(&self) -> &[CanonicalBasisEntry] {
        self.artifact.payload().entries()
    }
}
