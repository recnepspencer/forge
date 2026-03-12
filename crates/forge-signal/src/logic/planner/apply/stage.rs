use std::time::Instant;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::MemoizedResultOrigin;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::{
    apply_prepared_dependency_batch, apply_prepared_evaluation_after_dependencies_with_policy,
    collect_effect_dependency_inputs_batch,
};

use super::super::execution::task_reporting::record_execution_failure;
use super::super::execution::StageSlice;
use super::super::precompute::StageExecutionData;
use super::rewiring::rewiring_summary_from_capture;
use super::super::semantic::{
    finalize_stage_batch, segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch,
    StageSemanticIdentity,
};
use super::super::stage_precompute::StagePrecomputeResult;
use super::super::types::{ExecutionReport, PlanSummary, StageExecutionRecord, StageExecutor};

#[cfg(feature = "parallel")]
use super::full_parallel::apply_full_parallel_stage;

struct SerialApplyPass {
    semantic_batch: StageSemanticBatch,
}

pub(in crate::logic::planner) fn apply_stage<F, R>(
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
    dispatch_stage_apply(
        graph,
        summary,
        stage,
        precomputed,
        comparator_resolver,
        executor,
        stage_identities,
        report,
        stage_record,
    )
}

fn dispatch_stage_apply<R>(
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
    R: ComparatorPolicyResolver,
{
    #[cfg(feature = "parallel")]
    if executor.is_full_parallel() && precomputed.parallel_admission.use_parallel {
        return dispatch_stage_apply_parallel(
            graph,
            summary,
            stage,
            precomputed,
            comparator_resolver,
            executor,
            stage_identities,
            report,
            stage_record,
        );
    }

    dispatch_stage_apply_serial(
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

#[cfg(feature = "parallel")]
fn dispatch_stage_apply_parallel<R>(
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
    R: ComparatorPolicyResolver,
{
    apply_full_parallel_stage(
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
    )
}

#[allow(clippy::too_many_arguments)]
pub(in crate::logic::planner) fn dispatch_stage_apply_serial(
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
    let apply_pass = run_serial_apply_pass(
        graph,
        summary,
        stage.index,
        stage.tasks,
        stage_execution,
        comparator_resolver,
        executor,
        stage_identities,
    )?;
    let semantic_finalize_start = Instant::now();
    finalize_stage_batch(
        graph,
        stage.tasks,
        apply_pass.semantic_batch,
        report,
        stage_record,
    )?;
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

fn run_serial_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    stage_tasks: &[crate::logic::planner::EvaluationTask],
    stage_execution: StageExecutionData,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
) -> Result<SerialApplyPass, SignalError> {
    let mut semantic_batch = StageSemanticBatch::default();
    let patches = stage_execution.into_patches(stage_tasks);
    let rewiring = patches
        .iter()
        .map(|patch| rewiring_summary_from_capture(graph, patch.node, &patch.prepared.dependencies))
        .collect::<Result<Vec<_>, SignalError>>()?;
    let dependency_updates = apply_prepared_dependency_batch(
        graph,
        &patches
            .iter()
            .map(|patch| (patch.node, &patch.prepared.dependencies))
            .collect::<Vec<_>>(),
    )?;
    let dependency_inputs = collect_effect_dependency_inputs_batch(
        graph,
        &patches.iter().map(|patch| patch.node).collect::<Vec<_>>(),
    )?;

    for (((patch, rewiring), dependency_updates), dependency_inputs) in patches
        .into_iter()
        .zip(rewiring.into_iter())
        .zip(dependency_updates.into_iter())
        .zip(dependency_inputs.into_iter())
    {
        semantic_batch.push_segment(segment_for_single_update(apply_stage_patch(
            graph,
            summary,
            stage_index,
            patch,
            rewiring,
            dependency_updates,
            dependency_inputs,
            comparator_resolver,
            executor,
            stage_identities,
        )?));
    }
    Ok(SerialApplyPass { semantic_batch })
}

fn apply_stage_patch(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    patch: super::super::precompute::PreparedTaskPatch,
    rewiring: Option<crate::logic::explain::RewiringSummary>,
    dependency_updates: u32,
    dependency_inputs: crate::logic::evaluation::EffectDependencyInputs,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
) -> Result<SemanticTaskUpdate, SignalError> {
    let identity = stage_identities[patch.task_index];
    let partition_aware = !patch.prepared.result.changed_regions.is_empty();
    let before_state = graph.get_state(patch.node)?;
    let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
    let apply_result = apply_prepared_evaluation_after_dependencies_with_policy(
        graph,
        patch.node,
        patch.prepared,
        comparator_resolver,
        None,
        dependency_updates,
        Some(dependency_inputs),
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
    let memoized_origin = graph
        .get_entry(patch.node)?
        .get_trace_summary()
        .map(|trace| trace.memoized_origin)
        .unwrap_or(MemoizedResultOrigin::DirectCompute);
    Ok(SemanticTaskUpdate {
        task_index: patch.task_index,
        node: patch.node,
        identity,
        before_state,
        before_trace,
        after_state,
        dependency_updates: apply_result.dependency_updates,
        recomputed: matches!(
            apply_result.report.verdict,
            crate::logic::evaluation::EvaluationVerdict::Recomputed
        ),
        partition_aware,
        rewiring,
        verdict: apply_result.report.verdict,
        memoized_origin,
    })
}
