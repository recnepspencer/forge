use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE};
use forge_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::SnapshotArtifact,
    Authority::Authoritative,
    Lifecycle::CoreState,
    Lane::MaintenancePath,
    Owner::ForgeStoreSnapshots,
    Rebuild::PartialRebuildOnly,
    Migration::RollbackCapable,
    NONE,
)];
