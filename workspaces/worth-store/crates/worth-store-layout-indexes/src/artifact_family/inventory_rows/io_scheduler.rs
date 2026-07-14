use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, EVIDENCE_ONLY, NONE};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const RESERVATION_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::SchedulerReservationIndex,
    Authority::Derived,
    Lifecycle::OperationalSupport,
    Lane::HotPath,
    Owner::WorthStoreIoScheduler,
    Rebuild::RebuildFromAuthoritativeState,
    Migration::StableNoMigration,
    NONE,
)];

pub(super) const EVIDENCE_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::BackgroundPacingRecord,
        Authority::Diagnostic,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::WorthStoreIoScheduler,
        Rebuild::NoRebuild,
        Migration::StableNoMigration,
        EVIDENCE_ONLY,
    ),
    row(
        Family::ForegroundInterferenceRecord,
        Authority::Diagnostic,
        Lifecycle::OperationalSupport,
        Lane::HotPath,
        Owner::WorthStoreIoScheduler,
        Rebuild::NoRebuild,
        Migration::StableNoMigration,
        EVIDENCE_ONLY,
    ),
];
