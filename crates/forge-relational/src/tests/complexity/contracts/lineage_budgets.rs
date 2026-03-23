use crate::facade::history::BranchId;
use crate::facade::lineage::{HistoricalResolutionRequest, LineageDivergenceRequest};
use crate::facade::replay::{
    RelationalReplayRequest, ReplayAuthorityBasisKind, ReplayExecutionMode, ReplayVerificationMode,
};
use crate::tests::support::*;

#[test]
fn complexity_budget_lineage_historical_resolution_reports_branch_scoped_work() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;

    runtime.performance_access().reset_counters();
    let resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(HistoricalResolutionRequest {
            branch_id: BranchId("main".to_string()),
            lineage_id: start_lineage,
        });
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.lineage_historical_resolution_requests, 1);
    assert_eq!(
        counters.lineage_historical_resolution_branch_event_scans,
        resolution.metrics.branch_event_scan_count
    );
    assert_eq!(
        counters.lineage_historical_resolution_traversed_events,
        resolution.metrics.traversed_event_count
    );
}

#[test]
fn complexity_budget_lineage_branch_divergence_reports_breadth() {
    let mut runtime = runtime_with_test_schema();
    let _main = create_entity_outcome(&mut runtime, "main");
    runtime
        .history_authority()
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();
    let _feature =
        create_entity_outcome_on_branch(&mut runtime, "feature", BranchId("feature".to_string()));

    runtime.performance_access().reset_counters();
    let divergence = runtime
        .lineage_access()
        .divergence_between_branches(LineageDivergenceRequest {
            left_branch: BranchId("main".to_string()),
            right_branch: BranchId("feature".to_string()),
        });
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.lineage_branch_divergence_requests, 1);
    assert_eq!(
        counters.lineage_branch_divergence_event_scans,
        divergence.metrics.left_event_count + divergence.metrics.right_event_count
    );
    assert_eq!(
        counters.lineage_branch_divergence_node_scans,
        divergence.metrics.left_node_count + divergence.metrics.right_node_count
    );
    assert!(counters.lineage_graph_snapshot_requests >= 2);
    assert_eq!(
        counters.lineage_graph_snapshot_nodes_materialized,
        divergence.metrics.left_node_count + divergence.metrics.right_node_count
    );
    assert!(
        counters.visible_entity_records_materialized
            >= counters.lineage_graph_snapshot_nodes_materialized
    );
    assert!(
        counters.visibility_cache_hits + counters.visibility_cache_miss_reconstructions >= 2
    );
    assert_eq!(
        counters.lineage_graph_snapshot_visibility_cache_hits
            + counters.lineage_graph_snapshot_visibility_cache_miss_reconstructions,
        counters.lineage_graph_snapshot_requests
    );
    assert!(
        counters.lineage_graph_snapshot_visibility_cache_hits
            + counters.lineage_graph_snapshot_visibility_cache_miss_reconstructions
            >= 2
    );
}

#[test]
fn complexity_budget_replay_lineage_parity_reports_authority_basis_and_digest_width() {
    let mut runtime = runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "source");
    let second = create_entity_outcome(&mut runtime, "target");
    let first_entity = changed_entities(&first)[0];
    let second_entity = changed_entities(&second)[0];
    let first_lineage = runtime.lineage_access().for_record(first_entity).unwrap().lineage_id;
    let second_lineage = runtime.lineage_access().for_record(second_entity).unwrap().lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "lineage-budget-replay",
    );
    let promotion = runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let promoted_commit_id = promotion
        .promoted_commit_id
        .expect("promotion should publish a metadata-only commit");

    runtime.performance_access().reset_counters();
    let durable_replay = runtime.replay_authority().replay_commit(RelationalReplayRequest {
        commit_id: promoted_commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
    });
    let counters = runtime.performance_access().counters();

    assert_eq!(
        durable_replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.kind),
        Some(ReplayAuthorityBasisKind::DurableLogCanonical)
    );
    assert_eq!(counters.replay_lineage_authority_lookup_requests, 1);
    assert_eq!(counters.replay_lineage_log_index_hits, 1);
    assert_eq!(counters.replay_lineage_checkpoint_index_hits, 0);
    assert_eq!(counters.replay_lineage_durable_basis_selections, 1);
    assert_eq!(counters.replay_lineage_history_fallback_basis_selections, 0);
    assert_eq!(counters.replay_lineage_authoritative_basis_rejections, 0);
    assert_eq!(
        counters.replay_lineage_digest_event_width,
        durable_replay
            .lineage_authority_basis
            .as_ref()
            .unwrap()
            .lineage_event_count
    );
    assert_eq!(
        counters.replay_lineage_digest_decision_width,
        durable_replay
            .lineage_authority_basis
            .as_ref()
            .unwrap()
            .lineage_decision_count
    );

    assert!(runtime
        .durability_authority()
        .remove_durable_envelope_for_test(promoted_commit_id));
    runtime.performance_access().reset_counters();
    let fallback_replay = runtime.replay_authority().replay_commit(RelationalReplayRequest {
        commit_id: promoted_commit_id,
        branch_id: BranchId("main".to_string()),
        execution_mode: ReplayExecutionMode::SerialDeterministic,
        verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
    });
    let counters = runtime.performance_access().counters();
    assert_eq!(
        fallback_replay
            .lineage_authority_basis
            .as_ref()
            .map(|basis| basis.kind),
        Some(ReplayAuthorityBasisKind::HistoryEnvelopeFallback)
    );
    assert_eq!(counters.replay_lineage_authority_lookup_requests, 1);
    assert_eq!(counters.replay_lineage_log_index_hits, 0);
    assert_eq!(counters.replay_lineage_checkpoint_index_hits, 0);
    assert_eq!(counters.replay_lineage_durable_basis_selections, 0);
    assert_eq!(counters.replay_lineage_history_fallback_basis_selections, 1);
    assert_eq!(counters.replay_lineage_authoritative_basis_rejections, 0);
}
