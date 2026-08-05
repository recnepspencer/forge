use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterId(String);

impl CompatibilityAdapterId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterDigest(String);

impl CompatibilityAdapterDigest {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredCompatibilityAdapter {
    adapter_id: CompatibilityAdapterId,
    adapter_digest: CompatibilityAdapterDigest,
    cost_class: CompatibilityAdapterCostClass,
}

impl DeclaredCompatibilityAdapter {
    pub fn new(
        adapter_id: CompatibilityAdapterId,
        adapter_digest: CompatibilityAdapterDigest,
        cost_class: CompatibilityAdapterCostClass,
    ) -> Self {
        Self {
            adapter_id,
            adapter_digest,
            cost_class,
        }
    }

    pub fn cost_class(&self) -> CompatibilityAdapterCostClass {
        self.cost_class
    }

    pub fn adapter_id(&self) -> &CompatibilityAdapterId {
        &self.adapter_id
    }

    pub fn adapter_digest(&self) -> &CompatibilityAdapterDigest {
        &self.adapter_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredCompatibilityEdge {
    family_id: ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
    adapter: Option<DeclaredCompatibilityAdapter>,
}

impl DeclaredCompatibilityEdge {
    pub fn new(
        family_id: ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
        relation: CompatibilityRelation,
    ) -> Self {
        Self {
            family_id,
            from_semantic_version,
            to_semantic_version,
            relation,
            adapter: None,
        }
    }

    pub fn with_adapter(mut self, adapter: DeclaredCompatibilityAdapter) -> Self {
        self.adapter = Some(adapter);
        self
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn adapter(&self) -> Option<&DeclaredCompatibilityAdapter> {
        self.adapter.as_ref()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn from_semantic_version(&self) -> ArtifactSemanticVersion {
        self.from_semantic_version
    }

    pub fn to_semantic_version(&self) -> ArtifactSemanticVersion {
        self.to_semantic_version
    }
}
