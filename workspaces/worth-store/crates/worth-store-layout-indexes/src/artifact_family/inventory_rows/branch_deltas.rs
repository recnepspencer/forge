use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const PLACEMENT_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::PlacementStableBasis,
    Authority::Derived,
    Lifecycle::OperationalSupport,
    Lane::HotPath,
    Owner::WorthStoreBranchDeltas,
    Rebuild::RebuildFromAuthoritativeState,
    Migration::StableNoMigration,
    NONE,
)];

pub(super) const SUPPORT_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::SupportCursor,
    Authority::Derived,
    Lifecycle::OperationalSupport,
    Lane::HotPath,
    Owner::WorthStoreBranchDeltas,
    Rebuild::PartialRebuildOnly,
    Migration::StableNoMigration,
    NONE,
)];

pub(super) const ARTIFACT_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::BranchDeltaArtifact,
    Authority::Authoritative,
    Lifecycle::CoreState,
    Lane::MaintenancePath,
    Owner::WorthStoreBranchDeltas,
    Rebuild::PartialRebuildOnly,
    Migration::RollbackCapable,
    NONE,
)];
