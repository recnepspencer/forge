use crate::facade::{
    BranchId, DiagnosticsArtifactKind, DiagnosticsScope, RelationalReplayRequest,
    ReplayExecutionMode, ReplayFailureClass,
};

// CONTRACT: replay
// LANES: success, failure, determinism

#[test]
fn replay_contract_success_reproduces_canonical_surfaces() {
    let mut runtime = super::runtime_with_test_schema();
    let outcome = super::create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime.replay_commit(RelationalReplayRequest {
        commit_id: outcome.commit.commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
    });

    assert!(runtime.compare_replay_outcome(&replay));
    assert_eq!(
        replay.reconstructed_parent_chain,
        vec![outcome.commit.commit_id]
    );
    assert!(runtime
        .diagnostics()
        .by_scope(DiagnosticsScope::Replay)
        .iter()
        .any(|artifact| artifact.kind == DiagnosticsArtifactKind::Comparison));
}

#[test]
fn replay_contract_failure_wrong_branch_is_explicit() {
    let mut runtime = super::runtime_with_test_schema();
    let outcome = super::create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime.replay_commit(RelationalReplayRequest {
        commit_id: outcome.commit.commit_id,
        branch_id: BranchId("wrong".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
    });

    assert_eq!(replay.failure, Some(ReplayFailureClass::BranchMismatch));
}

#[test]
fn replay_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = super::runtime_with_test_schema();
    let parent = super::create_entity_outcome(&mut runtime, "parent");
    let child = super::create_entity_outcome(&mut runtime, "child");

    assert!(runtime.remove_commit_envelope_for_test(parent.commit.commit_id));

    let replay = runtime.replay_commit(RelationalReplayRequest {
        commit_id: child.commit.commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
    });

    assert_eq!(replay.failure, Some(ReplayFailureClass::MissingParentChain));
}

#[test]
fn replay_contract_success_preserves_merge_parent_order() {
    let mut runtime = super::runtime_with_test_schema();
    let main = super::create_entity_outcome(&mut runtime, "main");
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let feature = super::create_entity_outcome_on_branch(
        &mut runtime,
        "feature",
        BranchId("feature".to_string()),
    );
    let merge = super::merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let replay = runtime.replay_commit(RelationalReplayRequest {
        commit_id: merge.commit.commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
    });

    assert!(runtime.compare_replay_outcome(&replay));
    assert_eq!(
        runtime
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .commit
            .parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        runtime
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .merge_base_commits,
        vec![main.commit.commit_id]
    );
    assert!(replay
        .compared_surfaces
        .contains(&crate::facade::ReplayObservableSurface::History));
}
