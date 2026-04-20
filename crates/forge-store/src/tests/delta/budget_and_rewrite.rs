use super::*;

#[test]
fn direct_read_budget_rejects_too_deep_layer_stack() {
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
    for index in 0..=MAX_DIRECT_LAYER_READ_DEPTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("feature-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let error = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReadBudgetExceeded);
}

#[test]
fn rewrite_planning_returns_contiguous_segment_when_width_is_admitted() {
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
            &format!("rewrite-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let plan = store
        .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        plan.strategy(),
        BranchDeltaRewriteStrategy::ReplaceContiguousSegment
    );
    assert_eq!(plan.segment().unwrap().layer_ids().len(), 2);
}

#[test]
fn rewrite_budget_rejects_too_wide_segment() {
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
    for index in 0..=MAX_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rewrite-wide-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let error = store
        .plan_delta_rewrite(BranchDeltaRewriteRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::BranchDeltaRewriteBudgetExceeded
    );
}

#[test]
fn rewrite_recommendation_defers_below_recommended_width() {
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
    for index in 0..(RECOMMENDED_REWRITE_LAYER_WIDTH - 1) {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rewrite-policy-defer-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let recommendation = store
        .recommend_delta_rewrite(BranchDeltaRewriteRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        recommendation.decision,
        BranchDeltaRewritePolicyDecision::Defer
    );
    assert_eq!(
        recommendation.plan.strategy(),
        BranchDeltaRewriteStrategy::ReplaceContiguousSegment
    );
}

#[test]
fn rewrite_recommendation_compacts_at_recommended_width() {
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
    for index in 0..RECOMMENDED_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("rewrite-policy-compact-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    let recommendation = store
        .recommend_delta_rewrite(BranchDeltaRewriteRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        recommendation.decision,
        BranchDeltaRewritePolicyDecision::CompactNow
    );
    assert_eq!(
        recommendation.plan.rewrite_breadth(),
        RECOMMENDED_REWRITE_LAYER_WIDTH
    );
}
