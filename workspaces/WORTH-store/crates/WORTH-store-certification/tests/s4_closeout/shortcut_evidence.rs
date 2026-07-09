use worth_store_physical_backend::MmapFlushNotDurabilityCertifiedProfile;
use worth_store_recovery_physics::{
    AcknowledgmentPrecondition, AdmittedRedoFrame, BackendResidueKind, BackendResidueRejection,
    CheckpointIntervalContract, CheckpointManifest, IllegalAcknowledgmentDenial, LogSequenceNumber,
    RecoveryBudget, RecoveryBudgetDenial, RecoveryEntryAdmissionDenial,
    RecoveryEntryAdmissionDenialKind, RecoveryPhysicsCloseoutDenial, RecoveryRedoPlan,
    RecoverySourceCandidate, RuntimeRecoveryReportDenial, SyntheticRecoveryShortcutEvidence,
    WalAppendPlan, WalLsnRange, WalSegmentGeneration, WalSegmentId, WalTailReplayBudget,
};

use super::memory_budget_fixture::recovery_memory_envelope;
use super::redo_replay_fixture::{frame, grammar_for, page_lsn, redo_eligibility, valid_prefix};
use super::source_precedence_fixture::{checkpoint_base, trace, wal_tail_for_checkpoint};

pub fn all_shortcut_evidence() -> Vec<SyntheticRecoveryShortcutEvidence> {
    let eligibility = redo_eligibility(19, 20);
    vec![
        SyntheticRecoveryShortcutEvidence::from_raw_recovery_bytes(
            RecoveryEntryAdmissionDenial::new(
                RecoveryEntryAdmissionDenialKind::RawBytesCrossedIntegrityBoundary,
            ),
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_same_process_live_state_reuse(
            RuntimeRecoveryReportDenial::SameProcessLiveStateReuse,
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_backend_residue_guessing(
            BackendResidueRejection::new(
                BackendResidueKind::BackendDirectoryResidue,
                trace("backend-residue", 9),
            ),
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_unsupported_durability_claim(
            unsupported_durability_denial(),
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_invalid_page_lsn(
            super::redo_replay_fixture::missing_page_lsn(&eligibility).unwrap_err(),
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_torn_checkpoint(
            CheckpointManifest::torn_manifest().unwrap_err(),
        )
        .unwrap(),
        SyntheticRecoveryShortcutEvidence::from_unbounded_recovery_plan(unbounded_budget_denial())
            .unwrap(),
    ]
}

pub fn unrelated_residue_shortcut_denial() -> RecoveryPhysicsCloseoutDenial {
    SyntheticRecoveryShortcutEvidence::from_backend_residue_guessing(BackendResidueRejection::new(
        BackendResidueKind::StalePageImage,
        trace("stale-page", 10),
    ))
    .unwrap_err()
}

pub fn unrelated_budget_shortcut_denial() -> RecoveryPhysicsCloseoutDenial {
    SyntheticRecoveryShortcutEvidence::from_unbounded_recovery_plan(memory_budget_denial())
        .unwrap_err()
}

pub fn unsupported_durability_denial() -> IllegalAcknowledgmentDenial {
    let receipt = WalAppendPlan::<MmapFlushNotDurabilityCertifiedProfile>::new(
        WalSegmentId::new(42).unwrap(),
        WalSegmentGeneration::new(7).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(100), LogSequenceNumber::new(101)).unwrap(),
        "frame-digest-mmap",
        4096,
    )
    .unwrap()
    .record_written_bytes(4096)
    .finish()
    .unwrap();
    AcknowledgmentPrecondition::from_append_receipt(receipt).unwrap_err()
}

fn unbounded_budget_denial() -> RecoveryBudgetDenial {
    RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(128)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
    .source_precedence_graph("strict-test-profile")
    .reject_full_store_scan(98_765_432)
    .into_denial()
}

fn memory_budget_denial() -> RecoveryBudgetDenial {
    bounded_fixture_recovery_denial(4, 64)
}

fn bounded_fixture_recovery_denial(
    max_tail_frames: usize,
    max_memory_bytes: u64,
) -> RecoveryBudgetDenial {
    let (checkpoint, cutover) = checkpoint_base(10, 20, 19, 1);
    let tail = wal_tail_for_checkpoint(&cutover, 21, 2);
    let source = RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(4),
        WalTailReplayBudget::max_frames(4),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(128)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
    .source_precedence_graph("strict-test-profile")
    .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
    .unwrap()
    .discover(RecoverySourceCandidate::wal_tail(tail))
    .unwrap()
    .admit_sources();
    let prefix = valid_prefix(source.source(), 20, 21, [frame(20)]);
    let eligibility = redo_eligibility(19, 20);
    let grammar = grammar_for(&eligibility, 20, page_lsn(20)).unwrap();
    let admitted = AdmittedRedoFrame::admit(grammar, &prefix).unwrap();
    let plan =
        RecoveryRedoPlan::from_valid_prefix(source.source(), prefix, vec![admitted]).unwrap();
    RecoveryBudget::new(
        CheckpointIntervalContract::max_tail_frames(max_tail_frames),
        WalTailReplayBudget::max_frames(max_tail_frames),
        recovery_memory_envelope(),
    )
    .with_max_memory_envelope_bytes(max_memory_bytes)
    .with_max_allocation_bytes(128)
    .with_checkpoint_discovery_candidates(2)
    .admit_recovery(source, plan)
    .unwrap_err()
}
