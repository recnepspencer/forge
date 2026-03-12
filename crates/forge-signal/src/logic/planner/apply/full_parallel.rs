use std::time::Instant;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::MemoizedResultOrigin;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::apply_prepared_evaluation_with_policy;

use super::groups::build_stage_apply_plan;
use super::super::execution::task_reporting::record_execution_failure;
use super::super::semantic::{
    finalize_stage_batch, segment_for_single_update, segment_for_updates, SemanticTaskUpdate,
    StageSemanticBatch, StageSemanticIdentity,
};
use super::super::types::{
    EvaluationTask, ExecutionReport, ParallelApplyMode, PlanSummary, StageExecutionRecord,
    StageExecutor,
};
use super::{prepare_stage_patches, TaskPatch};

pub(super) fn apply_full_parallel_stage(
    graph: &mut SignalGraph,
    stage_index: u32,
    stage_tasks: &[EvaluationTask],
    patches: Vec<super::super::precompute::PreparedTaskPatch>,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    plan_summary: &PlanSummary,
    stage_identities: &[StageSemanticIdentity],
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    let policy = executor
        .parallel_policy()
        .expect("full parallel execution should carry a policy");
    let prepared = prepare_stage_patches(graph, patches, comparator_resolver)?;
    let apply_plan = build_stage_apply_plan(prepared.tasks, policy);
    let mut semantic_batch = StageSemanticBatch::default();
    for group in apply_plan.groups {
        stage_record.apply_mode = Some(ParallelApplyMode::SerialFallback);
        stage_record.serial_fallback_group_count += 1;
        stage_record.serial_apply_task_count += group.tasks.len() as u32;
        for task in group.tasks {
            apply_task_patch(
                graph,
                stage_index,
                task,
                comparator_resolver,
                executor,
                plan_summary,
                stage_identities,
                &mut semantic_batch,
            )?;
        }
    }

    if !prepared.serial_fallbacks.is_empty() && stage_record.apply_mode.is_none() {
        stage_record.apply_mode = Some(ParallelApplyMode::SerialFallback);
    }
    for patch in prepared.serial_fallbacks {
        stage_record.serial_fallback_group_count += 1;
        stage_record.serial_apply_task_count += 1;
        apply_serial_fallback_patch(
            graph,
            stage_index,
            patch,
            comparator_resolver,
            executor,
            plan_summary,
            stage_identities,
            &mut semantic_batch,
        )?;
    }

    let semantic_finalize_start = Instant::now();
    finalize_stage_batch(graph, stage_tasks, semantic_batch, report, stage_record)?;
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn apply_task_patch(
    graph: &mut SignalGraph,
    stage_index: u32,
    patch: TaskPatch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    plan_summary: &PlanSummary,
    stage_identities: &[StageSemanticIdentity],
    semantic_batch: &mut StageSemanticBatch,
) -> Result<(), SignalError> {
    let identity = stage_identities[patch.task_index];
    let partition_aware = !patch.prepared.result.changed_regions.is_empty();
    let before_state = graph.get_state(patch.node)?;
    let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
    let apply_result = apply_prepared_evaluation_with_policy(
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
                Some(plan_summary.clone()),
                err.to_string(),
            ),
        );
        err
    })?;
    let after_state = graph.get_state(patch.node)?;
    semantic_batch.push_segment(segment_for_single_update(SemanticTaskUpdate {
        task_index: patch.task_index,
        node: patch.node,
        identity,
        before_state: patch.before_state,
        before_trace: patch.before_trace,
        after_state,
        dependency_updates: apply_result.dependency_updates,
        recomputed: matches!(
            apply_result.report.verdict,
            crate::logic::evaluation::EvaluationVerdict::Recomputed
        ),
        partition_aware: patch.partition_aware,
        rewiring: patch.rewiring,
        verdict: apply_result.report.verdict,
        memoized_origin: graph
            .get_entry(patch.node)?
            .get_trace_summary()
            .map(|trace| trace.memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute),
    }));
    Ok(())
}
