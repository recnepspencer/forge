#[path = "../../../support/recovery/bounded_recovery_budget/budget_fixture.rs"]
mod budget_fixture;
#[path = "counter_assertions.rs"]
mod counter_assertions;
use worth_store_test_support::harness::recovery::memory_budget as memory_budget_fixture;
use worth_store_test_support::harness::recovery::redo_replay as redo_replay_fixture;
use worth_store_test_support::harness::recovery::source_precedence as source_precedence_fixture;

use worth_store_recovery_physics::{RecoveryBudgetDenialKind, RecoverySourceCandidate};

use budget_fixture::{
    budget_fixture, execute_equivalent_envelope, multi_frame_budget_fixture, multi_page_cursor,
    residue_budget_fixture, with_admitted_bounded, with_admitted_hostile_bounded, with_budget,
    wrong_same_count_source_admission, RecoveryBudgetLimits,
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
fn excessive_wal_tail_frames_deny_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_tail_interval_frames(0),
            |budget| {
                budget
                    .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
                    .unwrap_err()
            },
        ),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::WalTailFrameBudgetExceeded { planned: 1, max: 0 }
            )
        },
    );
}

#[test]
fn excessive_wal_tail_segments_deny_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_scanned_segments(0),
            |budget| {
                budget
                    .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
                    .unwrap_err()
            },
        ),
        |kind| {
            matches!(
                kind,
                RecoveryBudgetDenialKind::WalTailSegmentBudgetExceeded { scanned: 1, max: 0 }
            )
        },
    );
}

#[test]
fn excessive_memory_envelope_denies_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_memory_bytes(64),
            |budget| {
                budget
                    .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
                    .unwrap_err()
            },
        ),
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
}

#[test]
fn excessive_physical_allocation_denies_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_allocation_bytes(64),
            |budget| {
                budget
                    .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
                    .unwrap_err()
            },
        ),
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
}

#[test]
fn excessive_checkpoint_discovery_denies_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_checkpoint_candidates(1),
            |budget| {
                budget
                    .source_precedence_graph("strict-test-profile")
                    .discover(RecoverySourceCandidate::checkpoint_base(
                        fixture.checkpoint.clone(),
                    ))
                    .unwrap()
                    .discover(RecoverySourceCandidate::wal_tail(fixture.tail.clone()))
                    .unwrap_err()
            },
        ),
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
}

#[test]
fn forbidden_full_store_scan_denies_before_redo_execution() {
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture(),
            |budget| {
                budget
                    .source_precedence_graph("strict-test-profile")
                    .reject_full_store_scan(98_765_432)
                    .into_denial()
            },
        ),
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
}

#[test]
fn same_count_different_recovery_sources_deny_before_redo_execution() {
    let fixture = budget_fixture();
    assert_budget_denial_before_execution(
        with_budget(
            RecoveryBudgetLimits::bounded_single_frame_fixture(),
            |budget| {
                budget
                    .admit_recovery(
                        wrong_same_count_source_admission(&fixture),
                        fixture.redo_plan.clone(),
                    )
                    .unwrap_err()
            },
        ),
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
    let applied = with_admitted_bounded(&fixture, |plan| {
        plan.execute(&fixture.cursor_from_lsn(19, "checkpoint-page"))
            .unwrap()
    });
    assert_counter_snapshot(applied.counters(), 1, 0, 1);

    let skipped = with_admitted_bounded(&fixture, |plan| {
        plan.execute(&fixture.cursor_from_lsn(20, "already-current"))
            .unwrap()
    });
    assert_counter_snapshot(skipped.counters(), 1, 1, 0);

    let hostile = multi_frame_budget_fixture();
    let mixed = with_admitted_hostile_bounded(&hostile, |plan| {
        plan.execute(&multi_page_cursor([
            (20, 20, 20),
            (21, 21, 21),
            (22, 21, 22),
        ]))
        .unwrap()
    });
    assert_hostile_counter_snapshot(mixed.counters());
}

#[test]
fn recovery_counter_snapshot_counts_rejected_residue_candidates() {
    let fixture = residue_budget_fixture();
    let receipt = with_budget(
        RecoveryBudgetLimits::bounded_single_frame_fixture().with_max_checkpoint_candidates(3),
        |budget| {
            budget
                .admit_recovery(fixture.source_admission.clone(), fixture.redo_plan.clone())
                .unwrap()
                .execute(&fixture.cursor_from_lsn(19, "checkpoint-page"))
                .unwrap()
        },
    );

    assert_eq!(receipt.counters().residue_rejections(), 1);
}
