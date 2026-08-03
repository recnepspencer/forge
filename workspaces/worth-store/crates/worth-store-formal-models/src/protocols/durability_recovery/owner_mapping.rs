use worth_store_physical_backend::{
    BackendDurabilityProfile, PosixFileFsyncDirFsyncProfile, ProductionStorageBoundarySeam,
    StorageBoundaryFault, StorageBoundaryTrace, WindowsFlushFileBuffersProfile,
};
use worth_store_recovery_physics::{
    CheckpointBaseAdmission, CheckpointCutoverReceipt, ExecutedWalDurabilityOutcome,
    RecoveryCompletion, RedoExecutionReceipt, RedoPlanningDenial, RedoPlanningDenialKind,
    ReopenedRecoveryArtifactAdmission,
};

use super::DurabilityRecoveryAction;

pub fn map_executed_wal_durability<P: BackendDurabilityProfile>(
    outcome: &ExecutedWalDurabilityOutcome<P>,
) -> Vec<DurabilityRecoveryAction> {
    let mut actions = vec![
        DurabilityRecoveryAction::WalAppendProposed,
        DurabilityRecoveryAction::WalAppendCompletedInMemory,
        DurabilityRecoveryAction::WalFenceRequested,
    ];
    if outcome
        .execution()
        .completed_barriers()
        .satisfies(P::REQUIRED_BARRIERS)
    {
        actions.push(DurabilityRecoveryAction::WalFenceCompleted);
    }
    if outcome
        .acknowledgment()
        .ack_basis()
        .completed_barriers()
        .satisfies(P::REQUIRED_BARRIERS)
    {
        actions.push(DurabilityRecoveryAction::WalAcknowledgmentLegal);
    }
    actions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityOwnerMappingDenial {
    CheckpointProfileDoesNotProveDirectorySync,
    DirectorySyncAbortWasNotInjected,
    DirectorySyncDidNotFail,
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

pub fn map_directory_sync_failure(
    _failure: &std::io::Error,
    trace: &StorageBoundaryTrace,
) -> Result<[DurabilityRecoveryAction; 2], DurabilityOwnerMappingDenial> {
    let injected = trace.injected().contains(&(
        ProductionStorageBoundarySeam::DirectorySync,
        StorageBoundaryFault::AbortBeforeDurabilityBarrier,
    ));
    if !injected {
        return Err(DurabilityOwnerMappingDenial::DirectorySyncAbortWasNotInjected);
    }
    Ok([
        DurabilityRecoveryAction::DirectorySyncFailed,
        DurabilityRecoveryAction::Crash,
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
