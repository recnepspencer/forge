use std::time::Instant;

use crate::data::aspect::AspectMask;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::CanonicalChangedRegions;
use crate::data::performance::{
    AuthorityPolicy, PathClass, ResolvedExecutionStrategy, ResolvedMaintenanceStrategy,
};
use crate::data::proof::{
    DedupedNodeBatch, DirtyDelta, PartitionScopeSet, SingleConsumer, SnapshotBatchCommit,
    SortedSourceBatch, StructuralDelta, TouchedScopeSummary,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::SignalRuntimePolicy;
use crate::logic::evaluation::{
    apply_prepared_evaluation_after_dependencies_with_policy,
    collect_effect_dependency_inputs_batch,
};
use crate::logic::explain::{RewiringDependency, RewiringSummary};
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluationOrigin, PreparedEvaluationOutcome,
};

use super::super::execution::task_reporting::record_execution_failure;
use super::super::execution::StageSlice;
use super::super::precompute::{PreparedTaskPatch, StageExecutionData};
use super::super::semantic::{
    finalize_stage_batch, segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch,
    StageSemanticIdentity,
};
use super::super::stage_precompute::StagePrecomputeResult;
use super::super::types::{
    ApplyFootprint, DisjointApplyGroup, EligibleTask, ExecutionReport, LoweredStagePlan,
    LoweredTask, LoweredTaskExecution, PlanSummary, StageExecutionRecord, StageExecutor,
};
use super::workspace::StageScratch;
#[cfg(feature = "parallel")]
use super::groups::build_stage_apply_groups;

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
    let lowered = build_lowered_stage_plan(
        graph,
        stage.index,
        stage.tasks,
        precomputed.execution,
        executor,
    )?;
    if let Some(dirty) = lowered.dirty_delta.dirty.as_ref() {
        graph.telemetry_mut().invalidation.dirty_delta_breadth +=
            dirty.touched_nodes.len() as u64;
        graph.telemetry_mut().storage.structural_delta_size +=
            dirty.changed_regions.as_slice().len() as u64 + dirty.touched_nodes.len() as u64;
    }
    graph.telemetry_mut().execution.apply_group_width_total += lowered
        .apply_groups
        .iter()
        .map(|group| group.task_indices.len() as u64)
        .sum::<u64>();
    graph.telemetry_mut().execution.max_apply_group_width = graph
        .telemetry()
        .execution
        .max_apply_group_width
        .max(
            lowered
                .apply_groups
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .max()
                .unwrap_or(0),
        );
    graph.telemetry_mut().execution.apply_group_disjoint_count +=
        lowered
            .apply_groups
            .iter()
            .map(|group| group.task_indices.len().saturating_sub(1) as u64)
            .sum::<u64>();
    match lowered.maintenance_strategy {
        ResolvedMaintenanceStrategy::Incremental | ResolvedMaintenanceStrategy::DensityAdaptive => {
            graph.telemetry_mut().planner.incremental_strategy_count += 1;
        }
        ResolvedMaintenanceStrategy::Rebuild => {
            graph.telemetry_mut().planner.rebuild_strategy_count += 1;
        }
    }
    let stage_scratch = run_lowered_apply_pass(
        graph,
        summary,
        lowered,
        comparator_resolver,
        executor,
        stage_identities,
        stage_record,
    )?;
    if !stage_scratch.pending_snapshots.is_empty() {
        graph.apply_snapshot_batch_commit(&SnapshotBatchCommit::from_pending_snapshots(
            stage_scratch.pending_snapshots.into_iter(),
        ))?;
    }
    let semantic_finalize_start = Instant::now();
    finalize_stage_batch(
        graph,
        stage.tasks,
        stage_scratch.semantic_batch.into_inner(),
        report,
        stage_record,
    )?;
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

fn build_lowered_stage_plan(
    graph: &mut SignalGraph,
    stage_index: u32,
    stage_tasks: &[EligibleTask],
    stage_execution: StageExecutionData,
    executor: StageExecutor,
) -> Result<LoweredStagePlan, SignalError> {
    let lowered_tasks = stage_execution
        .into_patches(stage_tasks)
        .into_iter()
        .map(|patch| lower_task_patch(graph, patch))
        .collect::<Result<Vec<_>, SignalError>>()?;
    let resolved_policy =
        SignalRuntimePolicy::from_profile(graph.diagnostics_profile()).resolve_performance_policy();
    let apply_groups = build_lowered_apply_groups(&lowered_tasks, executor);
    let dirty_delta = build_lowered_dirty_delta(&lowered_tasks);
    let touched_scope = build_touched_scope_summary(&lowered_tasks);
    let authority_policy = lowered_tasks
        .iter()
        .find(|task| {
            matches!(
                task.authority_policy,
                crate::data::node::AuthorityPolicy::AuthoritativeOnly
            )
        })
        .map(|task| task.authority_policy)
        .unwrap_or(resolved_policy.authority_policy);

    Ok(LoweredStagePlan {
        stage_index,
        tasks: lowered_tasks,
        apply_groups,
        dirty_delta: StructuralDelta::new(Some(dirty_delta), Some(touched_scope)),
        execution_strategy: resolved_policy.execution_strategy,
        maintenance_strategy: resolved_policy.maintenance_strategy,
        authority_policy,
    })
}

fn validate_lowered_stage_plan(lowered: &LoweredStagePlan) {
    let rich_task_count = lowered
        .tasks
        .iter()
        .filter(|task| matches!(task.path_class, PathClass::Rich))
        .count();
    let authoritative_task_count = lowered
        .tasks
        .iter()
        .filter(|task| matches!(task.authority_policy, AuthorityPolicy::AuthoritativeOnly))
        .count();
    let recomputed_task_count = lowered
        .tasks
        .iter()
        .filter(|task| task.execution.recomputed)
        .count();

    debug_assert!(
        lowered.task_count() == lowered.tasks.len(),
        "lowered task count must match staged task collection"
    );
    debug_assert!(
        lowered.dirty_delta.is_empty() || !lowered.tasks.is_empty(),
        "structural delta should only be populated for non-empty lowered stages"
    );
    debug_assert!(
        !matches!(
            lowered.execution_strategy,
            ResolvedExecutionStrategy::FullGraphPass
        ) || lowered.tasks.is_empty()
            || !lowered.apply_groups.is_empty(),
        "full-graph execution stages must still lower into apply groups"
    );
    debug_assert!(
        !matches!(
            lowered.maintenance_strategy,
            ResolvedMaintenanceStrategy::Rebuild
        ) || lowered.dirty_delta.dirty.is_some(),
        "rebuild-oriented stages must carry a narrowed dirty delta"
    );
    debug_assert!(
        !matches!(lowered.authority_policy, AuthorityPolicy::AuthoritativeOnly)
            || authoritative_task_count > 0,
        "authoritative lowered stages must include authoritative tasks"
    );
    debug_assert!(
        rich_task_count <= lowered.tasks.len(),
        "rich-path accounting must remain bounded by lowered tasks"
    );
    debug_assert!(
        recomputed_task_count <= lowered.tasks.len(),
        "recomputed-task accounting must remain bounded by lowered tasks"
    );
}

fn run_lowered_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    lowered: LoweredStagePlan,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    validate_lowered_stage_plan(&lowered);
    stage_record.authority_policy = Some(lowered.authority_policy);
    #[cfg(not(feature = "parallel"))]
    let _ = stage_record;
    let mut semantic_batch = StageSemanticBatch::default();
    let mut pending_snapshots = Vec::with_capacity(lowered.task_count());
    graph.reconcile_dependencies_batch(
        &lowered
            .tasks
            .iter()
            .map(|task| (task.node, task.dependency_inputs.clone()))
            .collect::<Vec<_>>(),
    )?;
    let dependency_updates = lowered
        .tasks
        .iter()
        .map(|task| task.execution.dependency_updates)
        .collect::<Vec<_>>();
    let mut dependency_inputs = collect_effect_dependency_inputs_batch(
        graph,
        &lowered
            .tasks
            .iter()
            .map(|task| task.node)
            .collect::<Vec<_>>(),
    )?
    .into_iter()
    .map(Some)
    .collect::<Vec<_>>();
    let mut lowered_tasks = lowered.tasks.into_iter().map(Some).collect::<Vec<_>>();

    #[cfg(feature = "parallel")]
    if executor.is_full_parallel() {
        stage_record.apply_mode = Some(crate::logic::planner::ParallelApplyMode::SerialFallback);
        stage_record.apply_group_count = lowered.apply_groups.len() as u32;
    }

    for group in &lowered.apply_groups {
        #[cfg(feature = "parallel")]
        if executor.is_full_parallel() {
            stage_record.serial_fallback_group_count += 1;
            stage_record.serial_apply_task_count += group.task_indices.len() as u32;
        }
        for &task_index in &group.task_indices {
            let lowered_task = lowered_tasks[task_index]
                .take()
                .expect("lowered apply task should be consumed exactly once");
            semantic_batch.push_segment(segment_for_single_update(apply_lowered_task(
                graph,
                summary,
                lowered.stage_index,
                lowered_task,
                dependency_updates[task_index],
                dependency_inputs[task_index]
                    .take()
                    .expect("dependency inputs should align with lowered tasks"),
                comparator_resolver,
                executor,
                stage_identities,
                &mut pending_snapshots,
            )?));
        }
    }
    Ok(StageScratch {
        semantic_batch: SingleConsumer::new(semantic_batch),
        pending_snapshots,
    })
}

fn apply_lowered_task(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    task: LoweredTask,
    dependency_updates: u32,
    dependency_inputs: crate::logic::evaluation::EffectDependencyInputs,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    pending_snapshots: &mut Vec<crate::logic::evaluation::PendingDependencySnapshot>,
) -> Result<SemanticTaskUpdate, SignalError> {
    let identity = stage_identities[task.task_index];
    let LoweredTask {
        task_index,
        node,
        execution,
        ..
    } = task;
    let LoweredTaskExecution {
        prepared,
        before_state,
        before_artifact_state,
        dependency_updates: _,
        recomputed,
        partition_aware,
        rewiring,
    } = execution;
    let apply_result = apply_prepared_evaluation_after_dependencies_with_policy(
        graph,
        node,
        prepared,
        comparator_resolver,
        None,
        dependency_updates,
        Some(dependency_inputs),
        true,
    )
    .map_err(|err| {
        record_execution_failure(
            graph,
            ExecutionFailureContext::new(
                ExecutionFailurePhase::Apply,
                Some(stage_index),
                Some(node),
                Some(executor),
                Some(identity.record_id),
                Some(*summary),
                err.to_string(),
            ),
        );
        err
    })?;
    if let Some(snapshot) = apply_result.pending_snapshot {
        pending_snapshots.push(snapshot);
    }
    let after_state = graph.get_state(node)?;
    let memoized_origin = graph
        .get_entry(node)?
        .get_runtime_artifact_state()
        .map(|trace| trace.memoized_origin)
        .unwrap_or(crate::data::output::MemoizedResultOrigin::DirectCompute);
    let reuse_basis = graph
        .get_entry(node)?
        .get_runtime_artifact_state()
        .map(|trace| trace.reuse_basis)
        .unwrap_or(crate::data::reuse::ReuseBasis::FreshCompute);
    Ok(SemanticTaskUpdate {
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        after_state,
        dependency_updates: apply_result.dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
        verdict: apply_result.report.verdict,
        memoized_origin,
        reuse_basis,
    })
}

fn build_lowered_apply_groups(
    tasks: &[LoweredTask],
    _executor: StageExecutor,
) -> Vec<DisjointApplyGroup> {
    #[cfg(feature = "parallel")]
    if _executor.is_full_parallel() {
        if let Some(policy) = _executor.parallel_policy() {
            return build_stage_apply_groups(tasks, policy);
        }
    }

    tasks
        .iter()
        .enumerate()
        .map(|(task_index, task)| DisjointApplyGroup {
            task_indices: vec![task_index],
            footprint: task.footprint.clone(),
        })
        .collect()
}

fn lower_task_patch(
    graph: &mut SignalGraph,
    patch: PreparedTaskPatch,
) -> Result<LoweredTask, SignalError> {
    let current_dependencies =
        CanonicalDependencies::from_slice(graph.runtime_dependencies_of(patch.node)?);
    let next_dependencies = CanonicalDependencies::new(build_prepared_dependency_edges(
        graph,
        &patch.prepared.dependencies,
    )?);
    let before_entry = graph.get_entry(patch.node)?.clone();
    let contract = graph.get_contract(patch.node)?.clone();
    let recomputed = matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(
            patch.prepared.origin,
            PreparedEvaluationOrigin::MemoizedReuse
        );
    let partition_aware = !patch.prepared.result.changed_regions.is_empty();
    let rewiring = rewiring_summary_from_lowered_edges(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );
    let footprint = build_apply_footprint(patch.node, &current_dependencies, &next_dependencies);
    let path_class = contract.execution.path_class;
    let authority_policy = contract.authority.policy;
    let dependency_updates =
        count_dependency_updates(current_dependencies.as_slice(), next_dependencies.as_slice());

    Ok(LoweredTask {
        task_index: patch.task_index,
        node: patch.node,
        contract,
        dependency_inputs: next_dependencies,
        path_class,
        authority_policy,
        footprint,
        execution: LoweredTaskExecution {
            prepared: patch.prepared,
            before_state: *before_entry.get_state(),
            before_artifact_state: before_entry.get_runtime_artifact_state().cloned(),
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        },
    })
}

fn build_apply_footprint(
    node: crate::data::handle::NodeId,
    current_dependencies: &CanonicalDependencies,
    next_dependencies: &CanonicalDependencies,
) -> ApplyFootprint {
    let mut touched_nodes = vec![node];
    touched_nodes.extend(
        current_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    touched_nodes.extend(
        next_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    let mut touched_sources = current_dependencies
        .as_slice()
        .iter()
        .map(|edge| edge.source())
        .collect::<Vec<_>>();
    touched_sources.extend(
        next_dependencies
            .as_slice()
            .iter()
            .map(|edge| edge.source()),
    );
    let partitions = PartitionScopeSet::new(
        current_dependencies
            .as_slice()
            .iter()
            .chain(next_dependencies.as_slice().iter())
            .filter_map(|edge| edge.scope_ref().cloned()),
    );
    ApplyFootprint {
        partitions,
        touched_nodes: DedupedNodeBatch::new(touched_nodes),
        touched_sources: SortedSourceBatch::new(touched_sources),
    }
}

fn build_lowered_dirty_delta(tasks: &[LoweredTask]) -> DirtyDelta {
    let mut changed_aspects = AspectMask::EMPTY;
    let mut changed_regions = Vec::new();
    let mut touched_nodes = Vec::new();

    for task in tasks {
        changed_aspects = changed_aspects | task.contract.semantics.produces;
        changed_regions.extend_from_slice(&task.execution.prepared.result.changed_regions);
        touched_nodes.push(task.node);
    }

    DirtyDelta::new(
        changed_aspects,
        CanonicalChangedRegions::new(changed_regions),
        DedupedNodeBatch::new(touched_nodes),
    )
}

fn build_touched_scope_summary(tasks: &[LoweredTask]) -> TouchedScopeSummary {
    let mut scopes = Vec::new();
    let mut touched_nodes = Vec::new();
    let mut touched_sources = Vec::new();

    for task in tasks {
        touched_nodes.push(task.node);
        touched_sources.extend_from_slice(task.footprint.touched_sources.as_slice());
        scopes.extend(
            task.dependency_inputs
                .as_slice()
                .iter()
                .filter_map(|edge| edge.scope_ref().cloned()),
        );
    }

    TouchedScopeSummary::new(scopes, touched_nodes, touched_sources)
}

fn build_prepared_dependency_edges(
    graph: &mut SignalGraph,
    capture: &PreparedDependencyCapture,
) -> Result<Vec<DependencyEdge>, SignalError> {
    let mut edges = Vec::with_capacity(capture.as_slice().len());
    for dependency in capture.as_slice() {
        let edge = graph.build_dependency_edge(
            dependency.source,
            dependency.aspect,
            dependency.scope.clone(),
        );
        if !edges.contains(&edge) {
            edges.push(edge);
        }
    }
    Ok(edges)
}

fn count_dependency_updates(
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
) -> u32 {
    let mut current_index = 0usize;
    let mut next_index = 0usize;
    let mut changes = 0u32;

    while current_index < current_dependencies.len() && next_index < next_dependencies.len() {
        match compare_dependency_edges(
            &current_dependencies[current_index],
            &next_dependencies[next_index],
        ) {
            std::cmp::Ordering::Less => {
                changes += 1;
                current_index += 1;
            }
            std::cmp::Ordering::Greater => {
                changes += 1;
                next_index += 1;
            }
            std::cmp::Ordering::Equal => {
                current_index += 1;
                next_index += 1;
            }
        }
    }

    changes
        + (current_dependencies.len() - current_index) as u32
        + (next_dependencies.len() - next_index) as u32
}

fn compare_dependency_edges(left: &DependencyEdge, right: &DependencyEdge) -> std::cmp::Ordering {
    (
        left.source().index(),
        left.source().generation(),
        left.aspect().index(),
        left.scope_ref(),
    )
        .cmp(&(
            right.source().index(),
            right.source().generation(),
            right.aspect().index(),
            right.scope_ref(),
        ))
}

fn rewiring_summary_from_lowered_edges(
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
) -> Option<RewiringSummary> {
    let mut added = next_dependencies
        .iter()
        .filter(|candidate| !current_dependencies.contains(candidate))
        .map(|edge| RewiringDependency {
            source: edge.source(),
            aspect: edge.aspect(),
            subscription: edge.scope_ref().cloned(),
        })
        .collect::<Vec<_>>();
    let mut removed = current_dependencies
        .iter()
        .filter(|candidate| !next_dependencies.contains(candidate))
        .map(|edge| RewiringDependency {
            source: edge.source(),
            aspect: edge.aspect(),
            subscription: edge.scope_ref().cloned(),
        })
        .collect::<Vec<_>>();

    if added.is_empty() && removed.is_empty() {
        None
    } else {
        added.sort_by_key(|dependency| {
            (
                dependency.source.index(),
                dependency.source.generation(),
                dependency.aspect.index(),
            )
        });
        removed.sort_by_key(|dependency| {
            (
                dependency.source.index(),
                dependency.source.generation(),
                dependency.aspect.index(),
            )
        });
        Some(RewiringSummary { added, removed })
    }
}
