use crate::facade::history::BranchId;
use crate::tests::support::*;

#[test]
fn complexity_budget_snapshot_visibility_state_avoids_record_materialization() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity(&mut runtime, "first");
    let _ = create_entity(&mut runtime, "second");

    runtime.performance_access().reset_counters();
    let _snapshot = runtime.visibility_authority().snapshot();
    let counters = runtime.performance_access().counters();

    assert_eq!(
        counters.visible_authoritative_entity_records_materialized,
        0
    );
    assert_eq!(
        counters.visible_authoritative_relation_records_materialized,
        0
    );
}

#[test]
fn complexity_budget_snapshot_pin_maintenance_is_incremental() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }
    let snapshot = runtime.visibility_authority().snapshot();
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, target, "updated");
    let after_commit = runtime.performance_access().counters();
    assert_eq!(after_commit.snapshot_pin_full_rebuilds, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&snapshot));
    let after_release = runtime.performance_access().counters();
    assert_eq!(after_release.snapshot_pin_full_rebuilds, 0);
    assert!(after_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_budget_duplicate_active_snapshots_share_one_pin_lease_per_version() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.performance_access().reset_counters();
    let first = runtime.visibility_authority().snapshot();
    let first_open = runtime.performance_access().counters();
    assert!(first_open.snapshot_pin_adjustments > 0);
    assert_eq!(first_open.visibility_cache_snapshot_promotions, 1);

    runtime.performance_access().reset_counters();
    let second = runtime.visibility_authority().snapshot();
    let second_open = runtime.performance_access().counters();
    assert_eq!(second_open.snapshot_pin_adjustments, 0);
    assert_eq!(second_open.visibility_cache_snapshot_promotions, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&first));
    let first_release = runtime.performance_access().counters();
    assert_eq!(first_release.snapshot_pin_adjustments, 0);

    runtime.performance_access().reset_counters();
    assert!(runtime.visibility_authority().release_snapshot(&second));
    let second_release = runtime.performance_access().counters();
    assert!(second_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_budget_branch_creation_reuses_cached_visibility_state() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _ = create_relation(&mut runtime, left, right, "r0");

    runtime.performance_access().reset_counters();
    runtime
        .history_authority()
        .fork_branch_from(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.visibility_entity_slot_scans, 0);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_contract_visibility_scans_are_explicitly_measured() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r0");
    let snapshot = runtime.visibility_authority().snapshot();
    let historical_version = relation_outcome.version_id;
    let current_version = create_entity_outcome(&mut runtime, "later").version_id;

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_snapshot(&snapshot).unwrap();
    let snapshot_counters = runtime.performance_access().counters();

    assert_eq!(snapshot_counters.visibility_entity_slot_scans, 0);
    assert_eq!(snapshot_counters.visibility_relation_slot_scans, 0);
    assert!(snapshot_counters.visible_authoritative_entity_records_materialized >= 2);
    assert!(snapshot_counters.visible_authoritative_relation_records_materialized >= 1);

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(historical_version);
    let historical_version_counters = runtime.performance_access().counters();

    assert_eq!(historical_version_counters.visibility_entity_slot_scans, 2);
    assert_eq!(
        historical_version_counters.visibility_relation_slot_scans,
        1
    );
    assert_eq!(
        historical_version_counters.visibility_cache_miss_reconstructions,
        1
    );
    assert_eq!(
        historical_version_counters.visibility_exact_state_materializations,
        0
    );
    assert!(historical_version_counters.visible_authoritative_entity_records_materialized >= 2);
    assert!(historical_version_counters.visible_authoritative_relation_records_materialized >= 1);

    runtime.performance_access().reset_counters();
    let _ = runtime.read_truth().read_version(current_version);
    let current_version_counters = runtime.performance_access().counters();

    assert_eq!(current_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(current_version_counters.visibility_relation_slot_scans, 0);
    assert_eq!(
        current_version_counters.visibility_cache_miss_reconstructions,
        1
    );
    assert_eq!(
        current_version_counters.visibility_exact_state_materializations,
        0
    );
    assert_eq!(current_version_counters.visibility_cache_hits, 0);
}
