use crate::data::graph::SignalGraph;
use crate::data::node::NodeState;
use crate::data::output::MemoizedResultOrigin;
use crate::data::reuse::{ReuseBasis, ReuseOrigin, ReuseStrategy};
use crate::data::temporal::LoweredTemporalEligibility;
use crate::data::trace::RuntimeArtifactFinalizeImage;
use crate::diagnostics::failure::ExecutionFailureContext;
use crate::diagnostics::recorder::DiagnosticsRecorder;
use crate::logic::evaluation::{EvaluationVerdict, SuppressionReason};

use super::super::types::{
    EligibleTask, ExecutedTask, ExecutionPruneReason, ExecutionRecordId, ExecutionReport,
    SemanticSegmentId, TaskExecutionOutcome, TaskExecutionRecord,
};

#[allow(dead_code)]
pub(crate) fn classify_task_record(
    id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
    task: &EligibleTask,
    before_state: NodeState,
    after_state: NodeState,
    before_trace: Option<&RuntimeArtifactFinalizeImage>,
    after_trace: Option<&RuntimeArtifactFinalizeImage>,
    verdict: EvaluationVerdict,
    temporal_eligibility: Option<LoweredTemporalEligibility>,
    memoized_origin: MemoizedResultOrigin,
    reuse_basis: ReuseBasis,
) -> ExecutedTask {
    ExecutedTask {
        task: task.clone(),
        record: classify_task_execution_record(
            id,
            semantic_segment_id,
            task,
            before_state,
            after_state,
            before_trace,
            after_trace,
            verdict,
            temporal_eligibility,
            memoized_origin,
            reuse_basis,
        ),
    }
}

pub(crate) fn classify_task_execution_record(
    id: ExecutionRecordId,
    semantic_segment_id: SemanticSegmentId,
    task: &EligibleTask,
    before_state: NodeState,
    after_state: NodeState,
    before_trace: Option<&RuntimeArtifactFinalizeImage>,
    after_trace: Option<&RuntimeArtifactFinalizeImage>,
    verdict: EvaluationVerdict,
    temporal_eligibility: Option<LoweredTemporalEligibility>,
    memoized_origin: MemoizedResultOrigin,
    reuse_basis: ReuseBasis,
) -> TaskExecutionRecord {
    let trace_changed = before_trace != after_trace;
    let recomputed = after_trace
        .map(|trace| trace.recomputed())
        .unwrap_or(matches!(verdict, EvaluationVerdict::Recomputed));
    let reuse_origin = after_trace
        .map(|trace| trace.reuse_origin())
        .unwrap_or_else(|| classify_reuse_origin(&verdict, &reuse_basis));
    let propagation_suppressed = after_trace
        .map(|trace| trace.propagation_suppressed())
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
            _ if matches!(reuse_origin, ReuseOrigin::MemoizedArtifactReuse) => {
                (TaskExecutionOutcome::MemoizedReuse, None)
            }
            _ if matches!(reuse_origin, ReuseOrigin::SnapshotRestore) => {
                (TaskExecutionOutcome::SnapshotRestoreReuse, None)
            }
            _ if matches!(reuse_origin, ReuseOrigin::ReconciliationAdoption) => {
                (TaskExecutionOutcome::ReconciliationAdoption, None)
            }
            _ if matches!(reuse_origin, ReuseOrigin::CrossIdentityPersistentReuse) => {
                (TaskExecutionOutcome::CrossIdentityPersistentReuse, None)
            }
            _ if matches!(reuse_origin, ReuseOrigin::PartialArtifactSplice) => {
                (TaskExecutionOutcome::PartialArtifactSplice, None)
            }
            _ if recomputed && !propagation_suppressed => (TaskExecutionOutcome::Recomputed, None),
            _ if propagation_suppressed => (TaskExecutionOutcome::PropagationSuppressed, None),
            _ if !trace_changed
                && matches!(
                    (before_state, after_state),
                    (NodeState::Clean, NodeState::Clean)
                ) =>
            {
                (
                    TaskExecutionOutcome::Pruned,
                    Some(ExecutionPruneReason::CleanAtPlanTime),
                )
            }
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
        temporal_eligibility,
        prune_reason,
        recomputed,
        memoized_origin,
        reuse_basis,
        reuse_origin,
        propagation_suppressed,
    }
}

fn classify_reuse_origin(verdict: &EvaluationVerdict, reuse_basis: &ReuseBasis) -> ReuseOrigin {
    match reuse_basis.strategy {
        Some(ReuseStrategy::MemoizedArtifactReuse) => ReuseOrigin::MemoizedArtifactReuse,
        Some(ReuseStrategy::SnapshotRestoreReuse) => ReuseOrigin::SnapshotRestore,
        Some(ReuseStrategy::ReconciliationAdoption) => ReuseOrigin::ReconciliationAdoption,
        Some(ReuseStrategy::CrossIdentityPersistentMatch) => {
            ReuseOrigin::CrossIdentityPersistentReuse
        }
        Some(ReuseStrategy::PartialArtifactSplicing) => ReuseOrigin::PartialArtifactSplice,
        Some(ReuseStrategy::OutputSuppression) => ReuseOrigin::OutputSuppressed,
        None => match verdict {
            EvaluationVerdict::Suppressed {
                reason:
                    SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged
                    | SuppressionReason::ComparatorMatch,
            } => ReuseOrigin::OutputSuppressed,
            _ => ReuseOrigin::FreshCompute,
        },
    }
}

pub(crate) fn record_execution_failure(graph: &mut SignalGraph, context: ExecutionFailureContext) {
    if !graph.captures_failure_diagnostics() {
        graph.clear_pending_diagnostics_input();
        return;
    }
    DiagnosticsRecorder::new(graph).record_failure(context);
}

pub(crate) fn record_execution_failure_if_enabled(
    graph: &mut SignalGraph,
    build: impl FnOnce() -> ExecutionFailureContext,
) {
    if !graph.captures_failure_diagnostics() {
        graph.clear_pending_diagnostics_input();
        return;
    }
    record_execution_failure(graph, build());
}

pub(crate) fn accumulate_report_counters(
    report: &mut ExecutionReport,
    task_record: &TaskExecutionRecord,
) {
    report.latest_execution_record_id = Some(
        report
            .latest_execution_record_id
            .map_or(task_record.id.0, |current| current.max(task_record.id.0)),
    );
    *report
        .reuse_origin_counts
        .entry(task_record.reuse_origin)
        .or_insert(0) += 1;
    if let Some(temporal_eligibility) = task_record.temporal_eligibility.as_ref() {
        report.temporal_summary.observe(temporal_eligibility);
    }

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
        TaskExecutionOutcome::SnapshotRestoreReuse
        | TaskExecutionOutcome::ReconciliationAdoption
        | TaskExecutionOutcome::CrossIdentityPersistentReuse
        | TaskExecutionOutcome::PartialArtifactSplice => {
            report.tasks_executed += 1;
        }
        TaskExecutionOutcome::Pruned => {
            report.tasks_pruned += 1;
        }
    }

    if task_record.propagation_suppressed {
        report.tasks_with_suppressed_propagation += 1;
    }
}
