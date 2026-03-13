use crate::facade::transactions::CommitTopology;
use crate::tests::support::*;

#[test]
fn complexity_contract_registry_covers_runtime_hot_paths() {
    let runtime = runtime_with_test_schema();
    let contracts = runtime.performance_access().contracts();

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

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, left, "left-updated");
    let single_partition = runtime.performance_access().counters();
    assert_eq!(single_partition.partitions_touched_by_commit, 1);
    assert_eq!(single_partition.full_state_clones, 0);

    runtime.performance_access().reset_counters();
    let _ = create_relation_in_partition(&mut runtime, left, right, "cross", PartitionId(13));
    let cross_partition = runtime.performance_access().counters();
    assert_eq!(cross_partition.partitions_touched_by_commit, 3);
    assert_eq!(cross_partition.full_state_clones, 0);
}

#[test]
fn complexity_budget_commit_topology_inference_distinguishes_flat_and_graph_mutations() {
    let mut runtime = runtime_with_test_schema();
    let left = create_entity_in_partition(&mut runtime, "left", PartitionId(7));
    let right = create_entity_in_partition(&mut runtime, "right", PartitionId(11));

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, left, "left-updated");
    let flat = runtime.performance_access().counters();
    assert_eq!(
        flat.commit_topology_flags,
        CommitTopology::FlatEntityBatch.mask()
    );

    runtime.performance_access().reset_counters();
    let _ = create_relation_in_partition(&mut runtime, left, right, "cross", PartitionId(13));
    let graph = runtime.performance_access().counters();
    assert_eq!(
        graph.commit_topology_flags,
        CommitTopology::GraphMutation.mask()
    );
}

#[test]
fn complexity_budget_bulk_create_reserves_partition_local_capacity() {
    let mut runtime = runtime_with_test_schema();
    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("bulk-entities").push(MutationIntent::Create(
            CreateIntent::BulkEntities(BulkEntityCreateIntent {
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
            }),
        )),
    );
    let _ = txn.commit().unwrap();
    let counters = runtime.performance_access().counters();

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

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, target, "target-updated");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.entity_slots_touched_by_commit, 1);
    assert_eq!(counters.relation_slots_touched_by_commit, 0);
    assert_eq!(counters.invariant_entity_slot_scans, 1);
}

#[test]
fn complexity_budget_relation_structural_invariants_are_touched_slot_bounded() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let target = create_entity(&mut runtime, "target");

    runtime.performance_access().reset_counters();
    let _ = create_relation(&mut runtime, source, target, "r0");
    let counters = runtime.performance_access().counters();

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

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate").push(MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("dup".to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        ))),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::DuplicateRelationIdentity
    ));
    assert_eq!(counters.relation_identity_candidates_scanned, 1);
}

#[test]
fn complexity_budget_unique_entity_invariant_uses_changed_set_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::mutation_sensitive_blocking(
            InvariantRule::UniqueEntityPayloadField("name".to_string()),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.index_authority().rebuild_unique_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: target,
                payload: RecordPayload::StructuredJson(json!({"name":"other"})),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
            if conflict.code == DiagnosticCode::InvariantViolation
    ));
    assert_eq!(counters.invariant_entity_slot_scans, 1);
    assert_eq!(counters.invariant_entity_records_materialized, 0);
}

#[test]
fn complexity_budget_commit_boundary_unique_invariant_uses_merged_plan_lookup() {
    let mut runtime = runtime_with_test_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::UniqueEntityPayloadField("name".to_string()),
        )],
        ..InvariantCatalog::default()
    });
    let target = create_entity(&mut runtime, "target");
    let _other = create_entity(&mut runtime, "other");
    runtime.index_authority().rebuild_unique_field_indexes();

    runtime.performance_access().reset_counters();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-name").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: target,
                payload: RecordPayload::StructuredJson(json!({"name":"other"})),
            }),
        )),
    );
    let error = txn.commit().unwrap_err();
    let counters = runtime.performance_access().counters();

    assert!(matches!(
        error,
        TransactionCommitError::Conflict { error: ref conflict, .. }
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

    runtime.performance_access().reset_counters();
    let entity = create_entity(&mut runtime, "target");
    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, entity, "target-updated");
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.full_state_clones, 0);
    assert_eq!(counters.partitions_cloned, 0);
    assert_eq!(counters.entity_slots_cloned, 0);
    assert_eq!(counters.relation_slots_cloned, 0);
}
