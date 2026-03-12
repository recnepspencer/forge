use crate::data::graph::SignalGraph;
use crate::data::node::NodeState;
use crate::data::output::MemoizedResultOrigin;
use crate::data::trace::TraceSummary;
use crate::diagnostics::failure::ExecutionFailureContext;
use crate::diagnostics::recorder::DiagnosticsRecorder;
use crate::logic::evaluation::{EvaluationVerdict, SuppressionReason};

use super::super::types::{
    EvaluationTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport, SemanticSegmentId,
    TaskExecutionOutcome, TaskExecutionRecord,
};

pub(crate) fn classify_task_record(
    id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
    task: &EvaluationTask,
    before_state: NodeState,
    after_state: NodeState,
    before_trace: Option<&TraceSummary>,
    after_trace: Option<&TraceSummary>,
    verdict: EvaluationVerdict,
    memoized_origin: MemoizedResultOrigin,
) -> TaskExecutionRecord {
    let trace_changed = before_trace != after_trace;
    let recomputed = matches!(verdict, EvaluationVerdict::Recomputed);
    let memoized_reuse = memoized_origin == MemoizedResultOrigin::MemoizedFromCache;
    let propagation_suppressed = after_trace
        .map(|trace| trace.propagation_suppressed)
        .unwrap_or(false);
    let suppression_reason = match verdict {
        EvaluationVerdict::Suppressed { reason } => Some(reason),
        _ => None,
    };
    let deferral_reason = match verdict {
        EvaluationVerdict::Deferred { reason } => Some(reason),
        _ => None,
    };
    let (outcome, prune_reason) = match verdict {
        EvaluationVerdict::Recomputed => (TaskExecutionOutcome::Recomputed, None),
        EvaluationVerdict::Deferred { .. } => (TaskExecutionOutcome::ConditionDeferred, None),
        EvaluationVerdict::Suppressed { reason } => match reason {
            SuppressionReason::ValidatedClean => (
                TaskExecutionOutcome::ValidatedClean,
                Some(ExecutionPruneReason::CleanAfterValidation),
            ),
            SuppressionReason::ConditionRevertedClean => {
                (TaskExecutionOutcome::ConditionRevertedClean, None)
            }
            _ if memoized_reuse => (TaskExecutionOutcome::MemoizedReuse, None),
            _ if propagation_suppressed => (TaskExecutionOutcome::PropagationSuppressed, None),
            _ if !trace_changed && matches!((before_state, after_state), (NodeState::Clean, NodeState::Clean)) => (
                TaskExecutionOutcome::Pruned,
                Some(ExecutionPruneReason::CleanAtPlanTime),
            ),
            _ => (TaskExecutionOutcome::PropagationSuppressed, None),
        },
    };

    TaskExecutionRecord {
        id,
        semantic_segment_id,
        node: task.node,
        scheduled_reason: task.reason,
        direct_request: task.direct_request,
        outcome,
        verdict: Some(verdict),
        suppression_reason,
        deferral_reason,
        prune_reason,
        recomputed,
        memoized_origin,
        propagation_suppressed,
    }
}

pub(crate) fn record_execution_failure(
    graph: &mut SignalGraph,
    context: ExecutionFailureContext,
) {
    DiagnosticsRecorder::new(graph).record_failure(context);
}

pub(crate) fn accumulate_report_counters(
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
