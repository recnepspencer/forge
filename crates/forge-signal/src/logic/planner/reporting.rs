use crate::data::graph::SignalGraph;
use crate::data::node::NodeState;
use crate::data::output::MemoizedResultOrigin;
use crate::data::trace::TraceSummary;
use crate::diagnostics::failure::ExecutionFailureContext;
use crate::diagnostics::recorder::DiagnosticsRecorder;

use super::types::{
    EvaluationPlan, ExecutionPruneReason, ExecutionRecordId, ExecutionReport, EvaluationTask,
    TaskExecutionOutcome, TaskExecutionRecord,
};

pub(super) fn classify_task_record(
    id: ExecutionRecordId,
    task: &EvaluationTask,
    before_state: NodeState,
    after_state: NodeState,
    before_trace: Option<&TraceSummary>,
    after_trace: Option<&TraceSummary>,
) -> TaskExecutionRecord {
    let trace_changed = before_trace != after_trace;
    let recomputed = after_trace.map(|trace| trace.recomputed).unwrap_or(false);
    let memoized_reuse = after_trace
        .map(|trace| trace.memoized_origin == MemoizedResultOrigin::MemoizedFromCache)
        .unwrap_or(false);
    let propagation_suppressed = after_trace
        .map(|trace| trace.propagation_suppressed)
        .unwrap_or(false);

    let (outcome, prune_reason, condition_deferred, condition_reverted_clean) =
        match (before_state, after_state) {
            (NodeState::Clean, NodeState::Clean) => (
                TaskExecutionOutcome::Pruned,
                Some(ExecutionPruneReason::CleanAtPlanTime),
                false,
                false,
            ),
            (NodeState::MaybeStale, NodeState::Clean) if !trace_changed => (
                TaskExecutionOutcome::ValidatedClean,
                Some(ExecutionPruneReason::CleanAfterValidation),
                false,
                false,
            ),
            (_, NodeState::MaybeStale) => {
                (TaskExecutionOutcome::ConditionDeferred, None, true, false)
            }
            (_, NodeState::Clean) if memoized_reuse => {
                (TaskExecutionOutcome::MemoizedReuse, None, false, false)
            }
            (_, NodeState::Clean) if propagation_suppressed => (
                TaskExecutionOutcome::PropagationSuppressed,
                None,
                false,
                false,
            ),
            (_, NodeState::Clean) if recomputed => {
                (TaskExecutionOutcome::Recomputed, None, false, false)
            }
            (_, NodeState::Clean) => (
                TaskExecutionOutcome::ConditionRevertedClean,
                None,
                false,
                true,
            ),
            _ => (TaskExecutionOutcome::Recomputed, None, false, false),
        };

    TaskExecutionRecord {
        id,
        node: task.node,
        scheduled_reason: task.reason,
        direct_request: task.direct_request,
        outcome,
        prune_reason,
        recomputed,
        memoized_reuse,
        condition_deferred,
        condition_reverted_clean,
        propagation_suppressed,
    }
}

pub(super) fn record_successful_execution(
    graph: &mut SignalGraph,
    plan: &EvaluationPlan,
    report: &ExecutionReport,
) {
    DiagnosticsRecorder::new(graph).record_execution_completed(plan, report);
}

pub(super) fn record_execution_failure(graph: &mut SignalGraph, context: ExecutionFailureContext) {
    DiagnosticsRecorder::new(graph).record_failure(context);
}

pub(super) fn accumulate_report_counters(
    report: &mut ExecutionReport,
    task_record: &TaskExecutionRecord,
) {
    match task_record.outcome {
        TaskExecutionOutcome::Recomputed | TaskExecutionOutcome::PropagationSuppressed => {
            report.tasks_executed += 1;
        }
        TaskExecutionOutcome::ValidatedClean => {
            report.tasks_validated_clean += 1;
            report.tasks_pruned += 1;
        }
        TaskExecutionOutcome::ConditionDeferred => {
            report.tasks_deferred_by_condition += 1;
        }
        TaskExecutionOutcome::ConditionRevertedClean => {
            report.tasks_reverted_clean_by_condition += 1;
        }
        TaskExecutionOutcome::MemoizedReuse => {
            report.tasks_satisfied_by_memoization += 1;
        }
        TaskExecutionOutcome::Pruned => {
            report.tasks_pruned += 1;
        }
    }

    if task_record.propagation_suppressed {
        report.tasks_with_suppressed_propagation += 1;
    }
}
