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


#[path = "milestone_5_certification/suite.rs"]
mod suite;
#[path = "milestone_5_certification/cases.rs"]
mod cases;

use suite::*;
