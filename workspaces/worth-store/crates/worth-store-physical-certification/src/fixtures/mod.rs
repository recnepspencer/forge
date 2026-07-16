mod artifact_catalog;
mod authority;
mod builder;
mod capability;
mod denial;
mod manifest;
mod materialization;
mod mutation_boundary;
mod production_backed;
mod profile;

pub use artifact_catalog::PhysicalArtifactFixtureCatalog;
pub use authority::{
    FixtureAuthorityReceipt, FixtureConstructionAuthority, FixtureConstructionBasis,
    FixtureConstructionProofBasis, FixtureProvenance, ResolvedFixtureConstructionRecipe,
    StoreFixtureAuthority,
};
pub use builder::{FixtureNeedsBoundary, FixtureNeedsMaterialization, PhysicalFixtureBuilder};
pub use capability::FixtureCapabilityDeclaration;
pub use denial::SyntheticFixtureAuthorityDenied;
pub use manifest::PersistedStoreFixtureManifest;
pub(crate) use manifest::ReopenedFixtureManifestParts;
pub use materialization::{ProductionBackedFixtureMaterialization, ProductionBackedFixtureSource};
pub use mutation_boundary::{FixtureMutationBoundary, FixtureMutationBoundarySet};
pub use production_backed::ProductionBackedPhysicalFixture;
pub use profile::{
    FixtureActivityScale, FixtureProfileNonClaim, FixtureScaleDeclaration, FixtureStorageScale,
    LargeStoreFixtureProfile,
};
