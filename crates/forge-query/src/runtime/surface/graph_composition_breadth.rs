use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionBreadth {
    component_count: usize,
    symbolic_entity_declaration_count: usize,
    symbolic_relation_declaration_count: usize,
    breadth_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphCompositionBreadth {
    pub(crate) fn new(
        component_count: usize,
        symbolic_entity_declaration_count: usize,
        symbolic_relation_declaration_count: usize,
    ) -> Self {
        let breadth_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-breadth",
                )
                .field_usize(ForgeQueryEvidenceTag::new("components"), component_count)
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_entities"),
                    symbolic_entity_declaration_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("symbolic_relations"),
                    symbolic_relation_declaration_count,
                )
                .seal();
        Self {
            component_count,
            symbolic_entity_declaration_count,
            symbolic_relation_declaration_count,
            breadth_digest,
        }
    }

    pub(crate) fn empty() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn component_count(&self) -> usize {
        self.component_count
    }

    pub fn symbolic_entity_declaration_count(&self) -> usize {
        self.symbolic_entity_declaration_count
    }

    pub fn symbolic_relation_declaration_count(&self) -> usize {
        self.symbolic_relation_declaration_count
    }

    pub fn breadth_digest(&self) -> &str {
        self.breadth_digest.as_str()
    }

    pub fn breadth_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.breadth_digest
    }
}
