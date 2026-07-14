mod accuracy_class;
mod admission;
mod authority;
mod authority_role;
mod classification;
mod declaration;
mod declaration_registry;
mod denial;
mod inventory;
use crate::artifact_family::artifact_family_inventory_rows;
mod lifecycle;
#[cfg(test)]
mod lifecycle_authority_tests;
#[cfg(test)]
mod scope_authority_tests;
mod scope_partition;
pub(crate) mod system_families;
#[cfg(test)]
mod tests;
mod witness;

pub use accuracy_class::{ArtifactDerivedAccuracyWitness, DerivedAccuracyClass};
#[cfg(test)]
pub(crate) use admission::require_exact_accuracy_claim;
pub(crate) use admission::{
    classify_family, declare_authority_role, declare_derived_accuracy_class,
    require_production_authority, require_scope_partition, require_strategy_lifecycle,
};
pub use authority::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, DurableArtifactProjectionClass,
};
pub use authority_role::{ArtifactAuthorityRoleWitness, AuthorityRole};
pub use classification::{
    ArtifactFamilyAuthorityDisposition, ArtifactFamilyClassification,
    ArtifactFamilyLifecycleDisposition,
};
pub use declaration::{PhysicalArtifactFamily, PhysicalArtifactFamilyDeclaration};
pub use declaration_registry::layout_declarations;
pub(crate) use declaration_registry::LayoutDeclarationsFacade;
pub use denial::{ArtifactFamilyDenial, ArtifactFamilyDenialKind};
pub use inventory::{
    ArtifactFamilyInventory, ArtifactFamilyInventoryRow, ExistingArtifactFamilySurface,
};
pub use lifecycle::{
    ArtifactFamilyLifecycleClass, DurableArtifactMigrationPosture, DurableArtifactRebuildPosture,
};
pub use scope_partition::{
    ArtifactKeyScopePartition, ArtifactScopePartitionWitness, ArtifactTenantScopePartition,
};
pub use witness::{
    ArtifactFamilyAuthorityWitness, ArtifactFamilyLifecycleAdmission, ArtifactFamilyStrategyLane,
};
