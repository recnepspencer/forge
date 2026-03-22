use std::collections::BTreeMap;

use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExecutionReportSummary, ExplanationSummary,
    GraphSummary,
};
use crate::logic::planner::TaskExecutionOutcome;

pub fn render_graph_summary(summary: &GraphSummary) -> String {
    format!(
        "GraphSummary profile={:?} nodes={} dirty={} maybe_stale={} deps={} traces={} records={}",
        summary.profile,
        summary.active_node_count,
        summary.dirty_node_count,
        summary.maybe_stale_node_count,
        summary.dependency_edge_count,
        summary.nodes_with_trace_summary,
        summary.nodes_with_execution_record
    )
}

pub fn render_plan_summary(summary: &EvaluationPlanSummary) -> String {
    format!(
        "EvaluationPlanSummary profile={:?} targets={} stages={} tasks={} max_stage_width={}",
        summary.profile,
        summary.requested_target_count,
        summary.stage_count,
        summary.task_count,
        summary.max_stage_width
    )
}

pub fn render_execution_report_summary(summary: &ExecutionReportSummary) -> String {
    let advanced_reuse = render_labeled_counts(
        &summary.task_outcome_counts,
        &[
            TaskExecutionOutcome::SnapshotRestoreReuse,
            TaskExecutionOutcome::ReconciliationAdoption,
            TaskExecutionOutcome::CrossIdentityPersistentReuse,
            TaskExecutionOutcome::PartialArtifactSplice,
        ],
    );
    format!(
        "ExecutionReportSummary profile={:?} stages={} tasks={} executed={} memoized={} advanced_reuse=[{}] suppressed={}",
        summary.profile,
        summary.stage_count,
        summary.task_count,
        summary.tasks_executed,
        summary.tasks_satisfied_by_memoization,
        advanced_reuse,
        summary.tasks_with_suppressed_propagation
    )
}

pub fn render_explanation_summary(summary: &ExplanationSummary) -> String {
    format!(
        "ExplanationSummary profile={:?} node={} state={:?} upstream={} changed={} skipped={} deferred={} locality(discarded={},insufficient={}) triage={:?}",
        summary.profile,
        summary.node,
        summary.state,
        summary.upstream_count,
        summary.changed_upstream_count,
        summary.skipped_upstream_count,
        summary.condition_deferred_count,
        summary.discarded_scope_count,
        summary.insufficient_scope_count,
        summary.triage_classes
    )
}

pub fn render_execution_history_summary(summary: &ExecutionHistorySummary) -> String {
    let mut correspondence_counts = BTreeMap::new();
    let mut partial_splice_nodes = 0_u32;
    let mut composition_region_total = 0_u32;

    for node in &summary.nodes {
        if let Some(kind) = node.persistent_correspondence_kind {
            *correspondence_counts
                .entry(format!("{kind:?}"))
                .or_insert(0_u32) += 1;
        }
        if matches!(
            node.reuse_origin,
            Some(crate::data::reuse::ReuseOrigin::PartialArtifactSplice)
        ) {
            partial_splice_nodes += 1;
            composition_region_total += node.composition_region_count;
        }
    }

    format!(
        "ExecutionHistorySummary profile={:?} traced_nodes={} execution_records={} latest_execution_record_id={:?} reuse_origins={:?} cross_identity_families={:?} partial_splice_nodes={} partial_splice_regions={}",
        summary.profile,
        summary.traced_node_count,
        summary.execution_record_count,
        summary.latest_execution_record_id,
        summary.reuse_origin_counts,
        correspondence_counts,
        partial_splice_nodes,
        composition_region_total
    )
}

fn render_labeled_counts(
    counts: &crate::diagnostics::summary::TaskOutcomeCounts,
    labels: &[TaskExecutionOutcome],
) -> String {
    labels
        .iter()
        .map(|label| {
            format!(
                "{}={}",
                normalize_display_label(label),
                counts.get(label).copied().unwrap_or(0)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn normalize_display_label(label: &TaskExecutionOutcome) -> &'static str {
    match label {
        TaskExecutionOutcome::SnapshotRestoreReuse => "snapshot_restore",
        TaskExecutionOutcome::ReconciliationAdoption => "reconciliation",
        TaskExecutionOutcome::CrossIdentityPersistentReuse => "cross_identity",
        TaskExecutionOutcome::PartialArtifactSplice => "partial_splice",
        _ => "unknown",
    }
}

pub fn render_flow_summary(summary: &FlowSummary) -> String {
    format!(
        "FlowSummary profile={:?} changed_nodes={} changed_regions={} planned_tasks={} prepared={} applied={} rollback={} samples={} epochs={}",
        summary.profile,
        summary.change.changed_nodes.len(),
        summary.change.changed_region_count,
        summary.planning.plan.task_count,
        summary.precompute.prepared_evaluations_produced,
        summary.apply.prepared_evaluations_applied,
        summary.rollback.is_some(),
        summary.cause_samples.len(),
        summary.event_epochs.len()
    )
}

pub fn render_failure_summary(summary: &FailureSummary) -> String {
    format!(
        "FailureSummary profile={:?} phase={:?} node={:?} rolled_back={} message={}",
        summary.profile, summary.phase, summary.node, summary.rolled_back, summary.message
    )
}


