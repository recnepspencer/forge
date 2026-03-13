use crate::facade::diagnostics::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayFailureClass, ReplayMismatchClass,
    ReplayObservableSurface,
};
use crate::tests::support::*;

// CONTRACT: replay
// LANES: success, failure, determinism

#[test]
fn replay_contract_success_reproduces_canonical_surfaces() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        replay.reconstructed_parent_chain,
        vec![outcome.commit.commit_id]
    );
    assert!(runtime
        .publication_access()
        .diagnostics()
        .by_scope(DiagnosticsScope::Replay)
        .iter()
        .any(|artifact| artifact.kind == DiagnosticsArtifactKind::Comparison));
}

#[test]
fn replay_contract_failure_wrong_branch_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("wrong".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::BranchMismatch));
}

#[test]
fn replay_contract_failure_missing_parent_chain_is_explicit() {
    let mut runtime = runtime_with_test_schema();
    let parent = create_entity_outcome(&mut runtime, "parent");
    let child = create_entity_outcome(&mut runtime, "child");

    assert!(runtime
        .history_authority()
        .remove_commit_envelope_for_test(parent.commit.commit_id));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: child.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::MissingParentChain));
}

#[test]
fn replay_contract_success_preserves_merge_parent_order() {
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
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert!(runtime.replay_access().compare_outcome(&replay));
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .commit
            .parents,
        vec![main.commit.commit_id, feature.commit.commit_id]
    );
    assert_eq!(
        runtime
            .replay_access()
            .canonical_commit_envelope(merge.commit.commit_id)
            .unwrap()
            .merge_base_commits,
        vec![main.commit.commit_id]
    );
    assert!(replay
        .compared_surfaces
        .contains(&ReplayObservableSurface::History));
}

#[test]
fn replay_contract_reports_structured_patch_drift_when_canonical_envelope_is_tampered() {
    let mut runtime = runtime_with_test_schema();
    let outcome = create_entity_outcome(&mut runtime, "replayable");
    assert!(runtime.history_authority().tamper_commit_patch_for_test(
        outcome.commit.commit_id,
        |patch| {
            patch.records[0].detail =
                PatchDetail::StructuredJson(serde_json::json!({"tampered": true}));
        }
    ));

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
        });

    assert_eq!(replay.failure, Some(ReplayFailureClass::ObservableMismatch));
    assert_eq!(replay.mismatches.len(), 1);
    assert_eq!(replay.mismatches[0].class, ReplayMismatchClass::PatchDrift);
    assert_eq!(replay.mismatches[0].surface, ReplayObservableSurface::Patch);
    assert!(replay.mismatches[0].expected.is_some());
    assert!(replay.mismatches[0].observed.is_some());
}
