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
use crate::data::temporal::{
    ClockTick, DeferredTemporalEligibility, LoweredTemporalEligibility, ReadyTemporalEligibility,
    RuntimeClockBasis, TemporalCondition,
};
use crate::logic::evaluation::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver,
    TemporalConditionResolver,
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

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TemporalLoweringContext {
    runtime_clock_basis: Option<RuntimeClockBasis>,
}

impl TemporalLoweringContext {
    pub(crate) fn graph_only() -> Self {
        Self {
            runtime_clock_basis: None,
        }
    }

    pub(crate) fn runtime_clock_basis(runtime_clock_basis: RuntimeClockBasis) -> Self {
        Self {
            runtime_clock_basis: Some(runtime_clock_basis),
        }
    }

    fn runtime_tick_for(self, domain: crate::data::temporal::ClockDomain) -> Option<ClockTick> {
        self.runtime_clock_basis
            .filter(|basis| basis.domain() == domain)
            .map(RuntimeClockBasis::current_tick)
    }
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
    temporal_lowering: TemporalLoweringContext,
) -> Result<Vec<PreparedEvaluation>, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let validated = prevalidate_stage_tasks(graph, tasks, comparator_resolver, temporal_lowering)?;
    let snapshot = ExecutionSnapshot::new(&*graph);
    let mut prepared = Vec::with_capacity(tasks.len());
    for (task, prevalidated) in tasks.iter().zip(validated.into_iter()) {
        match prevalidated {
            PrevalidatedTask::Prepared(prepared_task) => {
                prepared.push(prepared_task);
                continue;
            }
            PrevalidatedTask::NeedsCompute { temporal_ready } => {
                let view = snapshot.read_view(task.node);
                let mut prepared_task = precompute(task.node, &view)?;
                if let Some(temporal_ready) = temporal_ready {
                    prepared_task = prepared_task.with_temporal_eligibility(
                        LoweredTemporalEligibility::Ready(temporal_ready),
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
    policy: ParallelExecutionPolicy,
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
        prevalidate_stage_tasks(graph, tasks, comparator_resolver, temporal_lowering)?;
    let mut compute_indices = Vec::new();
    for (task_index, prepared_task) in prepared.iter().enumerate() {
        if matches!(prepared_task, PrevalidatedTask::NeedsCompute { .. }) {
            compute_indices.push(task_index);
        }
    }
    let ready_temporal = prepared
        .iter()
        .map(PrevalidatedTask::temporal_ready)
        .collect::<Vec<_>>();

    if compute_indices.is_empty() {
        return Ok(prepared
            .into_iter()
            .map(|prepared| prepared.into_prepared())
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
                            LoweredTemporalEligibility::Ready(temporal_ready),
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
            .map(|prepared| prepared.into_prepared())
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
        prevalidate_stage_tasks(graph, tasks, comparator_resolver, temporal_lowering)?;
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
    let mut compute_indices = Vec::new();
    for (task_index, prepared) in prevalidated.into_iter().enumerate() {
        if matches!(prepared, PrevalidatedTask::NeedsCompute { .. }) {
            compute_indices.push(task_index);
        }
    }

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
                            LoweredTemporalEligibility::Ready(temporal_ready),
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
fn parallel_duplicate_task_index(_error: OrderedStreamMergeError<usize>) -> SignalError {
    SignalError::internal("parallel ordered merge encountered duplicate task index")
}

fn prevalidate_stage_tasks(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: TemporalLoweringContext,
) -> Result<Vec<PrevalidatedTask>, SignalError> {
    let mut prevalidated = Vec::with_capacity(tasks.len());
    for task in tasks {
        let prepared = if let Some(prepared) =
            prepare_condition_outcome_if_blocked(graph, task, temporal_lowering)?
        {
            prepared
        } else {
            prepare_validated_clean_if_unchanged(graph, task, comparator_resolver)?.unwrap_or(
                PrevalidatedTask::NeedsCompute {
                    temporal_ready: None,
                },
            )
        };
        prevalidated.push(prepared);
    }
    Ok(prevalidated)
}

fn prepare_condition_outcome_if_blocked(
    graph: &mut SignalGraph,
    task: &super::types::EligibleTask,
    temporal_lowering: TemporalLoweringContext,
) -> Result<Option<PrevalidatedTask>, SignalError> {
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
        EvaluationCondition::Temporal(condition) => {
            graph
                .telemetry_mut()
                .temporal
                .temporal_eligibility_lowering_count += 1;
            lower_temporal_condition(
                graph,
                task.node,
                condition,
                &ctx,
                temporal_lowering,
                &mut default_resolver,
            )
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
) -> Result<Option<PrevalidatedTask>, SignalError> {
    let dependencies = capture_current_dependencies_without_refresh(graph, node)?;
    Ok(Some(PrevalidatedTask::Prepared(
        prepared.with_dependencies(dependencies),
    )))
}

fn lower_temporal_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    condition: TemporalCondition,
    ctx: &ConditionEvaluationContext,
    temporal_lowering: TemporalLoweringContext,
    resolver: &mut impl TemporalConditionResolver,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    if let Some(prevalidated) =
        lower_temporal_condition_from_runtime_clock(condition.clone(), temporal_lowering)
    {
        return match prevalidated {
            PrevalidatedTask::Prepared(prepared) => {
                prepare_condition_blocked_result(graph, node, prepared)
            }
            other => Ok(Some(other)),
        };
    }

    if resolver.resolve_temporal(&condition, ctx)? {
        Ok(Some(PrevalidatedTask::NeedsCompute {
            temporal_ready: Some(ReadyTemporalEligibility::resolver_backed(condition)),
        }))
    } else {
        prepare_condition_blocked_result(
            graph,
            node,
            PreparedEvaluation::deferred_by_time(LoweredTemporalEligibility::resolver_deferred(
                condition,
            )),
        )
    }
}

fn lower_temporal_condition_from_runtime_clock(
    condition: TemporalCondition,
    temporal_lowering: TemporalLoweringContext,
) -> Option<PrevalidatedTask> {
    match condition.clone() {
        TemporalCondition::AtOrAfter(at_or_after) => {
            let authority_tick = temporal_lowering.runtime_tick_for(at_or_after.clock_domain())?;
            if authority_tick >= at_or_after.tick() {
                Some(PrevalidatedTask::NeedsCompute {
                    temporal_ready: Some(ReadyTemporalEligibility::runtime_clock_backed(
                        condition,
                        authority_tick,
                    )),
                })
            } else {
                Some(PrevalidatedTask::Prepared(
                    PreparedEvaluation::deferred_by_time(LoweredTemporalEligibility::Deferred(
                        DeferredTemporalEligibility::runtime_clock_backed(
                            condition,
                            authority_tick,
                        ),
                    )),
                ))
            }
        }
        _ => None,
    }
}

fn prepare_validated_clean_if_unchanged(
    graph: &mut SignalGraph,
    task: &super::types::EligibleTask,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Option<PrevalidatedTask>, SignalError> {
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
    Ok(Some(PrevalidatedTask::Prepared(
        PreparedEvaluation::validated_clean().with_dependencies(dependencies),
    )))
}

#[derive(Debug, Clone)]
enum PrevalidatedTask {
    Prepared(PreparedEvaluation),
    NeedsCompute {
        temporal_ready: Option<ReadyTemporalEligibility>,
    },
}

impl PrevalidatedTask {
    #[cfg(feature = "parallel")]
    fn temporal_ready(&self) -> Option<ReadyTemporalEligibility> {
        match self {
            Self::Prepared(_) => None,
            Self::NeedsCompute { temporal_ready } => temporal_ready.clone(),
        }
    }

    #[cfg(feature = "parallel")]
    fn into_prepared(self) -> PreparedEvaluation {
        match self {
            Self::Prepared(prepared) => prepared,
            Self::NeedsCompute { .. } => {
                panic!("compute-needed task was converted into prepared output too early")
            }
        }
    }
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
