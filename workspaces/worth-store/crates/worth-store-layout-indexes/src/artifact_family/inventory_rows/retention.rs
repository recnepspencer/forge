use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE, OFFLINE_ONLY};
use worth_store_contracts::{
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
        Owner::WorthStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::RetentionHold,
        Authority::Authoritative,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::QuarantineOnly,
        Migration::VersionedMigration,
        NONE,
    ),
];

pub(super) const DERIVED_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::DerivedRetentionLegacyLayoutMaterialization,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionLegacyScopeSliceMembership,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionLegacyStructuralBlock,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DerivedRetentionLegacyChunkMembership,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::LayoutCompactionUnit,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreRetention,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];
