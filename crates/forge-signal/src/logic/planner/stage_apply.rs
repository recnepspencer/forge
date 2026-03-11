use std::time::Instant;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::apply_prepared_evaluation_with_policy;
use crate::logic::prepared::{PreparedEvaluationOrigin, PreparedEvaluationOutcome};

use super::execution::StageSlice;
use super::precompute::StageExecutionData;
use super::reporting::record_execution_failure;
use super::rewiring::rewiring_summary_from_capture;
use super::semantic::{
    finalize_stage_batch, segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch,
    StageSemanticIdentity,
};
use super::stage_precompute::StagePrecomputeResult;
use super::types::{ExecutionReport, PlanSummary, StageExecutionRecord, StageExecutor};

#[cfg(feature = "parallel")]
use super::full_parallel::apply_full_parallel_stage;

pub(crate) fn apply_stage<F, R>(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage: &StageSlice<'_>,
    precomputed: StagePrecomputeResult,
    comparator_resolver: &mut R,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError>
where
    F: Fn(
            crate::data::handle::NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<crate::logic::prepared::PreparedEvaluation, SignalError>
        + Sync,
    R: ComparatorPolicyResolver,
{
    #[cfg(feature = "parallel")]
    if executor.is_full_parallel() && precomputed.parallel_admission.use_parallel {
        return apply_full_parallel_stage(
            graph,
            stage.index,
            stage.tasks,
            precomputed.execution.into_patches(stage.tasks),
            comparator_resolver,
            executor,
            summary,
            stage_identities,
            report,
            stage_record,
        );
    }

    apply_stage_serially(
        graph,
        summary,
        stage,
        precomputed.execution,
        comparator_resolver,
        executor,
        stage_identities,
        report,
        stage_record,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_stage_serially(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage: &StageSlice<'_>,
    stage_execution: StageExecutionData,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    let mut semantic_batch = StageSemanticBatch::default();
    for patch in stage_execution.into_patches(stage.tasks) {
        semantic_batch.push_segment(segment_for_single_update(apply_stage_patch(
            graph,
            summary,
            stage.index,
            patch,
            comparator_resolver,
            executor,
            stage_identities,
        )?));
    }
    let semantic_finalize_start = Instant::now();
    finalize_stage_batch(graph, stage.tasks, semantic_batch, report, stage_record)?;
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

fn apply_stage_patch(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    patch: super::precompute::PreparedTaskPatch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
) -> Result<SemanticTaskUpdate, SignalError> {
    let identity = stage_identities[patch.task_index];
    let recomputed = matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(patch.prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
    let prepared_outcome = patch.prepared.outcome;
    let prepared_origin = patch.prepared.origin;
    let partition_aware = !patch.prepared.result.changed_regions.is_empty();
    let before_state = graph.get_state(patch.node)?;
    let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
    let rewiring = rewiring_summary_from_capture(graph, patch.node, &patch.prepared.dependencies)?;
    let dependency_updates = apply_prepared_evaluation_with_policy(
        graph,
        patch.node,
        patch.prepared,
        comparator_resolver,
        None,
    )
    .map_err(|err| {
        record_execution_failure(
            graph,
            ExecutionFailureContext::new(
                ExecutionFailurePhase::Apply,
                Some(stage_index),
                Some(patch.node),
                Some(executor),
                Some(identity.record_id),
                Some(summary.clone()),
                err.to_string(),
            ),
        );
        err
    })?;
    let after_state = graph.get_state(patch.node)?;
    Ok(SemanticTaskUpdate {
        task_index: patch.task_index,
        node: patch.node,
        identity,
        before_state,
        before_trace,
        after_state,
        dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
        prepared_outcome,
        prepared_origin,
    })
}
