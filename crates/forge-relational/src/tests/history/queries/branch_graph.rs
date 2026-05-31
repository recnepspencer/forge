use crate::facade::history::BranchCreateErrorClass;
use crate::tests::support::*;

#[test]
fn branch_creation_and_branch_targeted_commits_build_a_version_graph() {
    let mut runtime = runtime_with_test_schema();
    let main_outcome = create_entity_outcome(&mut runtime, "main-a");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature_outcome =
        create_entity_outcome_on_branch(&mut runtime, "feature-a", BranchId("feature".to_string()));
    let main_second =
        create_entity_outcome_on_branch(&mut runtime, "main-b", BranchId("main".to_string()));
    let graph = runtime.history().version_graph();

    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("feature".to_string()))
            .unwrap(),
        &feature_outcome.commit
    );
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_string()))
            .unwrap(),
        &main_second.commit
    );
    assert_eq!(
        feature_outcome.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(
        main_second.commit.parents,
        vec![main_outcome.commit.commit_id]
    );
    assert_eq!(graph.branches.len(), 2);
    assert_eq!(graph.commits.len(), 3);
}

#[test]
fn branch_history_helpers_expose_ancestor_and_merge_base_reasoning() {
    let mut runtime = runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let chain = runtime
        .history()
        .ancestor_closure_by_commit_id_order(feature.commit.commit_id);
    let merge_base = runtime.history().latest_common_ancestor_between_branches(
        &BranchId("main".to_string()),
        &BranchId("feature".to_string()),
    );

    assert_eq!(chain, vec![main.commit.commit_id, feature.commit.commit_id]);
    assert_eq!(merge_base, Some(main.commit.commit_id));
    assert!(runtime.history().can_merge_branch_into(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string())
    ));
}

#[test]
fn duplicate_branch_creation_is_rejected() {
    let mut runtime = runtime_with_test_schema();
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let error = runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap_err();

    assert_eq!(error.class, BranchCreateErrorClass::BranchAlreadyExists);
}
