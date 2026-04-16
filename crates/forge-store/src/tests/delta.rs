use crate::{
    backend::records::StoreState, BranchDeltaAutoCompactDisposition, BranchDeltaFallbackClass,
    BranchDeltaReadRequest, BranchDeltaReadStrategy, BranchDeltaRewritePolicyDecision,
    BranchDeltaRewriteRequest, BranchDeltaRewriteStrategy, ComplexityStatus, ForgeStoreBuilder,
    SharedBaseBranchCreationRequest, SnapshotCaptureRequest, SnapshotReadRequest, StoreErrorKind,
    MAX_DIRECT_LAYER_READ_DEPTH, MAX_REWRITE_LAYER_WIDTH, RECOMMENDED_REWRITE_LAYER_WIDTH,
};
use forge_relational::facade::history::{BranchId, CommitId};

use super::harness::{
    corruption::local_file::{
        force_branch_delta_artifact_commit_mismatch, force_branch_delta_replacement_gap,
        force_branch_delta_replacement_proof_length_drift,
        force_branch_delta_replacement_proof_mismatch,
        force_branch_delta_replacement_self_reference, force_clear_branch_delta_layer_artifacts,
        force_remove_first_branch_delta_layer,
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::{unique_test_sqlite_path, unique_test_store_path},
    },
};

#[test]
fn append_delta_publication_admits_first_layer_for_empty_base_branch() {
    let mut state = StoreState::default();
    let layer_id = state.publish_branch_delta_layer_for_append(
        BranchId("feature-empty".to_string()),
        None,
        CommitId(41),
        vec![CommitId(41)],
    );

    assert_eq!(layer_id, Some(1));
    let record = state
        .branch_delta_layer_records
        .get(&1)
        .expect("first empty-base delta layer should publish");
    assert_eq!(record.base_frontier_commit_id, None);
    assert_eq!(record.commit_ids, vec![CommitId(41)]);
    assert!(record.replacement_lineage_proof.is_empty());
}

fn admitted_branch_delta_read(
    store: &crate::ForgeStore,
    branch_id: BranchId,
    target_commit_id: CommitId,
) -> crate::BranchDeltaReadResult {
    let witness = store
        .admit_same_branch_descendant(BranchDeltaReadRequest::new(branch_id, target_commit_id))
        .unwrap();
    store.read_branch_delta(witness).unwrap()
}

#[test]
fn shared_base_branch_creation_reuses_basis_without_copy() {
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

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
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

#[test]
fn auto_compact_branch_delta_defers_below_recommended_width() {
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
    let path = unique_test_store_path("forge-store-delta-auto-compact-gap");

    let target_commit_id = {
        let mut store = ForgeStoreBuilder::new()
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
    let mut store = ForgeStoreBuilder::new().local_file(path).build().unwrap();
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

#[test]
fn sqlite_rewrite_persists_after_reopen() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_sqlite_path("forge-store-delta-rewrite");

    let target_commit_id = {
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

        let mut target_commit_id = main_head.commit.commit_id;
        for index in 0..2 {
            update_entity_on_branch(
                &mut runtime,
                entity_id,
                &format!("sqlite-rewrite-{index}"),
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
        target_commit_id
    };

    let store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let plan = store
        .plan_branch_delta_read(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap();

    assert_eq!(plan.used_layer_ids.len(), 1);
}

#[test]
fn local_file_reopen_backfills_legacy_empty_branch_delta_artifacts() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-artifact-backfill");

    let target_commit_id = {
        let mut store = ForgeStoreBuilder::new()
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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "legacy-artifact-backfill",
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        let target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
        target_commit_id
    };

    force_clear_branch_delta_layer_artifacts(&path);
    let store = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let direct = admitted_branch_delta_read(&store, feature_branch, target_commit_id);

    assert_eq!(
        direct.plan.performance.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(direct.plan.used_layer_ids.len(), 1);
}

#[test]
fn local_file_reopen_rejects_branch_delta_artifact_commit_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-artifact-mismatch");

    {
        let mut store = ForgeStoreBuilder::new()
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
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "artifact-mismatch",
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        store.append_canonical_commit(feature_commit).unwrap();
    }

    force_branch_delta_artifact_commit_mismatch(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaPublicationGap);
}

#[test]
fn local_file_reopen_rejects_replacement_self_reference_corruption() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-replacement-self");

    {
        let mut store = ForgeStoreBuilder::new()
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
                &format!("replacement-self-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_self_reference(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(
        error.kind(),
        &StoreErrorKind::BranchDeltaShadowAuthorityViolation
    );
}

#[test]
fn local_file_reopen_rejects_replacement_gap_corruption() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-replacement-gap");

    {
        let mut store = ForgeStoreBuilder::new()
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
                &format!("replacement-gap-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_gap(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}

#[test]
fn local_file_reopen_rejects_replacement_lineage_proof_mismatch() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-replacement-proof-mismatch");

    {
        let mut store = ForgeStoreBuilder::new()
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
                &format!("replacement-proof-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_proof_mismatch(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}

#[test]
fn local_file_reopen_rejects_replacement_lineage_proof_length_drift() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_store_path("forge-store-delta-replacement-proof-length-drift");

    {
        let mut store = ForgeStoreBuilder::new()
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
                &format!("replacement-proof-length-{index}"),
                Some(feature_branch.clone()),
            );
            let feature_commit = latest_envelope(&runtime);
            target_commit_id = feature_commit.commit.commit_id;
            store.append_canonical_commit(feature_commit).unwrap();
        }
        store
            .auto_compact_branch_delta(BranchDeltaRewriteRequest::new(
                feature_branch,
                target_commit_id,
            ))
            .unwrap();
    }

    force_branch_delta_replacement_proof_length_drift(&path);
    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .unwrap_err();

    assert_eq!(error.kind(), &StoreErrorKind::BranchDeltaReplacementGap);
}

#[test]
fn sqlite_direct_layer_read_matches_authoritative_snapshot_after_reopen() {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature".to_string());
    let path = unique_test_sqlite_path("forge-store-delta-parity");

    let feature_target_commit_id = {
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
            "feature-one",
            Some(feature_branch.clone()),
        );
        let feature_one = latest_envelope(&runtime);
        store.append_canonical_commit(feature_one).unwrap();

        update_entity_on_branch(
            &mut runtime,
            entity_id,
            "feature-two",
            Some(feature_branch.clone()),
        );
        let feature_two = latest_envelope(&runtime);
        let feature_target_commit_id = feature_two.commit.commit_id;
        store.append_canonical_commit(feature_two).unwrap();
        feature_target_commit_id
    };

    let mut store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let direct =
        admitted_branch_delta_read(&store, feature_branch.clone(), feature_target_commit_id);
    let snapshot = store
        .capture_snapshot(SnapshotCaptureRequest::new(
            feature_branch.clone(),
            feature_target_commit_id,
        ))
        .unwrap();
    let authoritative = store
        .read_snapshot(SnapshotReadRequest::pure_snapshot(
            snapshot.snapshot_id,
            feature_target_commit_id,
        ))
        .unwrap();

    assert_eq!(
        direct.authoritative_export().canonical_json(),
        authoritative.image.authoritative_export().canonical_json()
    );
}
