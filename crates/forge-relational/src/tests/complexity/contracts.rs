// CONTRACT: complexity budgets and executable work proofs
// LANES: declarations, partition-local-commit, incremental-pin-maintenance, visibility-scan-accounting, live-history-trimming, bidirectional-adjacency

use crate::tests::support::*;

#[test]
fn complexity_contract_registry_covers_runtime_hot_paths() {
    let runtime = runtime_with_test_schema();
    let contracts = runtime.complexity_contracts();

    assert!(contracts.len() >= 6);
    assert!(contracts
        .iter()
        .all(|contract| !contract.proof_tests.is_empty()));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.partition_local_commit"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.slot_local_mutation_journal"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.relation_identity_validation"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.unique_entity_invariant_lookup"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.current_state.clone"));
    assert!(contracts
        .iter()
        .any(|contract| contract.id == "runtime.snapshot_pin_maintenance"));
}

#[test]
fn complexity_budget_partition_local_commit_reports_touched_partitions() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, left, "left-updated");
    let single_partition = runtime.complexity_counters();
    assert_eq!(single_partition.partitions_touched_by_commit, 1);
    assert_eq!(single_partition.full_state_clones, 0);

    runtime.reset_complexity_counters();
    let _ = create_relation_in_partition(&mut runtime, left, right, "cross", PartitionId(13));
    let cross_partition = runtime.complexity_counters();
    assert_eq!(cross_partition.partitions_touched_by_commit, 3);
    assert_eq!(cross_partition.full_state_clones, 0);
}

#[test]
fn complexity_budget_bulk_create_reserves_partition_local_capacity() {
    let mut runtime = runtime_with_test_schema();
    runtime.reset_complexity_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("bulk-entities").push(
        TransactionIntent::BulkCreateEntities {
            partition_id: PartitionId(41),
            kind_id: KindId(1),
            client_keys: vec![
                InternedString::Raw("a".to_string()),
                InternedString::Raw("b".to_string()),
                InternedString::Raw("c".to_string()),
            ],
            payloads: vec![
                RecordPayload::StructuredJson(json!({"name":"a"})),
                RecordPayload::StructuredJson(json!({"name":"b"})),
                RecordPayload::StructuredJson(json!({"name":"c"})),
            ],
        },
    ));
    let _ = txn.commit().unwrap();
    let counters = runtime.complexity_counters();

    assert_eq!(counters.bulk_entity_slots_reserved, 3);
    assert_eq!(counters.bulk_relation_slots_reserved, 0);
}

#[test]
fn complexity_budget_mutation_structural_invariants_are_touched_slot_bounded() {
    let mut runtime = runtime_with_test_schema();
    let target = create_entity(&mut runtime, "target");
    for index in 0..8 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, target, "target-updated");
    let counters = runtime.complexity_counters();

    assert_eq!(counters.entity_slots_touched_by_commit, 1);
    assert_eq!(counters.relation_slots_touched_by_commit, 0);
    assert_eq!(counters.invariant_entity_slot_scans, 1);
}

#[test]
fn complexity_budget_relation_structural_invariants_are_touched_slot_bounded() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    runtime.reset_complexity_counters();
    let _ = create_relation(&mut runtime, source, target, "r0");
    let counters = runtime.complexity_counters();

    assert_eq!(counters.entity_slots_touched_by_commit, 0);
    assert_eq!(counters.relation_slots_touched_by_commit, 1);
    assert_eq!(counters.invariant_relation_slot_scans, 1);
}

#[test]
fn complexity_budget_relation_identity_validation_avoids_partition_scan() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");
    for index in 0..12 {
        let other_source = create_entity(&mut runtime, &format!("other-source-{index}"));
        let other_target = create_entity(&mut runtime, &format!("other-target-{index}"));
        let _ = create_relation(
            &mut runtime,
            other_source,
            other_target,
            &format!("r{index}"),
        );
    }

    runtime.reset_complexity_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(TransactionIntent::CreateRelation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("dup".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.complexity_counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
    assert_eq!(counters.relation_identity_candidates_scanned, 1);
}

#[test]
fn complexity_budget_unique_entity_invariant_uses_changed_set_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        always_on_structural: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.rebuild_unique_field_indexes_for_test();

    runtime.reset_complexity_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("duplicate-name").push(
        TransactionIntent::UpdateEntity {
            entity_id: target,
            payload: RecordPayload::StructuredJson(json!({"name":"other"})),
        },
    ));
    let error = txn.commit().unwrap_err();
    let counters = runtime.complexity_counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_commit_boundary_unique_invariant_uses_merged_plan_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        commit_boundary: vec![InvariantRule::UniqueEntityPayloadField("name".to_string())],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.rebuild_unique_field_indexes_for_test();

    runtime.reset_complexity_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new("duplicate-name").push(
        TransactionIntent::UpdateEntity {
            entity_id: target,
            payload: RecordPayload::StructuredJson(json!({"name":"other"})),
        },
    ));
    let error = txn.commit().unwrap_err();
    let counters = runtime.complexity_counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict(ref conflict)
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_contract_current_state_clone_is_declared_and_measured() {
    let mut runtime = runtime_with_test_schema();
    for index in 0..8 {
        let _ = create_entity(&mut runtime, &format!("e{index}"));
    }

    runtime.reset_complexity_counters();
    let entity = create_entity(&mut runtime, "target");
    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, entity, "target-updated");
    let counters = runtime.complexity_counters();

    assert_eq!(counters.full_state_clones, 0);
    assert_eq!(counters.partitions_cloned, 0);
    assert_eq!(counters.entity_slots_cloned, 0);
    assert_eq!(counters.relation_slots_cloned, 0);
}

#[test]
fn complexity_budget_snapshot_visibility_state_avoids_record_materialization() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity(&mut runtime, "first");
    let _ = create_entity(&mut runtime, "second");

    runtime.reset_complexity_counters();
    let _snapshot = runtime.snapshot();
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
    let snapshot = runtime.snapshot();
    let target = create_entity(&mut runtime, "target");

    runtime.reset_complexity_counters();
    let _ = update_entity(&mut runtime, target, "updated");
    let after_commit = runtime.complexity_counters();
    assert_eq!(after_commit.snapshot_pin_full_rebuilds, 0);

    runtime.reset_complexity_counters();
    assert!(runtime.release_snapshot(&snapshot));
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
        .create_branch(
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
    let snapshot = runtime.snapshot();
    let historical_version = relation_outcome.version_id;
    let current_version = create_entity_outcome(&mut runtime, "later").version_id;

    runtime.reset_complexity_counters();
    let _ = runtime.read_snapshot(&snapshot).unwrap();
    let snapshot_counters = runtime.complexity_counters();

    assert_eq!(snapshot_counters.visibility_entity_slot_scans, 0);
    assert_eq!(snapshot_counters.visibility_relation_slot_scans, 0);
    assert!(snapshot_counters.visible_entity_records_materialized >= 2);
    assert!(snapshot_counters.visible_relation_records_materialized >= 1);

    runtime.reset_complexity_counters();
    let _ = runtime.read_version(historical_version);
    let current_version_counters = runtime.complexity_counters();

    assert_eq!(current_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(current_version_counters.visibility_relation_slot_scans, 0);
    assert!(current_version_counters.visible_entity_records_materialized >= 2);
    assert!(current_version_counters.visible_relation_records_materialized >= 1);

    runtime.reset_complexity_counters();
    let _ = runtime.read_version(current_version);
    let historical_version_counters = runtime.complexity_counters();

    assert_eq!(historical_version_counters.visibility_entity_slot_scans, 0);
    assert_eq!(
        historical_version_counters.visibility_relation_slot_scans,
        0
    );
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
    assert!(runtime.release_snapshot(&create_a.snapshot));
    assert!(runtime.release_snapshot(&create_b.snapshot));

    runtime.reset_complexity_counters();
    let update_a1 = update_entity(&mut runtime, entity_a, "a-1");
    assert!(runtime.release_snapshot(&update_a1.snapshot));
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
    let version_id = runtime.latest_commit().unwrap().version_id;

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
    let historical_version = runtime.latest_commit().unwrap().version_id;
    let _right_a = create_entity_in_partition(&mut runtime, "right-a", PartitionId(11));
    let _right_b = create_entity_in_partition(&mut runtime, "right-b", PartitionId(11));

    runtime.reset_complexity_counters();
    let records = runtime.visible_entities_of_kind_in_partition(
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
    let historical_version = runtime.latest_commit().unwrap().version_id;
    let _right_relation = create_relation_in_partition(
        &mut runtime,
        right_source,
        right_target,
        "right-r0",
        PartitionId(11),
    );

    runtime.reset_complexity_counters();
    let records = runtime.visible_relations_of_kind_in_partition(
        PartitionId(7),
        KindId(2),
        historical_version,
    );
    let counters = runtime.complexity_counters();

    assert_eq!(records.len(), 1);
    assert_eq!(counters.visibility_relation_slot_scans, 1);
}
