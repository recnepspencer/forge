use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionBreadth {
    component_count: usize,
    symbolic_entity_declaration_count: usize,
    symbolic_relation_declaration_count: usize,
    breadth_digest: String,
}

impl ForgeQueryGraphCompositionBreadth {
    pub(crate) fn new(
        component_count: usize,
        symbolic_entity_declaration_count: usize,
        symbolic_relation_declaration_count: usize,
    ) -> Self {
        let breadth_digest = hash_parts(&[
            "forge_query_graph_composition_breadth_v1".to_string(),
            format!("components:{component_count}"),
            format!("entities:{symbolic_entity_declaration_count}"),
            format!("relations:{symbolic_relation_declaration_count}"),
        ]);
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
        &self.breadth_digest
    }
}
