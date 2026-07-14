use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, NONE};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::SecurityCustodyLookup,
    Authority::Authoritative,
    Lifecycle::OperationalSupport,
    Lane::HotPath,
    Owner::WorthStoreSecurity,
    Rebuild::QuarantineOnly,
    Migration::VersionedMigration,
    NONE,
)];
