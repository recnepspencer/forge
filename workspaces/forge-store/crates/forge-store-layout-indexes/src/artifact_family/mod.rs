mod accuracy_class;
mod admission;
mod authority;
mod authority_role;
mod classification;
mod declaration;
mod denial;
mod inventory;
mod inventory_rows;
mod lifecycle;
#[cfg(test)]
mod phase_three_tests;
#[cfg(test)]
mod phase_two_tests;
mod scope_partition;
#[cfg(test)]
mod tests;
mod witness;

pub use accuracy_class::{ArtifactDerivedAccuracyWitness, DerivedAccuracyClass};
pub(crate) use admission::{
    classify_family, declare_authority_role, declare_derived_accuracy_class,
    require_exact_accuracy_claim, require_production_authority, require_scope_partition,
    require_strategy_lifecycle,
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
pub use denial::ArtifactFamilyDenial;
pub use inventory::{
    ArtifactFamilyInventoryRow, ExistingArtifactFamilySurface, S8ArtifactFamilyInventory,
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
