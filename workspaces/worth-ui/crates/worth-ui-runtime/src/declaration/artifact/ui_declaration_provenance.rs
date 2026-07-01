use worth_ui_dsl::UiDslSourceProvenance;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationProvenance {
    source_provenance: UiDslSourceProvenance,
    semantic_input_digest: u64,
}

impl UiDeclarationProvenance {
    pub(crate) fn new(
        source_provenance: UiDslSourceProvenance,
        semantic_input_digest: u64,
    ) -> Self {
        Self {
            source_provenance,
            semantic_input_digest,
        }
    }

    pub fn source_provenance(&self) -> &UiDslSourceProvenance {
        &self.source_provenance
    }

    pub fn semantic_input_digest(&self) -> u64 {
        self.semantic_input_digest
    }
}
