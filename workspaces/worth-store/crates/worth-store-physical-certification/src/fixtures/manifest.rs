use super::{
    FixtureCapabilityDeclaration, FixtureMutationBoundarySet, FixtureScaleDeclaration,
    LargeStoreFixtureProfile, PhysicalArtifactFixtureCatalog, ProductionBackedFixtureSource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedStoreFixtureManifest {
    name: String,
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    semantic_digest: String,
    artifact_catalog: PhysicalArtifactFixtureCatalog,
    capability_declarations: Vec<FixtureCapabilityDeclaration>,
    mutation_boundaries: FixtureMutationBoundarySet,
}

impl PersistedStoreFixtureManifest {
    pub(crate) fn from_reopened_fixture(
        name: String,
        profile: LargeStoreFixtureProfile,
        scale: FixtureScaleDeclaration,
        source: ProductionBackedFixtureSource,
        semantic_digest: String,
        artifact_catalog: PhysicalArtifactFixtureCatalog,
        capability_declarations: Vec<FixtureCapabilityDeclaration>,
        mutation_boundaries: FixtureMutationBoundarySet,
    ) -> Self {
        Self {
            name,
            profile,
            scale,
            source,
            semantic_digest,
            artifact_catalog,
            capability_declarations,
            mutation_boundaries,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn profile(&self) -> LargeStoreFixtureProfile {
        self.profile
    }

    pub const fn scale(&self) -> FixtureScaleDeclaration {
        self.scale
    }

    pub const fn source(&self) -> ProductionBackedFixtureSource {
        self.source
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }

    pub const fn artifact_catalog(&self) -> &PhysicalArtifactFixtureCatalog {
        &self.artifact_catalog
    }

    pub fn capability_declarations(&self) -> &[FixtureCapabilityDeclaration] {
        &self.capability_declarations
    }

    pub const fn mutation_boundaries(&self) -> &FixtureMutationBoundarySet {
        &self.mutation_boundaries
    }
}
