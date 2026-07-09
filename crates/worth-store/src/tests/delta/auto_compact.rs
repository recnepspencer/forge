use super::*;

#[test]
fn auto_compact_branch_delta_defers_below_recommended_width() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    for index in 0..(RECOMMENDED_REWRITE_LAYER_WIDTH - 1) {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("auto-compact-defer-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let outcome = store
        .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        outcome.disposition,
        BranchDeltaAutoCompactDisposition::Deferred
    );
    assert!(outcome.rewrite_receipt.is_none());
    assert_eq!(
        plan.used_layer_ids.len(),
        RECOMMENDED_REWRITE_LAYER_WIDTH - 1
    );
}

#[test]
fn auto_compact_branch_delta_executes_at_recommended_width() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
    for index in 0..RECOMMENDED_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("auto-compact-now-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let outcome = store
        .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();
    let counters = store.counters();

    assert_eq!(
        outcome.disposition,
        BranchDeltaAutoCompactDisposition::Compacted
    );
    assert!(outcome.rewrite_receipt.is_some());
    assert_eq!(plan.used_layer_ids.len(), 1);
    assert_eq!(counters.branch_delta_rewrite_count, 1);
}

#[test]
fn auto_compact_branch_delta_reports_no_action_for_single_layer() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
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
        "auto-compact-single",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);
    let target_commit_id = feature_commit.commit.commit_id;
    store.append_canonical_commit(feature_commit).unwrap();

    let outcome = store
        .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        outcome.disposition,
        BranchDeltaAutoCompactDisposition::NoAction
    );
    assert!(outcome.rewrite_receipt.is_none());
    assert_eq!(plan.used_layer_ids.len(), 1);
}

#[test]
fn auto_compact_branch_delta_rejects_missing_layer_path_as_too_broad() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("worth-store-delta-auto-compact-gap");

    let target_commit_id = {
        let mut store = WORTHStoreBuilder::new()
            .local_file(path.clone())
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

        let mut target_commit_id = main_head.commit.commit_id;
        for index in 0..RECOMMENDED_REWRITE_LAYER_WIDTH {
            update_entity_on_branch(
                &mut runtime,
                entity_id,
                &format!("auto-compact-gap-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        target_commit_id
    };

    force_remove_first_branch_delta_layer(&path, "feature");
    let mut store = WORTHStoreBuilder::new().local_file(path).build().unwrap();
    let outcome = store
        .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        outcome.disposition,
        BranchDeltaAutoCompactDisposition::RejectedAsTooBroad
    );
    assert!(outcome.rewrite_receipt.is_none());
    assert_eq!(
        outcome.recommendation.decision,
        BranchDeltaRewritePolicyDecision::RejectAsTooBroad
    );
}
