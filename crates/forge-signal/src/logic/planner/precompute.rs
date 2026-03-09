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
        let view = snapshot.read_view(task.node);
        prepared.push(precompute(task.node, &view)?);
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
pub(super) fn precompute_stage_parallel<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
    precompute: &F,
    policy: ParallelExecutionPolicy,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let chunk_size = policy.chunk_size_for(stage.tasks.len());
    let worker_count = policy.worker_count_for(stage.tasks.len());
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let joined = stage
            .tasks
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(chunk_index, task_chunk)| {
                let mut chunk_results = Vec::with_capacity(task_chunk.len());
                for task in task_chunk {
                    let view = snapshot.read_view(task.node);
                    chunk_results.push(precompute(task.node, &view)?);
                }
                Ok::<(usize, Vec<PreparedEvaluation>), SignalError>((chunk_index, chunk_results))
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        let mut prepared = Vec::with_capacity(stage.tasks.len());
        for (_, chunk_results) in joined {
            prepared.extend(chunk_results);
        }
        Ok(prepared)
    })
}

#[cfg(feature = "parallel")]
pub(super) fn build_parallel_stage_patches<F>(
    stage: &ExecutionStage,
    snapshot: &ExecutionSnapshot<'_>,
    precompute: &F,
    policy: ParallelExecutionPolicy,
) -> Result<Vec<PreparedTaskPatch>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let chunk_size = policy.chunk_size_for(stage.tasks.len());
    let worker_count = policy.worker_count_for(stage.tasks.len());
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let mut patches = stage
            .tasks
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(chunk_index, task_chunk)| {
                let mut chunk_patches = Vec::with_capacity(task_chunk.len());
                for (task_offset, task) in task_chunk.iter().enumerate() {
                    let task_index = chunk_index * chunk_size + task_offset;
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
