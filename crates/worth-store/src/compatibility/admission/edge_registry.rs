use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CompatibilityEdgeRegistry {
    edges: BTreeMap<EdgeKey, DeclaredCompatibilityEdge>,
}
impl CompatibilityEdgeRegistry {
    pub fn new(edges: Vec<DeclaredCompatibilityEdge>) -> Self {
        let mut registry = Self::default();
        for edge in edges {
            registry.declare(edge);
        }
        registry
    }

    pub fn declare(&mut self, edge: DeclaredCompatibilityEdge) {
        self.edges.insert(EdgeKey::from_edge(&edge), edge);
    }

    pub fn get(
        &self,
        family_id: &ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
    ) -> Option<&DeclaredCompatibilityEdge> {
        self.edges.get(&EdgeKey::new(
            family_id.clone(),
            from_semantic_version,
            to_semantic_version,
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct EdgeKey {
    family_id: ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
}

impl EdgeKey {
    fn new(
        family_id: ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            from_semantic_version,
            to_semantic_version,
        }
    }

    fn from_edge(edge: &DeclaredCompatibilityEdge) -> Self {
        Self::new(
            edge.family_id().clone(),
            edge.from_semantic_version(),
            edge.to_semantic_version(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityEdgeProof {
    edge: DeclaredCompatibilityEdge,
}

impl CompatibilityEdgeProof {
    pub(crate) fn new(edge: DeclaredCompatibilityEdge) -> Self {
        Self { edge }
    }
}
