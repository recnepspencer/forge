use crate::diagnostics::failure::FailureSummary;
use crate::diagnostics::flow::FlowSummary;
use crate::diagnostics::lineage::LineageRecord;
use crate::diagnostics::replay::ReplaySlice;
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExecutionReportSummary, ExplanationSummary,
    GraphSummary,
};

use super::model::{
    compare_value, push_mismatch, DiagnosticMismatchCategory, ExecutionReportDiff, ExplanationDiff,
    FailureDiff, FlowDiff, GraphDiff, HistoryDiff, LineageDiff, PlanDiff, ReplayDiff,
};

pub fn compare_graphs(left: &GraphSummary, right: &GraphSummary) -> GraphDiff {
    let mut diff = GraphDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "active_node_count",
        left.active_node_count,
        right.active_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "clean_node_count",
        left.clean_node_count,
        right.clean_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "maybe_stale_node_count",
        left.maybe_stale_node_count,
        right.maybe_stale_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphState,
        "dirty_node_count",
        left.dirty_node_count,
        right.dirty_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphStructure,
        "dependency_edge_count",
        left.dependency_edge_count,
        right.dependency_edge_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::GraphStructure,
        "nodes_with_partition_scopes",
        left.nodes_with_partition_scopes,
        right.nodes_with_partition_scopes,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "nodes_with_execution_record",
        left.nodes_with_execution_record,
        right.nodes_with_execution_record,
    );
    if left.sample_dirty_nodes != right.sample_dirty_nodes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::GraphState,
            "sample_dirty_nodes",
            format!("{:?}", left.sample_dirty_nodes),
            format!("{:?}", right.sample_dirty_nodes),
        );
    }
    if left.metrics != right.metrics {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Metrics,
            "metrics",
            format!("{:?}", left.metrics),
            format!("{:?}", right.metrics),
        );
    }
    diff
}

pub fn compare_plans(left: &EvaluationPlanSummary, right: &EvaluationPlanSummary) -> PlanDiff {
    let mut diff = PlanDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "stage_count",
        left.stage_count,
        right.stage_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "task_count",
        left.task_count,
        right.task_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::PlanShape,
        "max_stage_width",
        left.max_stage_width,
        right.max_stage_width,
    );
    if left.stage_widths != right.stage_widths {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::PlanShape,
            "stage_widths",
            format!("{:?}", left.stage_widths),
            format!("{:?}", right.stage_widths),
        );
    }
    if left.task_reason_counts != right.task_reason_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::PlanShape,
            "task_reason_counts",
            format!("{:?}", left.task_reason_counts),
            format!("{:?}", right.task_reason_counts),
        );
    }
    diff
}

pub fn compare_execution_reports(
    left: &ExecutionReportSummary,
    right: &ExecutionReportSummary,
) -> ExecutionReportDiff {
    let mut diff = ExecutionReportDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "stage_count",
        left.stage_count,
        right.stage_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "task_count",
        left.task_count,
        right.task_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_executed",
        left.tasks_executed,
        right.tasks_executed,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_validated_clean",
        left.tasks_validated_clean,
        right.tasks_validated_clean,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_satisfied_by_memoization",
        left.tasks_satisfied_by_memoization,
        right.tasks_satisfied_by_memoization,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::TaskOutcome,
        "tasks_with_suppressed_propagation",
        left.tasks_with_suppressed_propagation,
        right.tasks_with_suppressed_propagation,
    );
    if left.task_outcome_counts != right.task_outcome_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::TaskOutcome,
            "task_outcome_counts",
            format!("{:?}", left.task_outcome_counts),
            format!("{:?}", right.task_outcome_counts),
        );
    }
    if left.stage_outcome_counts != right.stage_outcome_counts {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "stage_outcome_counts",
            format!("{:?}", left.stage_outcome_counts),
            format!("{:?}", right.stage_outcome_counts),
        );
    }
    diff
}

pub fn compare_explanations(
    left: &ExplanationSummary,
    right: &ExplanationSummary,
) -> ExplanationDiff {
    let mut diff = ExplanationDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "materialization_mode",
        left.materialization_mode.clone(),
        right.materialization_mode.clone(),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "state",
        format!("{:?}", left.state),
        format!("{:?}", right.state),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "changed_upstream_count",
        left.changed_upstream_count,
        right.changed_upstream_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "skipped_upstream_count",
        left.skipped_upstream_count,
        right.skipped_upstream_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "condition_deferred_count",
        left.condition_deferred_count,
        right.condition_deferred_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "conservative_cause_count",
        left.conservative_cause_count,
        right.conservative_cause_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "direct_scope_count",
        left.direct_scope_count,
        right.direct_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "translated_scope_count",
        left.translated_scope_count,
        right.translated_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "discarded_scope_count",
        left.discarded_scope_count,
        right.discarded_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "insufficient_scope_count",
        left.insufficient_scope_count,
        right.insufficient_scope_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "rewired_dependency_count",
        left.rewired_dependency_count,
        right.rewired_dependency_count,
    );
    if left.direct_cause_kinds != right.direct_cause_kinds {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "direct_cause_kinds",
            format!("{:?}", left.direct_cause_kinds),
            format!("{:?}", right.direct_cause_kinds),
        );
    }
    if left.scope_provenance_kinds != right.scope_provenance_kinds {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "scope_provenance_kinds",
            format!("{:?}", left.scope_provenance_kinds),
            format!("{:?}", right.scope_provenance_kinds),
        );
    }
    if left.cause_note_samples != right.cause_note_samples {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "cause_note_samples",
            format!("{:?}", left.cause_note_samples),
            format!("{:?}", right.cause_note_samples),
        );
    }
    if left.triage_classes != right.triage_classes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "triage_classes",
            format!("{:?}", left.triage_classes),
            format!("{:?}", right.triage_classes),
        );
    }
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "propagation_suppressed",
        left.propagation_suppressed,
        right.propagation_suppressed,
    );
    if left.output_change != right.output_change {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "output_change",
            format!("{:?}", left.output_change),
            format!("{:?}", right.output_change),
        );
    }
    if left.memoized_origin != right.memoized_origin {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "memoized_origin",
            format!("{:?}", left.memoized_origin),
            format!("{:?}", right.memoized_origin),
        );
    }
    diff
}

pub fn compare_execution_history(
    left: &ExecutionHistorySummary,
    right: &ExecutionHistorySummary,
) -> HistoryDiff {
    let mut diff = HistoryDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::History,
        "traced_node_count",
        left.traced_node_count,
        right.traced_node_count,
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "execution_record_count",
        left.execution_record_count,
        right.execution_record_count,
    );
    if left.latest_execution_record_id != right.latest_execution_record_id {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "latest_execution_record_id",
            format!("{:?}", left.latest_execution_record_id),
            format!("{:?}", right.latest_execution_record_id),
        );
    }
    if left.nodes != right.nodes {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::History,
            "nodes",
            format!("{:?}", left.nodes),
            format!("{:?}", right.nodes),
        );
    }
    diff
}

pub fn compare_flows(left: &FlowSummary, right: &FlowSummary) -> FlowDiff {
    let mut diff = FlowDiff::default();
    if left.change != right.change {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "change",
            format!("{:?}", left.change),
            format!("{:?}", right.change),
        );
    }
    if left.invalidation != right.invalidation {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "invalidation",
            format!("{:?}", left.invalidation),
            format!("{:?}", right.invalidation),
        );
    }
    diff.mismatches
        .extend(compare_plans(&left.planning.plan, &right.planning.plan).mismatches);
    if left.precompute != right.precompute {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "precompute",
            format!("{:?}", left.precompute),
            format!("{:?}", right.precompute),
        );
    }
    diff.mismatches
        .extend(compare_execution_reports(&left.apply.report, &right.apply.report).mismatches);
    if left.apply != right.apply {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "apply",
            format!("{:?}", left.apply),
            format!("{:?}", right.apply),
        );
    }
    if left.rollback != right.rollback {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "rollback",
            format!("{:?}", left.rollback),
            format!("{:?}", right.rollback),
        );
    }
    match (&left.explanation, &right.explanation) {
        (Some(left), Some(right)) => diff
            .mismatches
            .extend(compare_explanations(left, right).mismatches),
        (None, Some(_)) | (Some(_), None) => push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "explanation_presence",
            left.explanation.is_some(),
            right.explanation.is_some(),
        ),
        (None, None) => {}
    }
    if left.cause_samples != right.cause_samples {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "cause_samples",
            format!("{:?}", left.cause_samples),
            format!("{:?}", right.cause_samples),
        );
    }
    if left.event_epochs != right.event_epochs {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Flow,
            "event_epochs",
            format!("{:?}", left.event_epochs),
            format!("{:?}", right.event_epochs),
        );
    }
    diff
}

pub fn compare_failures(left: &FailureSummary, right: &FailureSummary) -> FailureDiff {
    let mut diff = FailureDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::FailureState,
        "phase",
        format!("{:?}", left.phase),
        format!("{:?}", right.phase),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::FailureState,
        "rolled_back",
        left.rolled_back,
        right.rolled_back,
    );
    if left.node != right.node {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::FailureState,
            "node",
            format!("{:?}", left.node),
            format!("{:?}", right.node),
        );
    }
    if left.message != right.message {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::FailureState,
            "message",
            &left.message,
            &right.message,
        );
    }
    diff
}

pub fn compare_replay_slices(left: &ReplaySlice, right: &ReplaySlice) -> ReplayDiff {
    let mut diff = ReplayDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "start",
        format!("{:?}", left.start),
        format!("{:?}", right.start),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "end",
        format!("{:?}", left.end),
        format!("{:?}", right.end),
    );
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::ExecutionRecord,
        "frame_count",
        left.frames.len(),
        right.frames.len(),
    );
    if left.frames != right.frames {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::ExecutionRecord,
            "frames",
            format!("{:?}", left.frames),
            format!("{:?}", right.frames),
        );
    }
    diff
}

pub fn compare_lineage_records(left: &[LineageRecord], right: &[LineageRecord]) -> LineageDiff {
    let mut diff = LineageDiff::default();
    compare_value(
        &mut diff.mismatches,
        DiagnosticMismatchCategory::Provenance,
        "record_count",
        left.len(),
        right.len(),
    );
    if left != right {
        push_mismatch(
            &mut diff.mismatches,
            DiagnosticMismatchCategory::Provenance,
            "records",
            format!("{:?}", left),
            format!("{:?}", right),
        );
    }
    diff
}
