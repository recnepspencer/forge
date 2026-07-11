use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE, OFFLINE_ONLY};
use forge_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const REACHABILITY_AND_HOLD_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::ReachabilityEdge,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::RetentionHold,
        Authority::Authoritative,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::QuarantineOnly,
        Migration::VersionedMigration,
        NONE,
    ),
];

pub(super) const DERIVED_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::DerivedRetentionMilestone6LayoutMaterialization,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionMilestone6ScopeSliceMembership,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionMilestone6StructuralBlock,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionMilestone6ChunkMembership,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::LayoutCompactionUnit,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::ForgeStoreRetention,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];
