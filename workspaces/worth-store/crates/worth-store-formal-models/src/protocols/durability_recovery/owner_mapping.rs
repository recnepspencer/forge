use worth_store_physical_backend::{
    BackendDurabilityProfile, PosixFileFsyncDirFsyncProfile, WindowsFlushFileBuffersProfile,
};
use worth_store_recovery_physics::{
    CheckpointBaseAdmission, CheckpointCutoverReceipt, RecoveryCompletion, RedoExecutionReceipt,
    RedoPlanningDenial, RedoPlanningDenialKind, ReopenedRecoveryArtifactAdmission,
    WalDurabilityObservationDenial, WalDurabilityObservationDenialKind,
};

use super::DurabilityRecoveryAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityOwnerMappingDenial {
    CheckpointProfileDoesNotProveDirectorySync,
    WalFenceFailureRequired,
    ReplayGenerationMismatchRequired,
}

pub fn map_checkpoint_cutover(
    receipt: &CheckpointCutoverReceipt,
) -> Result<[DurabilityRecoveryAction; 4], DurabilityOwnerMappingDenial> {
    if receipt.profile_id() != PosixFileFsyncDirFsyncProfile::ID
        && receipt.profile_id() != WindowsFlushFileBuffersProfile::ID
    {
        return Err(DurabilityOwnerMappingDenial::CheckpointProfileDoesNotProveDirectorySync);
    }
    Ok([
        DurabilityRecoveryAction::CheckpointBegun,
        DurabilityRecoveryAction::CheckpointDurable,
        DurabilityRecoveryAction::DirectorySyncCompleted,
        DurabilityRecoveryAction::CheckpointPublished,
    ])
}

pub const fn map_checkpoint_selection(
    _selection: &CheckpointBaseAdmission,
) -> DurabilityRecoveryAction {
    DurabilityRecoveryAction::CheckpointSelected
}

pub fn map_failed_wal_fence(
    denial: &WalDurabilityObservationDenial,
) -> Result<[DurabilityRecoveryAction; 3], DurabilityOwnerMappingDenial> {
    if denial.kind() != WalDurabilityObservationDenialKind::BarrierFailed {
        return Err(DurabilityOwnerMappingDenial::WalFenceFailureRequired);
    }
    Ok([
        DurabilityRecoveryAction::WalAppendProposed,
        DurabilityRecoveryAction::WalAppendCompletedInMemory,
        DurabilityRecoveryAction::WalFenceRequested,
    ])
}

pub fn map_redo_execution(receipt: &RedoExecutionReceipt) -> Vec<DurabilityRecoveryAction> {
    let mut actions = vec![DurabilityRecoveryAction::RecoveryReplayRequired];
    if receipt.applied_frame_count() > 0 {
        actions.push(DurabilityRecoveryAction::RecoveryReplayApplied);
    }
    if !receipt.skipped_frames().is_empty() {
        actions.push(DurabilityRecoveryAction::RecoveryReplaySkippedIdempotent);
    }
    actions
}

pub fn map_redo_generation_denial(
    denial: &RedoPlanningDenial,
) -> Result<DurabilityRecoveryAction, DurabilityOwnerMappingDenial> {
    if matches!(
        denial.kind(),
        RedoPlanningDenialKind::RedoTargetPageGenerationMismatch { .. }
            | RedoPlanningDenialKind::CursorPageGenerationMismatch { .. }
    ) {
        Ok(DurabilityRecoveryAction::RecoveryReplayRejectedGenerationMismatch)
    } else {
        Err(DurabilityOwnerMappingDenial::ReplayGenerationMismatchRequired)
    }
}

pub const fn map_recovery_completion(
    _completion: &RecoveryCompletion,
) -> [DurabilityRecoveryAction; 2] {
    [
        DurabilityRecoveryAction::RecoveredRootPublicationPending,
        DurabilityRecoveryAction::RecoveredRootPublicationCompleted,
    ]
}

pub const fn map_reopened_recovery_artifact(
    _reopened: &ReopenedRecoveryArtifactAdmission,
) -> DurabilityRecoveryAction {
    DurabilityRecoveryAction::Reopen
}
