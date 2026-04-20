use super::*;

#[test]
fn rewrite_execution_compacts_layers_and_preserves_parity() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..2 {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rewrite-exec-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let before = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    assert_eq!(before.used_layer_ids.len(), 2);

    let receipt = store
        .rewrite_branch_delta(
            store
                .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
                    feature_branch.clone(),
                    target_commit_id,
                ))
                .unwrap(),
        )
        .unwrap();
    let after = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let direct = admitted_branch_delta_read(&store, feature_branch.clone(), target_commit_id);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let authoritative = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(receipt.replaced_layer_ids.len(), 2);
    assert!(receipt.replacement_layer_id.is_some());
    assert_eq!(after.used_layer_ids.len(), 1);
    assert_eq!(
        direct.authoritative_export().canonical_json(),
        authoritative.image.authoritative_export().canonical_json()
    );
}

#[test]
fn rewrite_execution_records_rewrite_counters_without_hidden_fallback() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..2 {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rewrite-counter-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    store
        .rewrite_branch_delta(
            store
                .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
                    feature_branch,
                    target_commit_id,
                ))
                .unwrap(),
        )
        .unwrap();
    let counters = store.counters();

    assert_eq!(counters.branch_delta_rewrite_count, 1);
    assert_eq!(counters.branch_delta_rewrite_layers_replaced_count, 2);
    assert_eq!(counters.branch_delta_rewrite_record_count, 2);
    assert_eq!(counters.branch_delta_hidden_full_stack_rewrite_count, 0);
}

#[test]
fn rebuild_from_authority_restores_single_commit_layers_and_preserves_parity() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..2 {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rebuild-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    store
        .rewrite_branch_delta(
            store
                .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
                    feature_branch.clone(),
                    target_commit_id,
                ))
                .unwrap(),
        )
        .unwrap();
    let compacted = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    assert_eq!(compacted.used_layer_ids.len(), 1);

    let receipt = store
        .rebuild_branch_delta_artifacts(feature_branch.clone())
        .unwrap();
    let rebuilt = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let direct = admitted_branch_delta_read(&store, feature_branch.clone(), target_commit_id);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let authoritative = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(receipt.rebuilt_layer_count, 2);
    assert_eq!(rebuilt.used_layer_ids.len(), 2);
    assert_eq!(
        direct.authoritative_export().canonical_json(),
        authoritative.image.authoritative_export().canonical_json()
    );
}

#[test]
fn rebuild_from_authority_records_rebuild_counters() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..2 {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rebuild-counter-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    store
        .rewrite_branch_delta(
            store
                .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
                    feature_branch.clone(),
                    target_commit_id,
                ))
                .unwrap(),
        )
        .unwrap();
    store
        .rebuild_branch_delta_artifacts(feature_branch)
        .unwrap();
    let counters = store.counters();

    assert_eq!(counters.branch_delta_rebuild_count, 1);
    assert_eq!(counters.branch_delta_rebuild_record_count, 2);
}
