use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExecutionReportSummary, ExplanationSummary,
    GraphSummary,
};

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
    format!(
        "ExecutionReportSummary profile={:?} stages={} tasks={} executed={} memoized={} suppressed={}",
        summary.profile,
        summary.stage_count,
        summary.task_count,
        summary.tasks_executed,
        summary.tasks_satisfied_by_memoization,
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
    format!(
        "ExecutionHistorySummary profile={:?} traced_nodes={} execution_records={} latest_execution_record_id={:?}",
        summary.profile,
        summary.traced_node_count,
        summary.execution_record_count,
        summary.latest_execution_record_id
    )
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
