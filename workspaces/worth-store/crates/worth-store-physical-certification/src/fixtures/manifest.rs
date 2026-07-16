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

pub(crate) struct ReopenedFixtureManifestParts {
    pub(crate) name: String,
    pub(crate) profile: LargeStoreFixtureProfile,
    pub(crate) scale: FixtureScaleDeclaration,
    pub(crate) source: ProductionBackedFixtureSource,
    pub(crate) semantic_digest: String,
    pub(crate) artifact_catalog: PhysicalArtifactFixtureCatalog,
    pub(crate) capability_declarations: Vec<FixtureCapabilityDeclaration>,
    pub(crate) mutation_boundaries: FixtureMutationBoundarySet,
}

impl PersistedStoreFixtureManifest {
    pub(crate) fn from_reopened_fixture(parts: ReopenedFixtureManifestParts) -> Self {
        Self {
            name: parts.name,
            profile: parts.profile,
            scale: parts.scale,
            source: parts.source,
            semantic_digest: parts.semantic_digest,
            artifact_catalog: parts.artifact_catalog,
            capability_declarations: parts.capability_declarations,
            mutation_boundaries: parts.mutation_boundaries,
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
