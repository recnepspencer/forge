use worth_ui_dsl::UiDslSourceProvenance;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationProvenance {
    source_provenance: UiDslSourceProvenance,
    semantic_input_digest: u64,
    source_artifact_generation: u64,
}

impl UiDeclarationProvenance {
    pub(crate) fn new(
        source_provenance: UiDslSourceProvenance,
        semantic_input_digest: u64,
        source_artifact_generation: u64,
    ) -> Self {
        Self {
            source_provenance,
            semantic_input_digest,
            source_artifact_generation,
        }
    }

    pub fn source_provenance(&self) -> &UiDslSourceProvenance {
        &self.source_provenance
    }

    pub fn semantic_input_digest(&self) -> u64 {
        self.semantic_input_digest
    }

    pub fn inspection_source_generation(&self) -> worth_ui_inspection::UiSourceArtifactGeneration {
        worth_ui_inspection::UiSourceArtifactGeneration::new(self.source_artifact_generation)
    }

    pub fn inspection_authored_source_provenance_ref(
        &self,
    ) -> worth_ui_inspection::UiAuthoredSourceProvenanceRef {
        worth_ui_inspection::UiAuthoredSourceProvenanceRef::file_declaration(
            worth_ui_inspection::UiSourceArtifactIdentity::dsl_module(
                self.source_provenance.module_path(),
            ),
            self.inspection_source_generation(),
            self.source_provenance.declaration_index(),
        )
    }
}
