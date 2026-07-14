use super::*;

#[test]
fn durability_contract_recovery_rebuilds_branch_pinned_retention_from_branch_heads() {
    let mut runtime = persisted_runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_entity = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_entity = changed_entities(&target)[0];
    let _relation = create_relation_outcome(&mut runtime, source_entity, target_entity, "r1");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _deleted = delete_entity(&mut runtime, source_entity);
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let retention = recovered.retention().inspect_plan();
    assert_eq!(retention.active_snapshot_count, 0);
    assert!(retention.branch_pinned_entities >= 1);
    assert!(retention.branch_pinned_relations >= 1);
    assert_eq!(retention.reclaimable_entities, 0);
    assert_eq!(retention.reclaimable_relations, 0);
}

#[test]
fn durability_contract_recovery_preserves_inspection_truth_bundle() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _main_update = update_entity(&mut runtime, entity, "main");
    let _feature_update = {
        let mut txn = runtime.begin_transaction(TransactionOptions {
            target_branch: Some(BranchId("feature".to_string())),
            ..TransactionOptions::default()
        });
        txn.push_batch(
            WorkerIntentBatch::new("feature-update").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: crate::tests::support::single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        crate::tests::support::field_key("name"),
                        "feature",
                    ),
                }),
            )),
        );
        txn.commit().unwrap()
    };
    runtime.durability_authority().checkpoint().unwrap();
    let expected = capture_inspection_truth_bundle(
        &runtime,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let actual = capture_inspection_truth_bundle(
        &recovered,
        &BranchId("feature".to_string()),
        entity,
        created.version_id,
    );

    assert_eq!(expected, actual);
}

#[test]
fn durability_contract_live_branch_pin_counts_match_branch_head_membership() {
    let mut runtime = persisted_runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let inspection = runtime.inspect_what_happened();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after create")
            .pins
            .branch_pins,
        1
    );

    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let inspection = runtime.inspect_what_happened();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after branch create")
            .pins
            .branch_pins,
        2
    );

    update_entity(&mut runtime, entity, "main");
    let inspection = runtime.inspect_what_happened();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after main update")
            .pins
            .branch_pins,
        2
    );

    update_entity_on_branch(
        &mut runtime,
        entity,
        "feature",
        BranchId("feature".to_string()),
    );
    let inspection = runtime.inspect_what_happened();
    assert_eq!(
        inspection
            .inspect_record_retention(RecordRef::Entity(entity))
            .expect("entity retention after feature update")
            .pins
            .branch_pins,
        2
    );
}

#[test]
fn durability_contract_persisted_commit_fails_closed_when_store_path_is_not_directory() {
    let root_path = unique_test_store_path("worth-relational-bad-store");
    std::fs::write(&root_path, b"not-a-directory").unwrap();
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: root_path.clone(),
            segment_commit_capacity: 2,
        })
        .build();

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("fail-closed"));
    let error = txn.commit().unwrap_err();

    assert!(matches!(error, TransactionCommitError::Publication { .. }));
    assert!(runtime.history().latest_commit().is_none());
}
