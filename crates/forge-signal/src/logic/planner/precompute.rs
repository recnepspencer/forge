#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

#[cfg(feature = "parallel")]
use super::executor_pool::PlannerExecutorPool;
use super::types::ExecutionStage;
#[cfg(feature = "parallel")]
use super::types::ParallelExecutionPolicy;
use super::validation::{capture_current_dependencies, preview_maybe_stale};
use crate::data::comparator::ComparatorPolicyResolver;

#[derive(Debug, Clone)]
pub(super) struct PreparedTaskPatch {
    pub task_index: usize,
    pub node: NodeId,
    pub prepared: PreparedEvaluation,
}

pub(super) enum StageExecutionData {
    Prepared(Vec<PreparedEvaluation>),
    #[cfg(feature = "parallel")]
    Patched(Vec<PreparedTaskPatch>),
}

impl StageExecutionData {
    pub fn len(&self) -> usize {
        match self {
            Self::Prepared(prepared) => prepared.len(),
            #[cfg(feature = "parallel")]
            Self::Patched(patches) => patches.len(),
        }
    }

    pub fn into_patches(self, stage: &ExecutionStage) -> Vec<PreparedTaskPatch> {
        match self {
            Self::Prepared(prepared) => prepared
                .into_iter()
                .enumerate()
                .map(|(task_index, prepared)| PreparedTaskPatch {
                    task_index,
                    node: stage.tasks[task_index].node,
                    prepared,
                })
                .collect(),
            #[cfg(feature = "parallel")]
            Self::Patched(patches) => patches,
        }
    }
}

pub(super) fn precompute_stage_serial<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
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
    let mut prepared = Vec::with_capacity(stage.tasks.len());
    for task in &stage.tasks {
        prepared.push(prepare_stage_task(
            task,
            snapshot,
            precompute,
            comparator_resolver,
        )?);
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
pub(super) fn precompute_stage_parallel<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
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
    let mut prepared = Vec::with_capacity(stage.tasks.len());
    let mut compute_indices = Vec::new();
    for task in &stage.tasks {
        match prepare_validated_clean_if_unchanged(task, snapshot, comparator_resolver)? {
            Some(prepared_task) => prepared.push(Some(prepared_task)),
            None => {
                prepared.push(None);
                compute_indices.push(prepared.len() - 1);
            }
        }
    }

    if compute_indices.is_empty() {
        return Ok(prepared
            .into_iter()
            .map(|prepared| prepared.expect("validated task should be populated"))
            .collect());
    }

    let chunk_size = policy.chunk_size_for(stage.tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let joined = compute_indices
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(chunk_index, index_chunk)| {
                let mut chunk_results = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &stage.tasks[task_index];
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
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
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
    let mut prepatched = Vec::with_capacity(stage.tasks.len());
    let mut compute_indices = Vec::new();
    for (task_index, task) in stage.tasks.iter().enumerate() {
        match prepare_validated_clean_if_unchanged(task, snapshot, comparator_resolver)? {
            Some(prepared) => prepatched.push(Some(PreparedTaskPatch {
                task_index,
                node: task.node,
                prepared,
            })),
            None => {
                prepatched.push(None);
                compute_indices.push(task_index);
            }
        }
    }

    if compute_indices.is_empty() {
        return Ok(prepatched
            .into_iter()
            .map(|patch| patch.expect("validated task patch should be populated"))
            .collect());
    }

    let chunk_size = policy.chunk_size_for(stage.tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let mut patches = compute_indices
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(_chunk_index, index_chunk)| {
                let mut chunk_patches = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &stage.tasks[task_index];
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

fn prepare_stage_task<F>(
    task: &super::types::EvaluationTask,
    snapshot: &ExecutionSnapshot<'_>,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<PreparedEvaluation, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    if let Some(prepared) =
        prepare_validated_clean_if_unchanged(task, snapshot, comparator_resolver)?
    {
        return Ok(prepared);
    }
    let view = snapshot.read_view(task.node);
    precompute(task.node, &view)
}

fn prepare_validated_clean_if_unchanged(
    task: &super::types::EvaluationTask,
    snapshot: &ExecutionSnapshot<'_>,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Option<PreparedEvaluation>, SignalError> {
    if matches!(
        task.request_mode,
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand
    ) {
        return Ok(None);
    }

    let graph = snapshot.graph();
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
