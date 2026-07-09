use super::*;

#[test]
fn shared_base_branch_creation_reuses_basis_without_copy() {
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

    let receipt = store
        .create_shared_base_branch(SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    let feature_head = store.fetch_branch_head(&feature_branch).unwrap();
    let counters = store.counters();

    assert_eq!(receipt.branch_id, feature_branch);
    assert_eq!(receipt.source_branch_id, main_branch);
    assert_eq!(
        receipt.source_frontier_commit_id,
        Some(main_head.commit.commit_id)
    );
    assert_eq!(
        feature_head.head_commit_id(),
        Some(main_head.commit.commit_id)
    );
    assert_eq!(counters.branch_base_reuse_count, 1);
    assert_eq!(counters.branch_base_copy_count, 0);
    assert_eq!(counters.branch_hidden_full_base_materialization_count, 0);
}

#[test]
fn shared_base_creation_admission_returns_witness() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let main_branch = root.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());

    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root.clone()).unwrap();

    let witness = store
        .admit_shared_base_branch_creation(SharedBaseBranchCreationRequest::new(
            feature_branch,
            main_branch,
        ))
        .unwrap();

    assert_eq!(
        witness.source_frontier_commit_id(),
        Some(root.commit.commit_id)
    );
    assert!(!witness.authority_basis_digest().is_empty());
}

#[test]
fn branch_delta_read_plan_uses_direct_layers_for_shared_base_branch() {
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
        "feature-only",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);
    store
        .append_canonical_commit(feature_commit.clone())
        .unwrap();

    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            feature_commit.commit.commit_id,
        ))
        .unwrap();

    assert_eq!(plan.strategy, BranchDeltaReadStrategy::DirectLayerRead);
    assert_eq!(
        plan.performance.fallback_class,
        BranchDeltaFallbackClass::None
    );
    assert_eq!(
        plan.performance.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(plan.performance.replay_commit_count, 1);
    assert_eq!(plan.used_layer_ids.len(), 1);
    assert_eq!(plan.commit_ids, vec![feature_commit.commit.commit_id]);
}

#[test]
fn same_branch_descendant_admission_returns_witness() {
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
        "feature-only",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);
    store
        .append_canonical_commit(feature_commit.clone())
        .unwrap();

    let witness = store
        .admit_same_branch_descendant(BranchDeltaReadRequest::new(
            feature_branch,
            feature_commit.commit.commit_id,
        ))
        .unwrap();

    assert_eq!(witness.commit_ids(), &[feature_commit.commit.commit_id]);
}
