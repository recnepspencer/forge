use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, EVIDENCE_ONLY, NONE, OFFLINE_ONLY};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const WAL_RECOVERY_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::WalRecoveryDecision,
    Authority::Authoritative,
    Lifecycle::RecoveryState,
    Lane::VerifierPath,
    Owner::WorthStoreRecoveryPhysics,
    Rebuild::ReplayRebuildable,
    Migration::VersionedReadmission,
    NONE,
)];

pub(super) const QUARANTINE_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::QuarantineRecord,
    Authority::Diagnostic,
    Lifecycle::RecoveryState,
    Lane::VerifierPath,
    Owner::WorthStoreRecoveryPhysics,
    Rebuild::QuarantineOnly,
    Migration::VersionedReadmission,
    EVIDENCE_ONLY,
)];

pub(super) const READMISSION_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::ReadmissionRecord,
    Authority::Authoritative,
    Lifecycle::RecoveryState,
    Lane::VerifierPath,
    Owner::WorthStoreRecoveryPhysics,
    Rebuild::QuarantineOnly,
    Migration::VersionedReadmission,
    NONE,
)];

pub(super) const SUPPORT_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::SupportSchema,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::WorthStoreRecoveryPhysics,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        Family::SupportLineage,
        Authority::Derived,
        Lifecycle::OperationalSupport,
        Lane::MaintenancePath,
        Owner::WorthStoreRecoveryPhysics,
        Rebuild::PartialRebuildOnly,
        Migration::StableNoMigration,
        OFFLINE_ONLY,
    ),
];

pub(super) const CHECKPOINT_SUPPORT_ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::SupportEmbeddedCheckpoint,
    Authority::Derived,
    Lifecycle::OperationalSupport,
    Lane::MaintenancePath,
    Owner::WorthStoreRecoveryPhysics,
    Rebuild::PartialRebuildOnly,
    Migration::StableNoMigration,
    OFFLINE_ONLY,
)];
