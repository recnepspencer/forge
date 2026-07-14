use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const CORE_STATE_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::PhysicalPage,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStorePhysicalFormat,
        Rebuild::NoRebuild,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::PhysicalSegment,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStorePhysicalFormat,
        Rebuild::NoRebuild,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::PhysicalExtent,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStorePhysicalFormat,
        Rebuild::NoRebuild,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::PhysicalRootManifest,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStorePhysicalFormat,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::VersionedMigration,
        NONE,
    ),
];
