use worth_store_formal_models::{
    map_executed_wal_durability, DurabilityRecoveryAction, DurabilityRecoveryDenial,
    DurabilityRecoveryFrontier,
};
use worth_store_physical_backend::{
    BackendDurabilityBarrierDenialKind, BackendDurabilityProfile, PosixFileFsyncDirFsyncProfile,
    StoreDurabilityAdmission, StoreDurabilityExecutionBoundary, StoreDurabilityRequirement,
    StoreDurabilityRuntime, WalDurabilityBarrier,
};
use worth_store_recovery_physics::{
    LogSequenceNumber, WalAppendPlan, WalLsnRange, WalSegmentGeneration, WalSegmentId,
};

use super::scenario::{
    admitted_backend, execute_ordinary_durability_recovery, execute_ordinary_wal,
};

#[test]
fn ordinary_owner_execution_covers_every_durability_recovery_action() {
    let mut observed = execute_ordinary_durability_recovery();
    observed.sort_unstable();
    observed.dedup();

    assert_eq!(observed, DurabilityRecoveryAction::all());
}

#[test]
fn real_wal_execution_maps_to_legal_acknowledgment() {
    let outcome = execute_ordinary_wal(&std::env::temp_dir());
    let actions = map_executed_wal_durability(&outcome);
    let mut frontier = DurabilityRecoveryFrontier::initial();
    for action in actions.iter().copied() {
        frontier.apply(action).expect("owner trace refines model");
    }

    assert!(frontier.wal_acknowledged());
    assert_eq!(
        actions.last(),
        Some(&DurabilityRecoveryAction::WalAcknowledgmentLegal)
    );
    assert_eq!(outcome.execution().persisted_bytes(), 175);
}

#[test]
fn file_sync_crash_seam_cannot_mint_directory_barrier() {
    let backend =
        admitted_backend(worth_store_physical_backend::BackendTargetProfile::PosixFileFsyncDirSync);
    let requirement = StoreDurabilityRequirement::wal_ordering_barrier(
        PosixFileFsyncDirFsyncProfile::REQUIRED_BARRIERS,
    );
    let admission = StoreDurabilityAdmission::admit(requirement, &backend).expect("admitted WAL");
    let scope = WalAppendPlan::<PosixFileFsyncDirFsyncProfile>::new(
        WalSegmentId::new(9).unwrap(),
        WalSegmentGeneration::new(2).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(1), LogSequenceNumber::new(2)).unwrap(),
        "file-sync-crash-seam",
        4,
    )
    .unwrap()
    .record_written_bytes(4)
    .durability_scope();
    let accepted = admission.submit_write(scope).backend_accepted();
    let proof = StoreDurabilityRuntime::new()
        .persist_and_execute_to(
            &std::env::temp_dir(),
            b"seam",
            &accepted,
            StoreDurabilityExecutionBoundary::FileSynchronized,
        )
        .expect("file sync seam is executable");

    let denial = proof
        .certify_completed_barrier::<PosixFileFsyncDirFsyncProfile>(
            WalDurabilityBarrier::WalDirectoryFsync,
        )
        .expect_err("directory barrier cannot precede directory sync");
    assert_eq!(
        denial.kind(),
        BackendDurabilityBarrierDenialKind::BarrierNotCompleted
    );
}

#[test]
fn frontier_rejects_publication_shortcuts() {
    let mut frontier = DurabilityRecoveryFrontier::initial();
    assert_eq!(
        frontier.apply(DurabilityRecoveryAction::PageFlushCompleted),
        Err(DurabilityRecoveryDenial::PageFlushAheadOfWal)
    );
    assert_eq!(
        frontier.apply(DurabilityRecoveryAction::CheckpointPublished),
        Err(DurabilityRecoveryDenial::DirectorySyncNotDurable)
    );
    assert_eq!(
        frontier.apply(DurabilityRecoveryAction::RecoveredRootPublicationCompleted),
        Err(DurabilityRecoveryDenial::ReplayNotResolved)
    );
}

#[test]
fn every_modeled_durability_cut_reopens_deterministically() {
    use DurabilityRecoveryAction as Action;

    let legal_trace = [
        Action::WalAppendProposed,
        Action::WalAppendCompletedInMemory,
        Action::WalFenceRequested,
        Action::WalFenceCompleted,
        Action::WalAcknowledgmentLegal,
        Action::PageFlushRequested,
        Action::PageFlushCompleted,
        Action::CheckpointBegun,
        Action::CheckpointDurable,
        Action::DirectorySyncCompleted,
        Action::CheckpointPublished,
        Action::CheckpointSelected,
        Action::RecoveryReplayRequired,
        Action::RecoveryReplayApplied,
        Action::RecoveredRootPublicationPending,
        Action::RecoveredRootPublicationCompleted,
    ];

    for seam in 0..=legal_trace.len() {
        let mut first = DurabilityRecoveryFrontier::initial();
        for action in legal_trace[..seam].iter().copied() {
            first.apply(action).unwrap();
        }
        let mut second = first;
        first.apply(Action::Crash).unwrap();
        first.apply(Action::Reopen).unwrap();
        second.apply(Action::Crash).unwrap();
        second.apply(Action::Reopen).unwrap();
        assert_eq!(
            first, second,
            "reopen classification drifted at seam {seam}"
        );
        assert!(!first.is_crashed());
    }
}

#[test]
fn wal_ack_and_redo_generation_denials_are_reachable() {
    let mut frontier = DurabilityRecoveryFrontier::initial();
    assert_eq!(
        frontier.apply(DurabilityRecoveryAction::WalAcknowledgmentLegal),
        Err(DurabilityRecoveryDenial::AmbiguousWalDurability)
    );
    assert_eq!(
        frontier.apply(DurabilityRecoveryAction::RecoveryReplayRejectedGenerationMismatch),
        Err(DurabilityRecoveryDenial::RedoGenerationMismatch)
    );
}
