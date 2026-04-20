use super::*;

#[test]
fn direct_layer_read_reconstructs_authoritative_branch_truth() {
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
    update_entity_on_branch(
        &mut runtime,
        entity_id,
        "feature-one",
        Some(feature_branch.clone()),
    );
    let feature_one = latest_envelope(&runtime);
    store.append_canonical_commit(feature_one.clone()).unwrap();

    update_entity_on_branch(
        &mut runtime,
        entity_id,
        "feature-two",
        Some(feature_branch.clone()),
    );
    let feature_two = latest_envelope(&runtime);
    store.append_canonical_commit(feature_two.clone()).unwrap();

    let direct =
        admitted_branch_delta_read(&store, feature_branch.clone(), feature_two.commit.commit_id);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            feature_branch.clone(),
            feature_two.commit.commit_id,
        ))
        .unwrap();
    let authoritative = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            feature_two.commit.commit_id,
        ))
        .unwrap();
    let counters = store.counters();

    assert_eq!(
        direct.authoritative_export().canonical_json(),
        authoritative.image.authoritative_export().canonical_json()
    );
    assert_eq!(direct.plan.used_layer_ids.len(), 2);
    assert_eq!(
        direct.plan.performance.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(direct.plan.performance.replay_commit_count, 2);
    assert_eq!(counters.branch_delta_replay_commit_count, 2);
}

#[test]
fn branch_delta_read_rejects_non_branch_target() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let feature_branch = BranchId("feature".to_string());
    let source_branch = root.branch_context.clone();
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let basis_frontier = latest_envelope(&runtime);

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root.clone()).unwrap();
    store
        .append_canonical_commit(basis_frontier.clone())
        .unwrap();
    store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            source_branch.clone(),
        ))
        .unwrap();
    update_entity_on_branch(&mut runtime, entity_id, "gamma", None);
    let unrelated_target = latest_envelope(&runtime);
    store
        .append_canonical_commit(unrelated_target.clone())
        .unwrap();

    let error = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            unrelated_target.commit.commit_id,
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReadTargetIllegal);
}

#[test]
fn sqlite_backend_reloads_shared_base_and_delta_layer_records() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_sqlite_path("forge-store-delta");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "feature-only",
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let feature_head = store.fetch_branch_head(&feature_branch).unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch.clone(),
            feature_head.head_commit_id().unwrap(),
        ))
        .unwrap();
    let direct = admitted_branch_delta_read(
        &store,
        feature_branch,
        feature_head.head_commit_id().unwrap(),
    );

    assert_eq!(plan.strategy, BranchDeltaReadStrategy::DirectLayerRead);
    assert_eq!(plan.used_layer_ids.len(), 1);
    assert_eq!(
        plan.performance.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        direct.authoritative_export().branch_head_records[0].head_commit_id,
        feature_head.head_commit_id()
    );
}
