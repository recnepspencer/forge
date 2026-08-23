use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::ExecutionFailureContext;
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::replay::{ReplayEvent, ReplayEventDetail, ReplayEventKind};
use crate::logic::transaction::BranchMergeExecutionSummary;
use crate::state::SignalBranchId;

use super::events;
use super::DiagnosticsRecorder;

pub(crate) fn record_branch_fork_lineage(
    graph: &mut SignalGraph,
    created_branch_id: SignalBranchId,
    parent_branch_id: SignalBranchId,
    created_branch_display_name: impl Into<String>,
    parent_branch_display_name: impl Into<String>,
) {
    if !graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::DescriptiveLineage,
    ) {
        return;
    }
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_fork(
            sequence,
            emitted_on_branch_id,
            created_branch_id,
            parent_branch_id,
            created_branch_display_name,
            parent_branch_display_name,
        ));
}

pub(crate) fn record_branch_switch_lineage(
    graph: &mut SignalGraph,
    from_branch_id: SignalBranchId,
    to_branch_id: SignalBranchId,
    from_branch_display_name: impl Into<String>,
    to_branch_display_name: impl Into<String>,
) {
    if !graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::DescriptiveLineage,
    ) {
        return;
    }
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    let emitted_on_branch_id = graph.observe().current_branch().id;
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_switch(
            sequence,
            emitted_on_branch_id,
            from_branch_id,
            to_branch_id,
            from_branch_display_name,
            to_branch_display_name,
        ));
}

pub(crate) fn record_branch_merge_summary(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    source_branch_display_name: impl Into<String> + Clone,
    target_branch_display_name: impl Into<String> + Clone,
) {
    let captures_lineage = graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::DescriptiveLineage,
    );
    let captures_replay = graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::ReplayDetail,
    );
    if !captures_lineage && !captures_replay {
        return;
    }
    let source_branch_display_name = source_branch_display_name.into();
    let target_branch_display_name = target_branch_display_name.into();
    let resolved_requirements = summary
        .resolution_plan
        .as_ref()
        .map(|plan| {
            plan.records
                .iter()
                .flat_map(|record| record.required_resolution.iter().copied())
                .collect::<std::collections::BTreeSet<_>>()
        })
        .unwrap_or_default();
    let detail = format!(
        "merged branch {} into {} with {:?}/{:?}/{:?}/{:?}, resolved_requirements={:?}",
        summary.source_branch_id.0,
        summary.target_branch_id.0,
        summary.merge_kind,
        summary.divergence,
        summary.merge_strategy,
        summary.reconciliation_policy,
        resolved_requirements
    );
    let branch_id = graph.observe().current_branch().id;
    if captures_replay {
        let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
        record_branch_merge_replay_event(graph, summary, detail, cursor, branch_id);
    }

    if captures_lineage {
        record_branch_merge_lineage(
            graph,
            summary,
            branch_id,
            source_branch_display_name,
            target_branch_display_name,
        );
    }
}

fn record_branch_merge_replay_event(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    detail: String,
    cursor: crate::diagnostics::replay::ReplayCursor,
    branch_id: SignalBranchId,
) {
    graph
        .diagnostics_state_mut()
        .record_replay_event(ReplayEvent::new(
            cursor,
            ReplayEventKind::BranchMerged,
            branch_id,
            summary.target_snapshot_id_after,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(ReplayEventDetail::BranchMergeSummary {
                message: detail,
                strategy_witness: summary.strategy_witness.clone(),
                compatibility_witness: summary.compatibility_witness.clone(),
                scoped_merge_proof: summary.scoped_merge_proof.clone(),
            }),
        ));
}

fn record_branch_merge_lineage(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    branch_id: SignalBranchId,
    source_branch_display_name: String,
    target_branch_display_name: String,
) {
    if !graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::DescriptiveLineage,
    ) {
        return;
    }
    let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
    record_branch_merge_lineage_record(
        graph,
        summary,
        branch_id,
        sequence,
        source_branch_display_name,
        target_branch_display_name,
    );

    for record in &summary.records {
        let sequence = graph.diagnostics_state_mut().allocate_lineage_sequence();
        record_artifact_merge_lineage_record(graph, summary, record, branch_id, sequence);
    }
}

fn record_branch_merge_lineage_record(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    branch_id: SignalBranchId,
    sequence: u64,
    source_branch_display_name: String,
    target_branch_display_name: String,
) {
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::branch_merge(
            sequence,
            branch_id,
            summary.source_branch_id,
            summary.target_branch_id,
            summary.merge_kind,
            summary.divergence,
            summary.merge_strategy,
            summary.reconciliation_policy,
            summary.resolution_plan.clone(),
            summary.target_snapshot_id_after,
            source_branch_display_name,
            target_branch_display_name,
        ));
}

fn record_artifact_merge_lineage_record(
    graph: &mut SignalGraph,
    summary: &BranchMergeExecutionSummary,
    record: &crate::logic::transaction::MergedArtifactRecord,
    branch_id: SignalBranchId,
    sequence: u64,
) {
    graph
        .diagnostics_state_mut()
        .record_lineage_record(LineageRecord::artifact_merge(
            sequence,
            branch_id,
            record.source_node,
            record.target_node,
            summary.source_branch_id,
            summary.target_branch_id,
            record.source_artifact_id,
            record.target_artifact_id_before,
            record.target_artifact_id_after,
            record.action,
            record.basis,
            summary.merge_kind,
            summary.divergence,
            summary.merge_strategy,
            summary.reconciliation_policy,
            record.resolved_conflict_kinds.clone(),
        ));
}

pub(crate) fn record_branch_merge_failure(
    graph: &mut SignalGraph,
    error: &crate::data::error::SignalError,
    source_branch: Option<crate::state::SignalBranchHandle>,
    target_branch: Option<crate::state::SignalBranchHandle>,
) {
    let captures_failure = graph.captures_failure_diagnostics();
    let captures_replay = graph.captures_observation_surface(
        crate::logic::transaction::SignalObservationSurface::ReplayDetail,
    );
    if !captures_failure && !captures_replay {
        graph.clear_pending_diagnostics_input();
        return;
    }
    let detail = branch_merge_failure_detail(error, source_branch.as_ref(), target_branch.as_ref());
    if captures_failure {
        DiagnosticsRecorder::new(graph).record_failure(ExecutionFailureContext::new(
            crate::diagnostics::ExecutionFailurePhase::Planning,
            None,
            None,
            None,
            None,
            None,
            detail.clone(),
        ));
    }
    if captures_replay {
        events::record_transaction_semantic_event(
            graph,
            ReplayEventKind::FailureRecorded,
            detail,
            None,
            None,
        );
    }
}

fn branch_merge_failure_detail(
    error: &crate::data::error::SignalError,
    source_branch: Option<&crate::state::SignalBranchHandle>,
    target_branch: Option<&crate::state::SignalBranchHandle>,
) -> String {
    match (source_branch, target_branch, error) {
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed {
                kind,
                evidence:
                    Some(crate::logic::transaction::BranchMergeFailureEvidence::Conflict(evidence)),
                ..
            },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}, divergence={:?}, primary={:?}, resolution={:?})",
            source_branch.id.0,
            target_branch.id.0,
            evidence.divergence,
            evidence.summary.primary_conflict_kind,
            evidence.summary.required_resolution
        ),
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed {
                kind,
                evidence:
                    Some(crate::logic::transaction::BranchMergeFailureEvidence::Identity(evidence)),
                ..
            },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}, identity_matcher={}, source_node={}, candidates={:?})",
            source_branch.id.0,
            target_branch.id.0,
            evidence.identity_matcher_name.as_str(),
            evidence.source_node,
            evidence.candidate_target_nodes
        ),
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed {
                kind,
                evidence:
                    Some(crate::logic::transaction::BranchMergeFailureEvidence::Deletion(evidence)),
                ..
            },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}, deletion_policy={}, target_only_nodes={:?})",
            source_branch.id.0,
            target_branch.id.0,
            evidence.deletion_policy_name.as_str(),
            evidence.target_only_nodes
        ),
        (
            Some(source_branch),
            Some(target_branch),
            crate::data::error::SignalError::BranchMergeFailed { kind, message, .. },
        ) => format!(
            "branch merge failed {} -> {} ({kind:?}): {message}",
            source_branch.id.0, target_branch.id.0
        ),
        _ => error.to_string(),
    }
}
