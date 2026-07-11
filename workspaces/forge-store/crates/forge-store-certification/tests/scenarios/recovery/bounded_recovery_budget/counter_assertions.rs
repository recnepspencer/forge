use forge_store_recovery_physics::{
    RecoveryBudgetDenial, RecoveryBudgetDenialKind, RecoveryCounterSnapshot,
};

pub(crate) fn assert_counter_snapshot(
    counters: RecoveryCounterSnapshot,
    replayed_frames: usize,
    skipped_frames: usize,
    page_redos: usize,
) {
    assert_eq!(counters.replayed_frames(), replayed_frames);
    assert_eq!(counters.skipped_frames(), skipped_frames);
    assert_eq!(counters.validated_checkpoints(), 1);
    assert_eq!(counters.scanned_segments(), 1);
    assert_eq!(counters.page_redos(), page_redos);
    assert_eq!(counters.memory_envelope_bytes(), 128);
    assert_eq!(counters.memory_envelope_frames(), 1);
    assert_eq!(counters.allocation_bytes(), 128);
    assert_eq!(counters.residue_rejections(), 0);
    assert_eq!(counters.forbidden_full_store_scans(), 0);
}

pub(crate) fn assert_hostile_counter_snapshot(counters: RecoveryCounterSnapshot) {
    assert_eq!(counters.replayed_frames(), 3);
    assert_eq!(counters.skipped_frames(), 2);
    assert_eq!(counters.validated_checkpoints(), 1);
    assert_eq!(counters.scanned_segments(), 2);
    assert_eq!(counters.page_redos(), 1);
    assert_eq!(counters.memory_envelope_bytes(), 128);
    assert_eq!(counters.memory_envelope_frames(), 1);
    assert_eq!(counters.allocation_bytes(), 128);
    assert_eq!(counters.residue_rejections(), 0);
    assert_eq!(counters.forbidden_full_store_scans(), 0);
}

pub(crate) fn assert_same_bounded_work_except_store_footprint(
    left: RecoveryCounterSnapshot,
    right: RecoveryCounterSnapshot,
) {
    assert_eq!(left.replayed_frames(), right.replayed_frames());
    assert_eq!(left.skipped_frames(), right.skipped_frames());
    assert_eq!(left.validated_checkpoints(), right.validated_checkpoints());
    assert_eq!(left.scanned_segments(), right.scanned_segments());
    assert_eq!(left.page_redos(), right.page_redos());
    assert_eq!(left.memory_envelope_bytes(), right.memory_envelope_bytes());
    assert_eq!(
        left.memory_envelope_frames(),
        right.memory_envelope_frames()
    );
    assert_eq!(left.allocation_bytes(), right.allocation_bytes());
    assert_eq!(left.residue_rejections(), right.residue_rejections());
    assert_eq!(
        left.forbidden_full_store_scans(),
        right.forbidden_full_store_scans()
    );
}

pub(crate) fn assert_budget_denial_before_execution(
    denial: RecoveryBudgetDenial,
    matches_kind: impl FnOnce(&RecoveryBudgetDenialKind) -> bool,
) {
    assert!(matches_kind(denial.kind()), "{denial:?}");
    assert_eq!(denial.redo_execution_attempts(), 0);
    assert!(!denial.execution_started());
}
