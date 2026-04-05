use serde::Serialize;

use crate::facade::diagnostics::{DiagnosticCode, DiagnosticsScope};
use crate::facade::history::BranchId;
use crate::facade::replay::{RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode};
use crate::tests::support::*;

/// Canonical proof artifact for 7A parent-list parity across publication,
/// replay, and durability recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ParentListSerializationArtifact {
    root_commit_id: u64,
    root_parents: Vec<u64>,
    linear_commit_id: u64,
    linear_parents: Vec<u64>,
    feature_commit_id: u64,
    feature_parents: Vec<u64>,
    merge_ready_commit_id: u64,
    merge_ready_parents: Vec<u64>,
    replayed_merge_ready_parents: Vec<u64>,
    recovered_merge_ready_parents: Vec<u64>,
}

/// Canonical proof artifact for 7A ancestry and branch-reasoning scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AncestryQueryMatrix {
    pre_merge_common_ancestor_commit_id: Option<u64>,
    post_merge_common_ancestor_commit_id: Option<u64>,
    merge_ready_commit_ancestor_closure: Vec<u64>,
    feature_head_ancestor_closure: Vec<u64>,
    main_head_commit_id: u64,
    main_head_ancestor_closure: Vec<u64>,
    inspected_merge_base_commit_id: Option<u64>,
    feature_only_commit_closure: Vec<u64>,
    main_only_commit_closure: Vec<u64>,
    can_merge_feature_into_main: bool,
}

/// Harness wrapper for 7A proof artifacts and derived digest evidence.
///
/// This bundle is not runtime authority. It certifies canonical runtime
/// surfaces that remain authoritative elsewhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct MergeReadyHistoryCertificationBundle {
    parent_list_serialization: ParentListSerializationArtifact,
    ancestry_query_matrix: AncestryQueryMatrix,
    parent_list_digest: String,
    ancestry_query_digest: String,
    replay_acceptance_digest: String,
    durability_parity_digest: String,
    diagnostics_digest: String,
    branch_reasoning_digest: String,
}

fn authoritative_parent_ids(
    runtime: &RelationalRuntime,
    commit_id: crate::facade::history::CommitId,
) -> Vec<u64> {
    runtime
        .replay()
        .canonical_commit_envelope(commit_id)
        .unwrap()
        .commit
        .ordered_parents()
        .as_slice()
        .iter()
        .map(|parent| parent.0)
        .collect()
}

fn commit_closure_ids(
    runtime: &RelationalRuntime,
    commit_id: crate::facade::history::CommitId,
) -> Vec<u64> {
    runtime
        .history()
        .ancestor_closure_by_commit_id_order(commit_id)
        .into_iter()
        .map(|commit| commit.0)
        .collect()
}

fn run_merge_ready_history_shape_certification() -> MergeReadyHistoryCertificationBundle {
    let mut runtime = persisted_runtime_with_test_schema();
    let root = create_entity_outcome(&mut runtime, "root");
    let linear = create_entity_outcome(&mut runtime, "linear");
    create_branch_from_main(&mut runtime, "feature");
    let feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));

    let pre_merge_common_ancestor = runtime
        .history()
        .latest_common_ancestor_between_branches(
            &BranchId("main".to_string()),
            &BranchId("feature".to_string()),
        )
        .map(|commit| commit.0);

    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );
    let post_merge_main = create_entity_outcome(&mut runtime, "main-post-merge");

    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });

    let (_recovery, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let post_merge_inspection = recovered.history().inspect_merge(
        &BranchId("feature".to_string()),
        &BranchId("main".to_string()),
    );

    let parent_list_serialization = ParentListSerializationArtifact {
        root_commit_id: root.commit.commit_id.0,
        root_parents: authoritative_parent_ids(&runtime, root.commit.commit_id),
        linear_commit_id: linear.commit.commit_id.0,
        linear_parents: authoritative_parent_ids(&runtime, linear.commit.commit_id),
        feature_commit_id: feature.commit.commit_id.0,
        feature_parents: authoritative_parent_ids(&runtime, feature.commit.commit_id),
        merge_ready_commit_id: merge.commit.commit_id.0,
        merge_ready_parents: authoritative_parent_ids(&runtime, merge.commit.commit_id),
        replayed_merge_ready_parents: replay
            .commit
            .as_ref()
            .map(|commit| {
                commit
                    .ordered_parents()
                    .as_slice()
                    .iter()
                    .map(|parent| parent.0)
                    .collect()
            })
            .unwrap_or_default(),
        recovered_merge_ready_parents: authoritative_parent_ids(&recovered, merge.commit.commit_id),
    };

    let ancestry_query_matrix = AncestryQueryMatrix {
        pre_merge_common_ancestor_commit_id: pre_merge_common_ancestor,
        post_merge_common_ancestor_commit_id: recovered
            .history()
            .latest_common_ancestor_between_branches(
                &BranchId("main".to_string()),
                &BranchId("feature".to_string()),
            )
            .map(|commit| commit.0),
        merge_ready_commit_ancestor_closure: commit_closure_ids(&recovered, merge.commit.commit_id),
        feature_head_ancestor_closure: commit_closure_ids(&recovered, feature.commit.commit_id),
        main_head_commit_id: post_merge_main.commit.commit_id.0,
        main_head_ancestor_closure: commit_closure_ids(
            &recovered,
            post_merge_main.commit.commit_id,
        ),
        inspected_merge_base_commit_id: post_merge_inspection.merge_base.map(|commit| commit.0),
        feature_only_commit_closure: post_merge_inspection
            .source_only_commits
            .iter()
            .map(|commit| commit.0)
            .collect(),
        main_only_commit_closure: post_merge_inspection
            .target_only_commits
            .iter()
            .map(|commit| commit.0)
            .collect(),
        can_merge_feature_into_main: post_merge_inspection.can_merge,
    };

    let publication_diagnostics = runtime
        .publication()
        .diagnostics()
        .by_scope(DiagnosticsScope::PatchPublication)
        .iter()
        .flat_map(|artifact| artifact.entries.iter())
        .filter(|entry| {
            matches!(
                entry.code,
                DiagnosticCode::MergeCommitPublished | DiagnosticCode::MergeBaseResolved
            )
        })
        .map(|entry| {
            (
                format!("{:?}", entry.code),
                entry.message.clone(),
                entry.fields.clone(),
            )
        })
        .collect::<Vec<_>>();

    MergeReadyHistoryCertificationBundle {
        parent_list_digest: certification_digest(&parent_list_serialization),
        ancestry_query_digest: certification_digest(&ancestry_query_matrix),
        replay_acceptance_digest: certification_digest(&(
            replay.failure.as_ref(),
            &replay.reconstructed_commit_closure,
            replay.commit.as_ref().map(|commit| {
                commit
                    .ordered_parents()
                    .as_slice()
                    .iter()
                    .map(|parent| parent.0)
                    .collect::<Vec<_>>()
            }),
            &replay.mismatches,
        )),
        durability_parity_digest: certification_digest(&(
            parent_list_serialization.merge_ready_parents.clone(),
            parent_list_serialization
                .recovered_merge_ready_parents
                .clone(),
        )),
        diagnostics_digest: certification_digest(&publication_diagnostics),
        branch_reasoning_digest: certification_digest(&(
            ancestry_query_matrix.pre_merge_common_ancestor_commit_id,
            ancestry_query_matrix.post_merge_common_ancestor_commit_id,
            ancestry_query_matrix.inspected_merge_base_commit_id,
            ancestry_query_matrix.feature_only_commit_closure.clone(),
            ancestry_query_matrix.main_only_commit_closure.clone(),
            ancestry_query_matrix.can_merge_feature_into_main,
        )),
        parent_list_serialization,
        ancestry_query_matrix,
    }
}

#[test]
fn merge_ready_history_shape_test() {
    let certification = run_merge_ready_history_shape_certification();

    assert_eq!(
        certification.parent_list_serialization.root_parents,
        Vec::<u64>::new()
    );
    assert_eq!(
        certification.parent_list_serialization.linear_parents,
        vec![certification.parent_list_serialization.root_commit_id]
    );
    assert_eq!(
        certification.parent_list_serialization.feature_parents,
        vec![certification.parent_list_serialization.linear_commit_id]
    );
    assert_eq!(
        certification.parent_list_serialization.merge_ready_parents,
        vec![
            certification.parent_list_serialization.linear_commit_id,
            certification.parent_list_serialization.feature_commit_id,
        ]
    );
    assert_eq!(
        certification
            .parent_list_serialization
            .replayed_merge_ready_parents,
        certification.parent_list_serialization.merge_ready_parents
    );
    assert_eq!(
        certification
            .parent_list_serialization
            .recovered_merge_ready_parents,
        certification.parent_list_serialization.merge_ready_parents
    );

    assert_eq!(
        certification
            .ancestry_query_matrix
            .pre_merge_common_ancestor_commit_id,
        Some(certification.parent_list_serialization.linear_commit_id)
    );
    assert_eq!(
        certification
            .ancestry_query_matrix
            .post_merge_common_ancestor_commit_id,
        Some(certification.parent_list_serialization.feature_commit_id)
    );
    assert_eq!(
        certification
            .ancestry_query_matrix
            .inspected_merge_base_commit_id,
        Some(certification.parent_list_serialization.feature_commit_id)
    );
    assert_eq!(
        certification
            .ancestry_query_matrix
            .merge_ready_commit_ancestor_closure,
        vec![
            certification.parent_list_serialization.root_commit_id,
            certification.parent_list_serialization.linear_commit_id,
            certification.parent_list_serialization.feature_commit_id,
            certification
                .parent_list_serialization
                .merge_ready_commit_id,
        ]
    );
    assert_eq!(
        certification.ancestry_query_matrix.main_head_commit_id,
        *certification
            .ancestry_query_matrix
            .main_head_ancestor_closure
            .last()
            .unwrap()
    );
    assert_ne!(
        certification
            .ancestry_query_matrix
            .main_head_ancestor_closure,
        certification
            .ancestry_query_matrix
            .merge_ready_commit_ancestor_closure
    );
    assert!(certification.parent_list_digest.len() > 8);
    assert!(certification.ancestry_query_digest.len() > 8);
    assert!(certification.replay_acceptance_digest.len() > 8);
    assert!(certification.durability_parity_digest.len() > 8);
    assert!(certification.diagnostics_digest.len() > 8);
    assert!(certification.branch_reasoning_digest.len() > 8);
}

#[test]
fn merge_ready_history_shape_reports_counter_breadth_explicitly() {
    let mut runtime = persisted_runtime_with_test_schema();
    let _root = create_entity_outcome(&mut runtime, "root");
    let _linear = create_entity_outcome(&mut runtime, "linear");
    create_branch_from_main(&mut runtime, "feature");
    let _feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));
    let merge = merge_commit_from_branches(
        &mut runtime,
        BranchId("main".to_string()),
        vec![BranchId("feature".to_string())],
    );

    runtime.performance_access().reset_counters();

    let _ = runtime
        .history()
        .ancestor_closure_by_commit_id_order(merge.commit.commit_id);
    let replay = runtime
        .replay_authority()
        .replay_commit(RelationalReplayRequest {
            commit_id: merge.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            execution_mode: ReplayExecutionMode::SerialDeterministic,
            verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
        });
    assert!(replay.failure.is_none(), "{:?}", replay);

    let runtime_counters = runtime.performance_access().counters();
    assert!(runtime_counters.merge_history_ancestry_traversals >= 1);
    assert!(runtime_counters.merge_history_ancestry_nodes_visited >= 4);
    assert!(runtime_counters.merge_history_parent_comparisons >= 2);
    assert!(runtime_counters.merge_history_replay_planning_nodes_visited >= 4);
    assert!(runtime_counters.merge_history_replay_parent_checks >= 4);

    let recovery_plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(recovery_plan)
        .unwrap();
    let recovered_counters = recovered.performance_access().counters();
    assert!(recovered_counters.merge_history_durability_validation_nodes_visited >= 4);
    assert!(recovered_counters.merge_history_durability_parent_checks >= 4);
    assert!(recovered_counters.merge_history_parent_comparisons >= 2);
}
