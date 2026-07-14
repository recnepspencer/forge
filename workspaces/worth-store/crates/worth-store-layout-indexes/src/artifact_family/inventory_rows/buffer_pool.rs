use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, OFFLINE_ONLY};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::ResidencyRecord,
    Authority::Diagnostic,
    Lifecycle::OperationalSupport,
    Lane::MaintenancePath,
    Owner::WorthStoreBufferPool,
    Rebuild::NoRebuild,
    Migration::StableNoMigration,
    OFFLINE_ONLY,
)];
