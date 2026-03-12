use crate::tests::support::*;

#[test]
fn complexity_budget_snapshot_visibility_state_avoids_record_materialization() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity(&mut runtime, "first");
    let _ = create_entity(&mut runtime, "second");

    runtime.reset_complexity_counters();
    let _snapshot = runtime.snapshot_access().snapshot();
    let counters = runtime.complexity_counters();

    assert_eq!(counters.visible_entity_records_materialized, 0);
    assert_eq!(counters.visible_relation_records_materialized, 0);
}

#[test]
fn complexity_budget_snapshot_pin_maintenance_is_incremental() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..6 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }
    let snapshot = runtime.snapshot_access().snapshot();
    let target = create_entity(&mut runtime, "target");

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, target, "updated");
    let after_commit = runtime.complexity_counters();
    assert_eq!(after_commit.snapshot_pin_full_rebuilds, 0);

    runtime.reset_complexity_counters();
    assert!(runtime.snapshot_access().release_snapshot(&snapshot));
    let after_release = runtime.complexity_counters();
    assert_eq!(after_release.snapshot_pin_full_rebuilds, 0);
    assert!(after_release.snapshot_pin_adjustments > 0);
}

#[test]
fn complexity_budget_branch_creation_reuses_cached_visibility_state() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let _ = create_relation(&mut runtime, left, right, "r0");

    runtime.reset_complexity_counters();
    runtime
        .history_authority().create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let counters = runtime.complexity_counters();

    assert_eq!(counters.visibility_entity_slot_scans, 0);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_contract_visibility_scans_are_explicitly_measured() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation_outcome = create_relation_outcome(&mut runtime, source, target, "r0");
    let snapshot = runtime.snapshot_access().snapshot();
    let historical_version = relation_outcome.version_id;
    let current_version = create_entity_outcome(&mut runtime, "later").version_id;

    runtime.reset_complexity_counters();
    let _ = runtime.visibility_reads().read_snapshot(&snapshot).unwrap();
    let snapshot_counters = runtime.complexity_counters();

    assert_eq!(snapshot_counters.visibility_entity_slot_scans, 0);
    assert_eq!(snapshot_counters.visibility_relation_slot_scans, 0);
    assert!(snapshot_counters.visible_entity_records_materialized >= 2);
    assert!(snapshot_counters.visible_relation_records_materialized >= 1);

    runtime.reset_complexity_counters();
    let _ = runtime.visibility_reads().read_version(historical_version);
    let current_version_counters = runtime.complexity_counters();

    assert_eq!(current_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(current_version_counters.visibility_relation_slot_scans, 0);
    assert!(current_version_counters.visible_entity_records_materialized >= 2);
    assert!(current_version_counters.visible_relation_records_materialized >= 1);

    runtime.reset_complexity_counters();
    let _ = runtime.visibility_reads().read_version(current_version);
    let historical_version_counters = runtime.complexity_counters();

    assert_eq!(historical_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(historical_version_counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_contract_invariant_materialization_is_declared_and_measured() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        commit_boundary: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });
    let entity = create_entity(&mut runtime, "a");

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, entity, "a-2");
    let counters = runtime.complexity_counters();

    assert!(counters.invariant_entity_slot_scans >= 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_snapshot_entity_limit_uses_live_bitsets_for_current_version() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        snapshot_audit: vec![InvariantRule::MaxSnapshotEntities(1)],
        ..InvariantCatalog::default()
    });
    let _ = create_entity(&mut runtime, "visible");

    runtime.reset_complexity_counters();
    let results = runtime.run_invariants(InvariantExecutionPoint::SnapshotPublication, false);
    let counters = runtime.complexity_counters();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].class, InvariantClass::SnapshotAudit);
    assert!(results[0].violations.is_empty());
    assert_eq!(counters.invariant_entity_slot_scans, 0);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_live_history_trimming_is_touched_record_bounded() {
    let mut runtime = runtime_with_test_schema();
    let create_a = create_entity_outcome(&mut runtime, "a");
    let entity_a = changed_entities(&create_a)[0];
    let create_b = create_entity_outcome(&mut runtime, "b");
    let entity_b = changed_entities(&create_b)[0];
    assert!(runtime.snapshot_access().release_snapshot(&create_a.snapshot));
    assert!(runtime.snapshot_access().release_snapshot(&create_b.snapshot));

    runtime.reset_complexity_counters();
    let update_a1 = update_entity(&mut runtime, entity_a, "a-1");
    assert!(runtime.snapshot_access().release_snapshot(&update_a1.snapshot));
    let _ = update_entity(&mut runtime, entity_a, "a-2");
    let counters = runtime.complexity_counters();

    assert_eq!(runtime.entity_history_len_for_test(entity_a), 1);
    assert_eq!(runtime.entity_history_len_for_test(entity_b), 1);
    assert!(counters.live_entity_history_entries_trimmed >= 1);
}

#[test]
fn complexity_budget_bidirectional_adjacency_avoids_relation_scans() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let relation = create_relation(&mut runtime, source, target, "r0");
    let version_id = runtime.history_access().latest_commit().unwrap().version_id;

    runtime.reset_complexity_counters();
    let outgoing = runtime.outgoing_relations_for_entity(source, version_id);
    let incoming = runtime.incoming_relations_for_entity(target, version_id);
    let counters = runtime.complexity_counters();

    assert_eq!(outgoing, vec![relation]);
    assert_eq!(incoming, vec![relation]);
    assert_eq!(counters.visibility_relation_slot_scans, 0);
}

#[test]
fn complexity_budget_partition_scoped_historical_entity_scans_are_partition_bounded() {
    let mut runtime = runtime_with_test_schema();
    let _left_a = create_entity_in_partition(&mut runtime, "left-a", PartitionId(7));
    let _left_b = create_entity_in_partition(&mut runtime, "left-b", PartitionId(7));
    let historical_version = runtime.history_access().latest_commit().unwrap().version_id;
    let _right_a = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let _right_b = create_entity_in_partition(&mut runtime, "right-b", PartitionId(11));

    runtime.reset_complexity_counters();
    let records = runtime.visibility_reads().visible_entities_of_kind_in_partition(
        PartitionId(7),
        KindId(1),
        historical_version,
    );
    let counters = runtime.complexity_counters();

    assert_eq!(records.len(), 2);
    assert_eq!(counters.visibility_entity_slot_scans, 2);
}

#[test]
fn complexity_budget_partition_scoped_historical_relation_scans_are_partition_bounded() {
    let mut runtime = runtime_with_test_schema();
    let left_source = create_entity_in_partition(&mut runtime, "left-source", PartitionId(7));
    let left_target = create_entity_in_partition(&mut runtime, "left-target", PartitionId(7));
    let right_source = create_entity_in_partition(&mut runtime, "right-source", PartitionId(11));
    let right_target = create_entity_in_partition(&mut runtime, "right-target", PartitionId(11));
    let _left_relation = create_relation_in_partition(
        &mut runtime,
        left_source,
        left_target,
        "left-r0",
        PartitionId(7),
    );
    let historical_version = runtime.history_access().latest_commit().unwrap().version_id;
    let _right_relation = create_relation_in_partition(
        &mut runtime,
        right_source,
        right_target,
        "right-r0",
        PartitionId(11),
    );

    runtime.reset_complexity_counters();
    let records = runtime.visibility_reads().visible_relations_of_kind_in_partition(
        PartitionId(7),
        KindId(2),
        historical_version,
    );
    let counters = runtime.complexity_counters();

    assert_eq!(records.len(), 1);
    assert_eq!(counters.visibility_relation_slot_scans, 1);
}
