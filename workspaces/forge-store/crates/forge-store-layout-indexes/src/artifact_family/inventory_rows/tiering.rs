use super::super::ArtifactFamilyInventoryRow;
use super::row::row;
use super::row::OFFLINE_ONLY;
use forge_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const PLACEMENT_AUTHORITY_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::PlacementAuthoritativeBranchHead,
        Authority::Authoritative,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::PlacementRetainedAuthority,
        Authority::Authoritative,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];

pub(super) const PLACEMENT_PROJECTION_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::PlacementSnapshotFamily,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::PlacementBranchDeltaFamily,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::PlacementMilestone6LayoutFamily,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];

pub(super) const TIER_OPERATION_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::TierPlacementManifest,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::ColdRecallQueue,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::RecallAmplificationIndex,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreTiering,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];
