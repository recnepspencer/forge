use crate::{BranchDeltaReadRequest, BranchDeltaReadStrategy, ComplexityStatus, ForgeStoreBuilder};

use super::harness::{
    certification::{
        assertions::{assert_all_equal, assert_any_not_equal},
        core::{AssertionClass, CanonicalRow, CertificationSuite, LaneResult},
        requirements::{
            evaluate_completeness, BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST,
        },
    },
    fixtures::{
        runtime::{
            create_entity, latest_envelope, runtime_with_demo_schema, update_entity_on_branch,
        },
        stores::unique_test_sqlite_path,
    },
};
use forge_relational::facade::history::BranchId;

fn no_edit_bundle() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-no-edit".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch,
        ))
        .unwrap();

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            main_head.commit.commit_id,
        ))
        .unwrap()
}

fn small_edit_bundle_in_memory() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-small".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
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
        "feature-small-only",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);
    let target_commit_id = feature_commit.commit.commit_id;
    store.append_canonical_commit(feature_commit).unwrap();

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap()
}

fn small_edit_bundle_sqlite() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-small".to_string());
    let path = unique_test_sqlite_path("forge-store-m5-small");

    let mut store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
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
        "feature-small-only",
        Some(feature_branch.clone()),
    );
    let feature_commit = latest_envelope(&runtime);
    let target_commit_id = feature_commit.commit.commit_id;
    store.append_canonical_commit(feature_commit).unwrap();

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap()
}

fn deep_edit_bundle() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-deep".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..crate::RECOMMENDED_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("feature-deep-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap()
}

fn rewritten_bundle_in_memory() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-rewrite".to_string());

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..crate::RECOMMENDED_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("feature-rewrite-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }
    store
        .auto_compact_branch_delta(crate::BranchDeltaRewriteRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap()
}

fn rewritten_bundle_sqlite() -> crate::Milestone5CertificationBundle {
    let mut runtime = runtime_with_demo_schema();
    let entity_id = create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    update_entity_on_branch(&mut runtime, entity_id, "beta", None);
    let main_head = latest_envelope(&runtime);
    let main_branch = main_head.branch_context.clone();
    let feature_branch = BranchId("feature-rewrite".to_string());
    let path = unique_test_sqlite_path("forge-store-m5-rewrite");

    let mut store = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    store.append_canonical_commit(root).unwrap();
    store.append_canonical_commit(main_head.clone()).unwrap();
    store
        .create_shared_base_branch(crate::SharedBaseBranchCreationRequest::new(
            feature_branch.clone(),
            main_branch.clone(),
        ))
        .unwrap();
    runtime
        .history_authority()
        .create_branch(feature_branch.clone(), &main_branch)
        .unwrap();

    let mut target_commit_id = main_head.commit.commit_id;
    for index in 0..crate::RECOMMENDED_REWRITE_LAYER_WIDTH {
        update_entity_on_branch(
            &mut runtime,
            entity_id,
            &format!("feature-rewrite-{index}"),
            Some(feature_branch.clone()),
        );
        let feature_commit = latest_envelope(&runtime);
        target_commit_id = feature_commit.commit.commit_id;
        store.append_canonical_commit(feature_commit).unwrap();
    }
    store
        .auto_compact_branch_delta(crate::BranchDeltaRewriteRequest::new(
            feature_branch.clone(),
            target_commit_id,
        ))
        .unwrap();

    store
        .milestone_5_certification_bundle(BranchDeltaReadRequest::new(
            feature_branch,
            target_commit_id,
        ))
        .unwrap()
}

fn milestone_5_suite() -> CertificationSuite<String, String> {
    let no_edit = no_edit_bundle();
    let small_in_memory = small_edit_bundle_in_memory();
    let small_sqlite = small_edit_bundle_sqlite();
    let deep = deep_edit_bundle();
    let rewritten_in_memory = rewritten_bundle_in_memory();
    let rewritten_sqlite = rewritten_bundle_sqlite();

    CertificationSuite::new(BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST.suite_name)
        .with_canonical_row(CanonicalRow::new(
            "backend_variation_parity",
            vec![
                LaneResult::new("in_memory", small_in_memory.canonical_json()),
                LaneResult::new("sqlite", small_sqlite.canonical_json()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "delta_growth_tracks_semantic_delta",
            vec![
                LaneResult::new(
                    "no_edit",
                    serde_json::to_string(&no_edit.delta_storage_report).unwrap(),
                ),
                LaneResult::new(
                    "small_edit",
                    serde_json::to_string(&small_in_memory.delta_storage_report).unwrap(),
                ),
                LaneResult::new(
                    "deep_edit",
                    serde_json::to_string(&deep.delta_storage_report).unwrap(),
                ),
            ],
            &[AssertionClass::Inequality, AssertionClass::ExactCounter],
        ))
        .with_canonical_row(CanonicalRow::new(
            "rewritten_stack_control_lane_parity",
            vec![
                LaneResult::new("in_memory", rewritten_in_memory.canonical_json()),
                LaneResult::new("sqlite", rewritten_sqlite.canonical_json()),
            ],
            &[AssertionClass::Equality, AssertionClass::ExactCounter],
        ))
}

#[test]
fn milestone_5_certification_bundle_proves_no_edit_branch_is_near_free() {
    let bundle = no_edit_bundle();

    assert_eq!(
        bundle.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::EmptyBranchReuse
    );
    assert_eq!(
        bundle.delta_storage_report.control_path.strategy,
        BranchDeltaReadStrategy::AuthorityReplayControl
    );
    assert_eq!(bundle.delta_storage_report.live_layer_count, 0);
    assert_eq!(bundle.counter_snapshot.branch_base_reuse_count, 1);
    assert_eq!(bundle.counter_snapshot.branch_base_copy_count, 0);
    assert_eq!(
        bundle
            .counter_snapshot
            .branch_hidden_full_base_materialization_count,
        0
    );
    assert_eq!(
        bundle.delta_storage_report.direct_path.complexity_status,
        ComplexityStatus::Verified
    );
    assert_eq!(
        bundle.delta_storage_report.control_reference_surface,
        "Milestone7IndependentReference"
    );
}

#[test]
fn milestone_5_certification_bundle_matches_backend_variation_parity() {
    let suite = milestone_5_suite();
    assert_all_equal(&suite.canonical_rows()[0]);
}

#[test]
fn milestone_5_certification_bundle_proves_delta_growth_tracks_semantic_delta() {
    let suite = milestone_5_suite();
    assert_any_not_equal(&suite.canonical_rows()[1]);

    let no_edit = no_edit_bundle();
    let small = small_edit_bundle_in_memory();
    let deep = deep_edit_bundle();

    assert_eq!(no_edit.delta_storage_report.live_layer_count, 0);
    assert_eq!(small.delta_storage_report.live_layer_count, 1);
    assert!(deep.delta_storage_report.live_layer_count >= 3);
    assert_eq!(
        small.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::DirectLayerRead
    );
    assert_eq!(
        deep.delta_storage_report.direct_path.strategy,
        BranchDeltaReadStrategy::DirectLayerRead
    );
}

#[test]
fn milestone_5_certification_bundle_proves_rewritten_stack_control_lane_parity() {
    let suite = milestone_5_suite();
    assert_all_equal(&suite.canonical_rows()[2]);

    let rewritten = rewritten_bundle_in_memory();
    assert_eq!(rewritten.delta_storage_report.live_layer_count, 1);
    assert_eq!(
        rewritten
            .counter_snapshot
            .branch_delta_hidden_full_stack_rewrite_count,
        0
    );
    assert_eq!(rewritten.counter_snapshot.branch_delta_rewrite_count, 1);
}

#[test]
fn milestone_5_certification_suite_is_complete() {
    let suite = milestone_5_suite();
    let completeness =
        evaluate_completeness(&suite, &BRANCH_DELTA_PROPORTIONALITY_AND_REPLAY_PARITY_TEST);
    assert!(completeness.missing_rows().is_empty());
    assert!(completeness.missing_assertion_classes().is_empty());
}
