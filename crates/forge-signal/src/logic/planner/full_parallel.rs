use std::time::Instant;

use std::collections::BTreeSet;

use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeEntry;
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::logic::evaluation::apply_prepared_evaluation_with_policy;
use crate::logic::prepared::{PreparedEvaluationOrigin, PreparedEvaluationOutcome};

use super::apply::{materialize_apply_group, prepare_stage_patches, TaskPatch};
use super::apply_groups::build_stage_apply_plan;
use super::reporting::record_execution_failure;
use super::semantic::{
    finalize_stage_batch, segment_for_single_update, segment_for_updates, SemanticTaskUpdate,
    StageSemanticBatch, StageSemanticIdentity,
};
use super::types::{
    EvaluationPlan, ExecutionReport, ParallelApplyMode, StageExecutionRecord, StageExecutor,
};

struct MaterializedApplyGroup {
    tasks: Vec<TaskPatch>,
    updates: Vec<(crate::data::handle::NodeId, NodeEntry)>,
    touched_nodes: BTreeSet<NodeId>,
}

pub(super) fn apply_full_parallel_stage(
    graph: &mut SignalGraph,
    stage: &super::types::ExecutionStage,
    patches: Vec<super::precompute::PreparedTaskPatch>,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    plan: &EvaluationPlan,
    stage_identities: &[StageSemanticIdentity],
    report: &mut ExecutionReport,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    let policy = executor
        .parallel_policy()
        .expect("full parallel execution should carry a policy");
    let prepared = prepare_stage_patches(graph, patches, comparator_resolver)?;
    let apply_plan = build_stage_apply_plan(prepared.tasks, policy);
    let mut concurrent_batch = Vec::new();
    let mut concurrent_nodes = BTreeSet::new();
    let mut semantic_batch = StageSemanticBatch::default();
    for group in apply_plan.groups {
        if group.footprint.conflicts_with_nodes(&concurrent_nodes) {
            flush_concurrent_groups(
                graph,
                &mut concurrent_batch,
                &mut concurrent_nodes,
                stage_identities,
                &mut semantic_batch,
                stage_record,
            )?;
        }

        let materialized = {
            let updates = materialize_apply_group(graph, &group)?;
            MaterializedApplyGroup {
                touched_nodes: updates.iter().map(|(node, _)| *node).collect(),
                updates,
                tasks: group.tasks,
            }
        };

        if materialized.updates.len() >= policy.apply_group_min_width.get() {
            concurrent_nodes.extend(materialized.touched_nodes.iter().copied());
            concurrent_batch.push(materialized);
            continue;
        }

        flush_concurrent_groups(
            graph,
            &mut concurrent_batch,
            &mut concurrent_nodes,
            stage_identities,
            &mut semantic_batch,
            stage_record,
        )?;
        apply_group_with_rollback(graph, &materialized.updates, false)?;
        if stage_record.apply_mode.is_none() {
            stage_record.apply_mode = Some(ParallelApplyMode::SerialFallback);
        }
        stage_record.serial_fallback_group_count += 1;
        stage_record.serial_apply_task_count += materialized.tasks.len() as u32;
        semantic_batch.push_segment(build_group_segment(
            graph,
            &materialized.tasks,
            stage_identities,
        )?);
    }

    flush_concurrent_groups(
        graph,
        &mut concurrent_batch,
        &mut concurrent_nodes,
        stage_identities,
        &mut semantic_batch,
        stage_record,
    )?;

    if !prepared.serial_fallbacks.is_empty() && stage_record.apply_mode.is_none() {
        stage_record.apply_mode = Some(ParallelApplyMode::SerialFallback);
    }
    for patch in prepared.serial_fallbacks {
        stage_record.serial_fallback_group_count += 1;
        stage_record.serial_apply_task_count += 1;
        apply_serial_fallback_patch(
            graph,
            stage,
            patch,
            comparator_resolver,
            executor,
            plan,
            stage_identities,
            &mut semantic_batch,
        )?;
    }

    let semantic_finalize_start = Instant::now();
    finalize_stage_batch(graph, &stage.tasks, semantic_batch, report, stage_record)?;
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

fn flush_concurrent_groups(
    graph: &mut SignalGraph,
    groups: &mut Vec<MaterializedApplyGroup>,
    touched_nodes: &mut BTreeSet<NodeId>,
    stage_identities: &[StageSemanticIdentity],
    semantic_batch: &mut StageSemanticBatch,
    stage_record: &mut StageExecutionRecord,
) -> Result<(), SignalError> {
    if groups.is_empty() {
        return Ok(());
    }

    let merged_updates = groups
        .iter()
        .flat_map(|group| group.updates.iter().cloned())
        .collect::<Vec<_>>();
    apply_group_with_rollback(graph, &merged_updates, true)?;
    stage_record.apply_mode = Some(ParallelApplyMode::GroupedConcurrentApply);
    stage_record.apply_group_count += groups.len() as u32;
    stage_record.concurrent_apply_task_count += groups
        .iter()
        .map(|group| group.tasks.len() as u32)
        .sum::<u32>();

    let completed_groups = std::mem::take(groups);
    for group in completed_groups {
        semantic_batch.push_segment(build_group_segment(graph, &group.tasks, stage_identities)?);
    }
    touched_nodes.clear();
    Ok(())
}

fn apply_group_with_rollback(
    graph: &mut SignalGraph,
    updates: &[(crate::data::handle::NodeId, NodeEntry)],
    parallel: bool,
) -> Result<(), SignalError> {
    let originals = updates
        .iter()
        .map(|(node, _)| Ok((*node, graph.get_entry(*node)?.clone())))
        .collect::<Result<Vec<_>, SignalError>>()?;

    let apply_result = if parallel {
        graph.replace_entries_parallel(updates)
    } else {
        for (node, entry) in updates {
            graph.replace_entry(*node, entry.clone())?;
        }
        Ok(())
    };

    if let Err(err) = apply_result {
        for (node, entry) in originals {
            graph.replace_entry(node, entry)?;
        }
        return Err(err);
    }
    Ok(())
}

fn build_group_segment(
    graph: &mut SignalGraph,
    tasks: &[TaskPatch],
    stage_identities: &[StageSemanticIdentity],
) -> Result<super::semantic::SemanticSegment, SignalError> {
    let updates = tasks
        .iter()
        .map(|patch| {
            let identity = stage_identities
                .get(patch.task_index)
                .copied()
                .expect("stage identities should exist for every task");
            Ok(SemanticTaskUpdate {
                task_index: patch.task_index,
                node: patch.node,
                identity,
                before_state: patch.before_state,
                before_trace: patch.before_trace.clone(),
                after_state: graph.get_state(patch.node)?,
                dependency_updates: patch.dependency_updates,
                recomputed: patch.recomputed,
                partition_aware: patch.partition_aware,
            })
        })
        .collect::<Result<Vec<_>, SignalError>>()?;
    Ok(segment_for_updates(updates))
}

#[allow(clippy::too_many_arguments)]
fn apply_serial_fallback_patch(
    graph: &mut SignalGraph,
    stage: &super::types::ExecutionStage,
    patch: super::precompute::PreparedTaskPatch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    plan: &EvaluationPlan,
    stage_identities: &[StageSemanticIdentity],
    semantic_batch: &mut StageSemanticBatch,
) -> Result<(), SignalError> {
    let identity = stage_identities[patch.task_index];
    let recomputed = matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(
            patch.prepared.origin,
            PreparedEvaluationOrigin::MemoizedReuse
        );
    let partition_aware = !patch.prepared.result.changed_regions.is_empty();
    let before_state = graph.get_state(patch.node)?;
    let before_trace = graph.get_entry(patch.node)?.get_trace_summary().cloned();
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
                Some(stage.index),
                Some(patch.node),
                Some(executor),
                Some(identity.record_id),
                Some(plan.summary.clone()),
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
        before_state,
        before_trace,
        after_state,
        dependency_updates,
        recomputed,
        partition_aware,
    }));
    Ok(())
}
