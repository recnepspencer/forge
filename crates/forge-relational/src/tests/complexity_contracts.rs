// CONTRACT: complexity budgets and executable work proofs
// LANES: declarations, partition-local-commit, incremental-pin-maintenance, visibility-scan-accounting, live-history-trimming, bidirectional-adjacency

use super::*;

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
fn complexity_budget_relation_identity_validation_avoids_partition_scan() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _existing = create_relation(&mut runtime, source, target, "existing");
    for index in 0..12 {
        let other_source = create_entity(&mut runtime, &format!("other-source-{index}"));
        let other_target = create_entity(&mut runtime, &format!("other-target-{index}"));
        let _ = create_relation(&mut runtime, other_source, other_target, &format!("r{index}"));
    }

    runtime.reset_complexity_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(TransactionIntent::CreateRelation(
            crate::data::transaction::RelationSpec {
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
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(TransactionIntent::UpdateEntity {
            entity_id: target,
            payload: RecordPayload::StructuredJson(json!({"name":"other"})),
        }),
    );
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
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(TransactionIntent::UpdateEntity {
            entity_id: target,
            payload: RecordPayload::StructuredJson(json!({"name":"other"})),
        }),
    );
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
fn complexity_contract_visibility_scans_are_explicitly_measured() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");
    let _relation = create_relation(&mut runtime, source, target, "r0");
    let snapshot = runtime.snapshot();

    runtime.reset_complexity_counters();
    let _ = runtime.read_snapshot(&snapshot).unwrap();
    let counters = runtime.complexity_counters();

    assert!(counters.visibility_entity_slot_scans >= 2);
    assert!(counters.visibility_relation_slot_scans >= 1);
    assert!(counters.visible_entity_records_materialized >= 2);
    assert!(counters.visible_relation_records_materialized >= 1);
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
