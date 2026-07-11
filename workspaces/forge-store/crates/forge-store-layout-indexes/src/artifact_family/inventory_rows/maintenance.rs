use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, EVIDENCE_ONLY, OFFLINE_ONLY};
use forge_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const RECLAIM_EVIDENCE_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::ReclaimReceipt,
    Authority::Diagnostic,
    Lifecycle::OperationalSupport,
    Lane::MaintenancePath,
    Owner::ForgeStoreMaintenance,
    Rebuild::NoRebuild,
    Migration::StableNoMigration,
    EVIDENCE_ONLY,
)];

pub(super) const SUPPORT_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::MaintenanceSnapshot,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreMaintenance,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::MaintenanceCompaction,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreMaintenance,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::MaintenanceCapsule,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreMaintenance,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::MaintenanceQueueDeclaration,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::ForgeStoreMaintenance,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];
