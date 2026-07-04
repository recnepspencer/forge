use crate::semantic::{UiDslSemanticArtifact, UiDslSourceProvenance};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDslLoweringReceipt {
    semantic_artifact: UiDslSemanticArtifact,
    semantic_input_digest: u64,
    source_artifact_generation: u64,
    source_provenance: UiDslSourceProvenance,
}

impl UiDslLoweringReceipt {
    pub(crate) fn new(
        semantic_artifact: UiDslSemanticArtifact,
        semantic_input_digest: u64,
        source_artifact_generation: u64,
        source_provenance: UiDslSourceProvenance,
    ) -> Self {
        Self {
            semantic_artifact,
            semantic_input_digest,
            source_artifact_generation,
            source_provenance,
        }
    }

    pub fn semantic_artifact(&self) -> &UiDslSemanticArtifact {
        &self.semantic_artifact
    }

    pub fn semantic_input_digest(&self) -> u64 {
        self.semantic_input_digest
    }

    pub fn source_artifact_generation(&self) -> u64 {
        self.source_artifact_generation
    }

    pub fn source_provenance(&self) -> &UiDslSourceProvenance {
        &self.source_provenance
    }
}
