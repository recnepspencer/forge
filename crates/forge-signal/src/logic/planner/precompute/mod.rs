#[cfg(feature = "parallel")]
pub(crate) mod admission;
pub(crate) mod dispatch;
#[cfg(feature = "parallel")]
pub(crate) mod executor_pool;
pub(crate) mod reporting;
pub(crate) mod stage;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::proof::SingleConsumer;
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

#[cfg(feature = "parallel")]
use self::executor_pool::PlannerExecutorPool;
use super::types::EvaluationTask;
#[cfg(feature = "parallel")]
use super::types::ParallelExecutionPolicy;
use super::validation::{capture_current_dependencies, preview_maybe_stale};
use crate::data::comparator::ComparatorPolicyResolver;

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct PreparedTaskPatch {
    pub task_index: usize,
    pub node: NodeId,
    pub prepared: PreparedEvaluation,
}

pub(in crate::logic::planner) enum StageExecutionData {
    Prepared(SingleConsumer<Vec<PreparedEvaluation>>),
    #[cfg(feature = "parallel")]
    Patched(SingleConsumer<Vec<PreparedTaskPatch>>),
}

impl StageExecutionData {
    pub fn len(&self) -> usize {
        match self {
            Self::Prepared(prepared) => prepared.as_ref().len(),
            #[cfg(feature = "parallel")]
            Self::Patched(patches) => patches.as_ref().len(),
        }
    }

    pub fn into_patches(self, tasks: &[EvaluationTask]) -> Vec<PreparedTaskPatch> {
        match self {
            Self::Prepared(prepared) => prepared
                .into_inner()
                .into_iter()
                .enumerate()
                .map(|(task_index, prepared)| PreparedTaskPatch {
                    task_index,
                    node: tasks[task_index].node,
                    prepared,
                })
                .collect(),
            #[cfg(feature = "parallel")]
            Self::Patched(patches) => patches.into_inner(),
        }
    }
}

pub(super) fn precompute_stage_serial<F>(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let validated = prevalidate_stage_tasks(graph, tasks, comparator_resolver)?;
    let snapshot = ExecutionSnapshot::new(&*graph);
    let mut prepared = Vec::with_capacity(tasks.len());
    for (task, prevalidated) in tasks.iter().zip(validated.into_iter()) {
        if let Some(prepared_task) = prevalidated {
            prepared.push(prepared_task);
            continue;
        }
        let view = snapshot.read_view(task.node);
        prepared.push(precompute(task.node, &view)?);
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
pub(super) fn precompute_stage_parallel<F>(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &F,
    policy: ParallelExecutionPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut prepared = prevalidate_stage_tasks(graph, tasks, comparator_resolver)?;
    let mut compute_indices = Vec::new();
    for (task_index, prepared_task) in prepared.iter().enumerate() {
        if prepared_task.is_none() {
            compute_indices.push(task_index);
        }
    }

    if compute_indices.is_empty() {
        return Ok(prepared
            .into_iter()
            .map(|prepared| prepared.expect("validated task should be populated"))
            .collect());
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let joined = compute_indices
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(chunk_index, index_chunk)| {
                let mut chunk_results = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    chunk_results.push((task_index, precompute(task.node, &view)?));
                }
                Ok::<(usize, Vec<(usize, PreparedEvaluation)>), SignalError>((
                    chunk_index,
                    chunk_results,
                ))
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        let mut computed = joined
            .into_iter()
            .flat_map(|(_, chunk_results)| chunk_results)
            .collect::<Vec<_>>();
        computed.sort_by_key(|(task_index, _)| *task_index);
        for (task_index, prepared_task) in computed {
            prepared[task_index] = Some(prepared_task);
        }
        Ok(prepared
            .into_iter()
            .map(|prepared| prepared.expect("every task should have a prepared evaluation"))
            .collect())
    })
}

#[cfg(feature = "parallel")]
pub(super) fn build_parallel_stage_patches<F>(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    precompute: &F,
    policy: ParallelExecutionPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Vec<PreparedTaskPatch>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let prepatched = prevalidate_stage_tasks(graph, tasks, comparator_resolver)?
        .into_iter()
        .enumerate()
        .map(|(task_index, prepared)| {
            prepared.map(|prepared| PreparedTaskPatch {
                task_index,
                node: tasks[task_index].node,
                prepared,
            })
        })
        .collect::<Vec<_>>();
    let mut compute_indices = Vec::new();
    for (task_index, patch) in prepatched.iter().enumerate() {
        if patch.is_none() {
            compute_indices.push(task_index);
        }
    }

    if compute_indices.is_empty() {
        return Ok(prepatched
            .into_iter()
            .map(|patch| patch.expect("validated task patch should be populated"))
            .collect());
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let mut patches = compute_indices
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(_chunk_index, index_chunk)| {
                let mut chunk_patches = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    chunk_patches.push(PreparedTaskPatch {
                        task_index,
                        node: task.node,
                        prepared: precompute(task.node, &view)?,
                    });
                }
                Ok::<Vec<PreparedTaskPatch>, SignalError>(chunk_patches)
            })
            .collect::<Result<Vec<_>, SignalError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        for patch in prepatched.into_iter().flatten() {
            patches.push(patch);
        }
        patches.sort_by_key(|patch| patch.task_index);
        for window in patches.windows(2) {
            if let [left, right] = window {
                if left.task_index == right.task_index || left.node == right.node {
                    return Err(SignalError::internal(
                        "parallel patch merge encountered duplicate task target",
                    ));
                }
            }
        }
        Ok(patches)
    })
}

fn prevalidate_stage_tasks(
    graph: &mut SignalGraph,
    tasks: &[EvaluationTask],
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Vec<Option<PreparedEvaluation>>, SignalError> {
    let mut prevalidated = Vec::with_capacity(tasks.len());
    for task in tasks {
        prevalidated.push(prepare_validated_clean_if_unchanged(
            graph,
            task,
            comparator_resolver,
        )?);
    }
    Ok(prevalidated)
}

fn prepare_validated_clean_if_unchanged(
    graph: &mut SignalGraph,
    task: &super::types::EvaluationTask,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Option<PreparedEvaluation>, SignalError> {
    if matches!(
        task.request_mode,
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand
    ) {
        return Ok(None);
    }

    if !matches!(
        graph.get_state(task.node)?,
        crate::data::node::NodeState::MaybeStale
    ) {
        return Ok(None);
    }

    if !graph
        .get_entry(task.node)?
        .get_dirty_partition_scopes()
        .is_empty()
    {
        return Ok(None);
    }

    let preview = preview_maybe_stale(graph, task.node, comparator_resolver)?;
    if !preview.unchanged {
        return Ok(None);
    }

    let dependencies = capture_current_dependencies(graph, task.node)?;
    Ok(Some(
        PreparedEvaluation::validated_clean().with_dependencies(dependencies),
    ))
}
