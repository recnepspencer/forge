use crate::{evidence::StoreCounters, WORTHStoreBuilder, StoreCounterSnapshot};

#[test]
fn milestone_10_counter_fields_default_to_zero() {
    let snapshot = StoreCounterSnapshot::default();

    assert_eq!(snapshot.retention_policy_evaluation_count, 0);
    assert_eq!(snapshot.retained_authoritative_range_count, 0);
    assert_eq!(snapshot.expired_authoritative_range_count, 0);
    assert_eq!(snapshot.compaction_plan_count, 0);
    assert_eq!(snapshot.compacted_delta_layer_count, 0);
    assert_eq!(snapshot.compacted_snapshot_family_count, 0);
    assert_eq!(snapshot.compacted_layout_family_count, 0);
    assert_eq!(snapshot.compaction_cutover_count, 0);
    assert_eq!(snapshot.compaction_cutover_rejection_count, 0);
    assert_eq!(snapshot.reclaim_candidate_count, 0);
    assert_eq!(snapshot.reclaimed_authoritative_artifact_count, 0);
    assert_eq!(snapshot.reclaimed_derived_artifact_count, 0);
    assert_eq!(snapshot.reclaim_rejected_live_basis_count, 0);
    assert_eq!(snapshot.retention_closure_ancestor_count, 0);
    assert_eq!(snapshot.retention_closure_failure_count, 0);
    assert_eq!(snapshot.retained_range_rebuild_count, 0);
    assert_eq!(snapshot.rebuild_debt_count, 0);
    assert_eq!(snapshot.compaction_debt_count, 0);
    assert_eq!(snapshot.retention_truth_parity_failure_count, 0);
    assert_eq!(snapshot.retention_restore_parity_failure_count, 0);
    assert_eq!(snapshot.retention_artifact_rebuild_failure_count, 0);
}

#[test]
fn live_store_snapshot_exposes_zeroed_milestone_10_counters_before_wiring() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let snapshot = store.counters();

    assert_eq!(snapshot.retention_policy_evaluation_count, 0);
    assert_eq!(snapshot.compaction_plan_count, 0);
    assert_eq!(snapshot.reclaim_candidate_count, 0);
    assert_eq!(snapshot.retention_artifact_rebuild_failure_count, 0);
}

#[test]
fn milestone_10_counter_recorders_increment_exact_fields() {
    let counters = StoreCounters::default();

    counters.record_retention_policy_evaluation();
    counters.record_retained_authoritative_ranges(3);
    counters.record_expired_authoritative_ranges(2);
    counters.record_compaction_plan();
    counters.record_compacted_delta_layers(4);
    counters.record_compacted_snapshot_families(5);
    counters.record_compacted_layout_families(6);
    counters.record_compaction_cutover();
    counters.record_compaction_cutover_rejection();
    counters.record_reclaim_candidates(7);
    counters.record_reclaimed_authoritative_artifacts(8);
    counters.record_reclaimed_derived_artifacts(9);
    counters.record_reclaim_rejected_live_basis();
    counters.record_retention_closure(10);
    counters.record_retention_closure_failure();
    counters.record_retained_range_rebuild();
    counters.record_rebuild_debt(11);
    counters.record_compaction_debt(12);
    counters.record_retention_truth_parity_failure();
    counters.record_retention_restore_parity_failure();
    counters.record_retention_artifact_rebuild_failure();

    let snapshot = counters.snapshot();
    assert_eq!(snapshot.retention_policy_evaluation_count, 1);
    assert_eq!(snapshot.retained_authoritative_range_count, 3);
    assert_eq!(snapshot.expired_authoritative_range_count, 2);
    assert_eq!(snapshot.compaction_plan_count, 1);
    assert_eq!(snapshot.compacted_delta_layer_count, 4);
    assert_eq!(snapshot.compacted_snapshot_family_count, 5);
    assert_eq!(snapshot.compacted_layout_family_count, 6);
    assert_eq!(snapshot.compaction_cutover_count, 1);
    assert_eq!(snapshot.compaction_cutover_rejection_count, 1);
    assert_eq!(snapshot.reclaim_candidate_count, 7);
    assert_eq!(snapshot.reclaimed_authoritative_artifact_count, 8);
    assert_eq!(snapshot.reclaimed_derived_artifact_count, 9);
    assert_eq!(snapshot.reclaim_rejected_live_basis_count, 1);
    assert_eq!(snapshot.retention_closure_ancestor_count, 10);
    assert_eq!(snapshot.retention_closure_failure_count, 1);
    assert_eq!(snapshot.retained_range_rebuild_count, 1);
    assert_eq!(snapshot.rebuild_debt_count, 11);
    assert_eq!(snapshot.compaction_debt_count, 12);
    assert_eq!(snapshot.retention_truth_parity_failure_count, 1);
    assert_eq!(snapshot.retention_restore_parity_failure_count, 1);
    assert_eq!(snapshot.retention_artifact_rebuild_failure_count, 1);
}
