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
    stage_index: u32,
    readiness_epoch: crate::data::proof::invalidation::progression::InvalidationReadinessEpoch,
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
    let validated = prevalidate_stage_tasks(
        graph,
        tasks,
        stage_index,
        readiness_epoch,
        comparator_resolver,
        &temporal_lowering,
    )?;
    let snapshot = ExecutionSnapshot::new(&*graph);
    let mut prepared = Vec::with_capacity(tasks.len());
    for (task, prevalidated) in tasks.iter().zip(validated.into_iter()) {
        match prevalidated {
            PrevalidatedTask::Prepared(prepared_task) => prepared.push(prepared_task),
            PrevalidatedTask::NeedsCompute {
                temporal_ready,
                ready_invalidation,
            } => {
                let view = snapshot.read_view(task.node);
                let compute = || precompute(task.node, &view);
                let mut prepared_task = match ready_invalidation {
                    Some(ready) => crate::logic::invalidation::scheduling::execute_ready(
                        view.graph(),
                        ready,
                        compute,
                    )?,
                    None => compute()?,
                };
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
    stage_index: u32,
    readiness_epoch: crate::data::proof::invalidation::progression::InvalidationReadinessEpoch,
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
    let prevalidated = prevalidate_stage_tasks(
        graph,
        tasks,
        stage_index,
        readiness_epoch,
        comparator_resolver,
        &temporal_lowering,
    )?;
    let (mut prepared, compute_work) = split_prevalidated_work(prevalidated);

    if compute_work.is_empty() {
        return prepared
            .into_iter()
            .map(|prepared| prepared.ok_or_else(|| SignalError::internal("prepared task missing")))
            .collect();
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_work.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let joined = into_chunks(compute_work, chunk_size)
            .into_par_iter()
            .map(|chunk| {
                let mut chunk_results = Vec::with_capacity(chunk.len());
                for work in chunk {
                    let task_index = work.task_index;
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    let prepared_task = compute_work_item(work, task.node, &view, precompute)?;
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
            prepared[task_index] = Some(prepared_task);
        }
        Ok(prepared
            .into_iter()
            .map(|prepared| prepared.expect("every compute slot was filled"))
            .collect())
    })
}

#[cfg(feature = "parallel")]
pub(super) fn build_parallel_stage_patches<F>(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    stage_index: u32,
    readiness_epoch: crate::data::proof::invalidation::progression::InvalidationReadinessEpoch,
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
    let prevalidated = prevalidate_stage_tasks(
        graph,
        tasks,
        stage_index,
        readiness_epoch,
        comparator_resolver,
        &temporal_lowering,
    )?;
    let (prepared, compute_work) = split_prevalidated_work(prevalidated);
    let prepatched = prepared
        .into_iter()
        .enumerate()
        .filter_map(|(task_index, prepared)| {
            prepared.map(|prepared| PreparedTaskPatch {
                task_index,
                node: tasks[task_index].node,
                prepared,
            })
        })
        .collect::<Vec<_>>();

    if compute_work.is_empty() {
        return Ok(prepatched);
    }

    let chunk_size = policy.chunk_size_for(tasks.len());
    let worker_count = policy.worker_count_for(compute_work.len());
    let snapshot = ExecutionSnapshot::new(&*graph);
    let pool = PlannerExecutorPool::shared(worker_count)?;
    pool.install(|| {
        let computed = into_chunks(compute_work, chunk_size)
            .into_par_iter()
            .map(|chunk| {
                let mut chunk_patches = Vec::with_capacity(chunk.len());
                for work in chunk {
                    let task_index = work.task_index;
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    let prepared_task = compute_work_item(work, task.node, &view, precompute)?;
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
struct ComputeWork {
    task_index: usize,
    temporal_ready: Option<crate::data::temporal::ReadyTemporalEligibility>,
    ready_invalidation:
        Option<crate::data::proof::invalidation::progression::ReadyInvalidationBatch>,
}

#[cfg(feature = "parallel")]
fn split_prevalidated_work(
    prevalidated: Vec<PrevalidatedTask>,
) -> (Vec<Option<PreparedEvaluation>>, Vec<ComputeWork>) {
    let mut prepared = Vec::with_capacity(prevalidated.len());
    let mut compute = Vec::new();
    for (task_index, task) in prevalidated.into_iter().enumerate() {
        match task {
            PrevalidatedTask::Prepared(value) => prepared.push(Some(value)),
            PrevalidatedTask::NeedsCompute {
                temporal_ready,
                ready_invalidation,
            } => {
                prepared.push(None);
                compute.push(ComputeWork {
                    task_index,
                    temporal_ready,
                    ready_invalidation,
                });
            }
        }
    }
    (prepared, compute)
}

#[cfg(feature = "parallel")]
fn into_chunks(mut work: Vec<ComputeWork>, chunk_size: usize) -> Vec<Vec<ComputeWork>> {
    let mut chunks = Vec::new();
    while !work.is_empty() {
        let remaining = work.split_off(work.len().min(chunk_size));
        chunks.push(work);
        work = remaining;
    }
    chunks
}

#[cfg(feature = "parallel")]
fn compute_work_item<F>(
    work: ComputeWork,
    node: NodeId,
    view: &crate::logic::prepared::ExecutionReadView<'_>,
    precompute: &F,
) -> Result<PreparedEvaluation, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let compute = || precompute(node, view);
    let mut prepared = match work.ready_invalidation {
        Some(ready) => {
            crate::logic::invalidation::scheduling::execute_ready(view.graph(), ready, compute)?
        }
        None => compute()?,
    };
    if let Some(temporal_ready) = work.temporal_ready {
        prepared = prepared.with_temporal_eligibility(
            crate::data::temporal::LoweredTemporalEligibility::Ready(temporal_ready),
        );
    }
    Ok(prepared)
}

#[cfg(feature = "parallel")]
fn parallel_duplicate_task_index(
    _error: crate::data::proof::OrderedStreamMergeError<usize>,
) -> SignalError {
    SignalError::internal("parallel ordered merge encountered duplicate task index")
}
