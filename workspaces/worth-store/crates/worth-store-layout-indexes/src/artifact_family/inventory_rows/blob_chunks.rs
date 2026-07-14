use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE, OFFLINE_ONLY};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::BlobChunk,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStoreBlobChunks,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::BlobManifest,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStoreBlobChunks,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::BlobStream,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStoreBlobChunks,
        Rebuild::PartialRebuildOnly,
        Migration::VersionedMigration,
        NONE,
    ),
    row(
        Family::ChunkTreeRoot,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreBlobChunks,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::DedupeIndex,
        Authority::Derived,
        Lifecycle::DerivedState,
        Lane::MaintenancePath,
        Owner::WorthStoreBlobChunks,
        Rebuild::RebuildFromAuthoritativeState,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];
