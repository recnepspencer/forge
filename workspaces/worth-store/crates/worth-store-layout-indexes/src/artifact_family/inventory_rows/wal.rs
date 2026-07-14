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
        Family::WalDurableMutationIntent,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStoreWal,
        Rebuild::ReplayRebuildable,
        Migration::VersionedReadmission,
        NONE,
    ),
    row(
        Family::WalHostedRuntimeCommitResult,
        Authority::Authoritative,
        Lifecycle::CoreState,
        Lane::HotPath,
        Owner::WorthStoreWal,
        Rebuild::ReplayRebuildable,
        Migration::VersionedReadmission,
        NONE,
    ),
];

pub(super) const RECOVERY_STATE_ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        Family::WalBulkCheckpointPublicationIntent,
        Authority::Authoritative,
        Lifecycle::RecoveryState,
        Lane::MaintenancePath,
        Owner::WorthStoreWal,
        Rebuild::ReplayRebuildable,
        Migration::VersionedReadmission,
        NONE,
    ),
    row(
        Family::WalDurablePublicationProgress,
        Authority::Authoritative,
        Lifecycle::RecoveryState,
        Lane::MaintenancePath,
        Owner::WorthStoreWal,
        Rebuild::ReplayRebuildable,
        Migration::VersionedReadmission,
        NONE,
    ),
];
