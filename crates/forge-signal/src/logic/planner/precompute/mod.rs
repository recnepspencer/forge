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
use crate::data::node::EvaluationCondition;
#[cfg(feature = "parallel")]
use crate::data::proof::{LocallyOrderedShard, MergeableOrderedStream, OrderedStreamMergeError};
use crate::data::proof::{OrderedStreamItem, SingleConsumer};
use crate::logic::evaluation::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver,
};
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

#[cfg(feature = "parallel")]
use self::executor_pool::PlannerExecutorPool;
use super::types::EligibleTask;
#[cfg(feature = "parallel")]
use super::types::ParallelExecutionPolicy;
use super::validation::capture_current_dependencies_without_refresh;
use crate::data::comparator::ComparatorPolicyResolver;

#[derive(Debug, Clone)]
pub(in crate::logic::planner) struct PreparedTaskPatch {
    pub task_index: usize,
    pub node: NodeId,
    pub prepared: PreparedEvaluation,
}

impl OrderedStreamItem for (usize, PreparedEvaluation) {
    type OrderKey = usize;

    fn order_key(&self) -> Self::OrderKey {
        self.0
    }
}

impl OrderedStreamItem for PreparedTaskPatch {
    type OrderKey = usize;

    fn order_key(&self) -> Self::OrderKey {
        self.task_index
    }
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

    pub fn into_patches(self, tasks: &[EligibleTask]) -> Vec<PreparedTaskPatch> {
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
    tasks: &[EligibleTask],
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
    tasks: &[EligibleTask],
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
            .map(|index_chunk| {
                let mut chunk_results = Vec::with_capacity(index_chunk.len());
                for &task_index in index_chunk {
                    let task = &tasks[task_index];
                    let view = snapshot.read_view(task.node);
                    chunk_results.push((task_index, precompute(task.node, &view)?));
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
            .map(|prepared| prepared.expect("every task should have a prepared evaluation"))
            .collect())
    })
}

#[cfg(feature = "parallel")]
pub(super) fn build_parallel_stage_patches<F>(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
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
        let computed = compute_indices
            .par_chunks(chunk_size)
            .map(|index_chunk| {
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
                Ok::<LocallyOrderedShard<PreparedTaskPatch>, SignalError>(LocallyOrderedShard::new(
                    chunk_patches,
                ))
            })
            .collect::<Result<Vec<_>, SignalError>>()?;
        let prevalidated =
            LocallyOrderedShard::new(prepatched.into_iter().flatten().collect::<Vec<_>>());
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
fn parallel_duplicate_task_index(_error: OrderedStreamMergeError<usize>) -> SignalError {
    SignalError::internal("parallel ordered merge encountered duplicate task index")
}

fn prevalidate_stage_tasks(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Vec<Option<PreparedEvaluation>>, SignalError> {
    let mut prevalidated = Vec::with_capacity(tasks.len());
    for task in tasks {
        let prepared = if let Some(prepared) = prepare_condition_outcome_if_blocked(graph, task)? {
            Some(prepared)
        } else {
            prepare_validated_clean_if_unchanged(graph, task, comparator_resolver)?
        };
        prevalidated.push(prepared);
    }
    Ok(prevalidated)
}

fn prepare_condition_outcome_if_blocked(
    graph: &mut SignalGraph,
    task: &super::types::EligibleTask,
) -> Result<Option<PreparedEvaluation>, SignalError> {
    let dirty_aspects = graph.node_dirty_aspects(task.node)?;
    let required_context = graph.get_contract(task.node)?.semantics.required_context;
    let max_dependency_delta = max_dependency_delta(graph, task.node)?;
    let ctx = ConditionEvaluationContext {
        node: task.node,
        request_mode: task.request_mode,
        dirty_aspects,
        max_dependency_delta,
        required_context,
    };
    let has_dependency_snapshot = !graph.get_dep_snapshot(task.node)?.entries().is_empty();
    let mut default_resolver = DefaultConditionResolver;

    match graph.node_eval_config(task.node)?.condition.clone() {
        EvaluationCondition::Always => Ok(None),
        EvaluationCondition::AspectFilter(mask) => {
            if dirty_aspects.is_empty() || dirty_aspects.intersects(mask) {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::deferred_by_condition(),
                )
            }
        }
        EvaluationCondition::OnDemand => Ok(None),
        EvaluationCondition::DeltaThreshold(threshold) => {
            if !has_dependency_snapshot
                || dirty_aspects.is_empty()
                || (max_dependency_delta as f64) > threshold
            {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::reverted_clean_by_condition(),
                )
            }
        }
        EvaluationCondition::Debounce(quiet_period_ms) => {
            if default_resolver.debounce_ready(quiet_period_ms, &ctx)? {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::deferred_by_condition(),
                )
            }
        }
        EvaluationCondition::Custom(key) => {
            if default_resolver.resolve_custom(&key, &ctx)? {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::deferred_by_condition(),
                )
            }
        }
    }
}

fn prepare_condition_blocked_result(
    graph: &mut SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
) -> Result<Option<PreparedEvaluation>, SignalError> {
    let dependencies = capture_current_dependencies_without_refresh(graph, node)?;
    Ok(Some(prepared.with_dependencies(dependencies)))
}

fn prepare_validated_clean_if_unchanged(
    graph: &mut SignalGraph,
    task: &super::types::EligibleTask,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Option<PreparedEvaluation>, SignalError> {
    if matches!(
        task.request_mode,
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand
    ) {
        return Ok(None);
    }

    if !matches!(
        task.admission.node_state_at_admission,
        Some(crate::data::node::NodeState::MaybeStale)
    ) {
        return Ok(None);
    }

    if task.admission.dirty_partition_scopes_present {
        return Ok(None);
    }

    if !task
        .admission
        .maybe_stale
        .is_some_and(|admission| admission.unchanged_at_admission)
    {
        return Ok(None);
    }

    let dependencies = capture_current_dependencies_without_refresh(graph, task.node)?;
    Ok(Some(
        PreparedEvaluation::validated_clean().with_dependencies(dependencies),
    ))
}

fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for snapshot_entry in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            continue;
        }
        let current_version = graph.node_version_for_scope(
            snapshot_entry.source,
            snapshot_entry.aspect,
            snapshot_entry.scope.as_ref(),
        )?;
        max_delta = max_delta.max(current_version.abs_diff(snapshot_entry.cached_version));
    }
    Ok(max_delta)
}
