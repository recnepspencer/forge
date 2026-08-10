use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

#[cfg(feature = "parallel")]
use super::super::precompute::executor_pool::PlannerExecutorPool;
#[cfg(feature = "parallel")]
use crate::data::proof::{LocallyOrderedShard, MergeableOrderedStream};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::super::types::EligibleTask;
use super::eligibility::{prevalidate_stage_tasks, PrevalidatedTask};
#[cfg(feature = "parallel")]
use super::stage_data::PreparedTaskPatch;
use super::temporal::TemporalLoweringContext;

pub(super) fn precompute_stage_serial<F>(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let validated = prevalidate_stage_tasks(graph, tasks, comparator_resolver, &temporal_lowering)?;
    let snapshot = ExecutionSnapshot::new(&*graph);
    let mut prepared = Vec::with_capacity(tasks.len());
    for (task, prevalidated) in tasks.iter().zip(validated.into_iter()) {
        match prevalidated {
            PrevalidatedTask::Prepared(prepared_task) => prepared.push(prepared_task),
            PrevalidatedTask::NeedsCompute { temporal_ready } => {
                let view = snapshot.read_view(task.node);
                let mut prepared_task = precompute(task.node, &view)?;
                if let Some(temporal_ready) = temporal_ready {
                    prepared_task = prepared_task.with_temporal_eligibility(
                        crate::data::temporal::LoweredTemporalEligibility::Ready(temporal_ready),
                    );
                }
                prepared.push(prepared_task);
            }
        }
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
pub(super) fn precompute_stage_parallel<F>(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &F,
    policy: super::super::types::ParallelExecutionPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut prepared =
        prevalidate_stage_tasks(graph, tasks, comparator_resolver, &temporal_lowering)?;
    let compute_indices = prepared
        .iter()
        .enumerate()
        .filter_map(|(index, task)| {
            matches!(task, PrevalidatedTask::NeedsCompute { .. }).then_some(index)
        })
        .collect::<Vec<_>>();
    let ready_temporal = prepared
        .iter()
        .map(PrevalidatedTask::temporal_ready)
        .collect::<Vec<_>>();

    if compute_indices.is_empty() {
        return Ok(prepared
            .into_iter()
            .map(PrevalidatedTask::into_prepared)
            .collect());
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let joined = compute_indices
            .par_chunks(chunk_size)
            .map(|index_chunk| {
                let mut chunk_results = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    let mut prepared_task = precompute(task.node, &view)?;
                    if let Some(temporal_ready) = ready_temporal[task_index].clone() {
                        prepared_task = prepared_task.with_temporal_eligibility(
                            crate::data::temporal::LoweredTemporalEligibility::Ready(
                                temporal_ready,
                            ),
                        );
                    }
                    chunk_results.push((task_index, prepared_task));
                }
                Ok::<LocallyOrderedShard<(usize, PreparedEvaluation)>, SignalError>(
                    LocallyOrderedShard::new(chunk_results),
                )
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        let computed = MergeableOrderedStream::new(joined)
            .try_into_vec()
            .map_err(parallel_duplicate_task_index)?;
        for (task_index, prepared_task) in computed {
            prepared[task_index] = PrevalidatedTask::Prepared(prepared_task);
        }
        Ok(prepared
            .into_iter()
            .map(PrevalidatedTask::into_prepared)
            .collect())
    })
}

#[cfg(feature = "parallel")]
pub(super) fn build_parallel_stage_patches<F>(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    precompute: &F,
    policy: super::super::types::ParallelExecutionPolicy,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
) -> Result<Vec<PreparedTaskPatch>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let prevalidated =
        prevalidate_stage_tasks(graph, tasks, comparator_resolver, &temporal_lowering)?;
    let ready_temporal = prevalidated
        .iter()
        .map(PrevalidatedTask::temporal_ready)
        .collect::<Vec<_>>();
    let prepatched = prevalidated
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(task_index, prepared)| match prepared {
            PrevalidatedTask::Prepared(prepared) => Some(PreparedTaskPatch {
                task_index,
                node: tasks[task_index].node,
                prepared,
            }),
            PrevalidatedTask::NeedsCompute { .. } => None,
        })
        .collect::<Vec<_>>();
    let compute_indices = prevalidated
        .into_iter()
        .enumerate()
        .filter_map(|(index, task)| {
            matches!(task, PrevalidatedTask::NeedsCompute { .. }).then_some(index)
        })
        .collect::<Vec<_>>();

    if compute_indices.is_empty() {
        return Ok(prepatched);
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_indices.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let computed = compute_indices
            .par_chunks(chunk_size)
            .map(|index_chunk| {
                let mut chunk_patches = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    let mut prepared_task = precompute(task.node, &view)?;
                    if let Some(temporal_ready) = ready_temporal[task_index].clone() {
                        prepared_task = prepared_task.with_temporal_eligibility(
                            crate::data::temporal::LoweredTemporalEligibility::Ready(
                                temporal_ready,
                            ),
                        );
                    }
                    chunk_patches.push(PreparedTaskPatch {
                        task_index,
                        node: task.node,
                        prepared: prepared_task,
                    });
                }
                Ok::<LocallyOrderedShard<PreparedTaskPatch>, SignalError>(LocallyOrderedShard::new(
                    chunk_patches,
                ))
            })
            .collect::<Result<Vec<_>, SignalError>>()?;
        let prevalidated = LocallyOrderedShard::new(prepatched);
        let patches =
            MergeableOrderedStream::new(std::iter::once(prevalidated).chain(computed.into_iter()))
                .try_into_vec()
                .map_err(parallel_duplicate_task_index)?;
        for window in patches.windows(2) {
            if let [left, right] = window {
                if left.node == right.node {
                    return Err(SignalError::internal(
                        "parallel patch merge encountered duplicate task target",
                    ));
                }
            }
        }
        Ok(patches)
    })
}

#[cfg(feature = "parallel")]
fn parallel_duplicate_task_index(
    _error: crate::data::proof::OrderedStreamMergeError<usize>,
) -> SignalError {
    SignalError::internal("parallel ordered merge encountered duplicate task index")
}
