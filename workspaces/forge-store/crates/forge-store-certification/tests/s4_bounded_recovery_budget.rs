#[path = "s4_bounded_recovery_budget/budget_fixture.rs"]
mod budget_fixture;
#[path = "s4_bounded_recovery_budget/counter_assertions.rs"]
mod counter_assertions;
#[path = "s4_bounded_recovery_budget/memory_budget_fixture.rs"]
mod memory_budget_fixture;
#[allow(dead_code, unused_imports)]
#[path = "s4_idempotent_redo_replay/redo_replay_fixture.rs"]
mod redo_replay_fixture;
#[allow(dead_code)]
#[path = "s4_recovery_source_precedence/source_precedence_fixture.rs"]
mod source_precedence_fixture;

use forge_store_recovery_physics::{RecoveryBudgetDenialKind, RecoverySourceCandidate};

use budget_fixture::{
    admit_bounded, admit_hostile_bounded, budget_fixture, budget_with, execute_equivalent_envelope,
    multi_frame_budget_fixture, multi_page_cursor, residue_budget_fixture,
    wrong_same_count_source_admission,
};
use counter_assertions::{
    assert_budget_denial_before_execution, assert_counter_snapshot,
    assert_hostile_counter_snapshot, assert_same_bounded_work_except_store_footprint,
};

#[test]
fn equivalent_checkpoint_tail_envelopes_recover_independent_of_total_store_size() {
    let small_store = execute_equivalent_envelope(1_024);
    let large_store = execute_equivalent_envelope(98_765_432);

    assert_ne!(small_store.total_store_pages, large_store.total_store_pages);
    assert_eq!(small_store.counters.total_store_pages(), 1_024);
    assert_eq!(large_store.counters.total_store_pages(), 98_765_432);
    assert_eq!(small_store.counters.forbidden_full_store_scans(), 0);
    assert_eq!(large_store.counters.forbidden_full_store_scans(), 0);
    assert_eq!(
        small_store.recovered_root, large_store.recovered_root,
        "total store size metadata must not enter bounded recovery authority"
    );
    assert_same_bounded_work_except_store_footprint(small_store.counters, large_store.counters);
}

#[test]
fn oversized_recovery_work_denies_before_redo_execution() {
    let fixture = budget_fixture();

    assert_budget_denial_before_execution(
        budget_with(0, 4, 2, 4, 128, 128, 2)
            .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::WalTailFrameBudgetExceeded { planned: 1, max: 0 }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 0, 4, 128, 128, 2)
            .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::WalTailSegmentBudgetExceeded { scanned: 1, max: 0 }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 2, 4, 64, 128, 2)
            .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::MemoryEnvelopeBudgetExceeded {
                    admitted_bytes: 128,
                    max_bytes: 64,
                }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 2, 4, 128, 64, 2)
            .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::AllocationBudgetExceeded {
                    allocated_bytes: 128,
                    max_bytes: 64,
                }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 2, 4, 128, 128, 1)
            .source_precedence_graph("strict-test-profile")
            .discover(RecoverySourceCandidate::checkpoint_base(
                fixture.checkpoint.clone(),
            ))
            .unwrap()
            .discover(RecoverySourceCandidate::wal_tail(fixture.tail.clone()))
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::CheckpointDiscoveryBudgetExceeded {
                    discovered: 2,
                    max: 1,
                }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 2, 4, 128, 128, 2)
            .source_precedence_graph("strict-test-profile")
            .reject_full_store_scan(98_765_432)
            .into_denial(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::ForbiddenFullStoreScan {
                    attempted_pages: 98_765_432,
                    checkpoint_interval_frames: 4,
                    wal_tail_frame_limit: 4,
                }
            )
        },
    );

    assert_budget_denial_before_execution(
        budget_with(4, 4, 2, 4, 128, 128, 2)
            .admit_recovery(
                wrong_same_count_source_admission(&fixture),
                fixture.redo_plan.clone(),
            )
            .unwrap_err(),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::RecoverySourceAdmissionMismatch {
                    admitted_candidates: 2,
                    planned_candidates: 2,
                    ..
                }
            )
        },
    );
}

#[test]
fn recovery_counter_snapshot_matches_applied_and_skipped_replay_work() {
    let fixture = budget_fixture();
    let applied = admit_bounded(&fixture)
        .execute(&fixture.cursor_from_lsn(19, "checkpoint-page"))
        .unwrap();
    assert_counter_snapshot(applied.counters(), 1, 0, 1);

    let skipped = admit_bounded(&fixture)
        .execute(&fixture.cursor_from_lsn(20, "already-current"))
        .unwrap();
    assert_counter_snapshot(skipped.counters(), 1, 1, 0);

    let hostile = multi_frame_budget_fixture();
    let mixed = admit_hostile_bounded(&hostile)
        .execute(&multi_page_cursor([
            (20, 20, 20),
            (21, 21, 21),
            (22, 21, 22),
        ]))
        .unwrap();
    assert_hostile_counter_snapshot(mixed.counters());
}

#[test]
fn recovery_counter_snapshot_counts_rejected_residue_candidates() {
    let fixture = residue_budget_fixture();
    let receipt = budget_with(4, 4, 2, 4, 128, 128, 3)
        .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
        .unwrap()
        .execute(&fixture.cursor_from_lsn(19, "checkpoint-page"))
        .unwrap();

    assert_eq!(receipt.counters().residue_rejections(), 1);
}
