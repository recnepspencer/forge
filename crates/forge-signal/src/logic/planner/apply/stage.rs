use std::time::Instant;

use crate::data::aspect::AspectMask;
use crate::data::comparator::ComparatorPolicyResolver;
#[cfg(feature = "parallel")]
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::CanonicalChangedRegions;
use crate::data::performance::{
    AuthorityPolicy, PathClass, ResolvedExecutionStrategy, ResolvedMaintenanceStrategy,
};
use crate::data::proof::{
    DedupedNodeBatch, DirtyDelta, PartitionScopeSet, SnapshotBatchCommit, SortedSourceBatch,
    StructuralDelta, TouchedScopeSummary,
};
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::SignalRuntimePolicy;
use crate::logic::evaluation::{
    apply_prepared_evaluation_after_dependencies_with_policy, collect_effect_dependency_inputs_iter,
};
#[cfg(feature = "parallel")]
use crate::logic::evaluation::{
    build_prepared_apply_commit_packet, record_reuse_rejection_telemetry, ApplyCommitBuildError,
};
use crate::logic::explain::{RewiringDependency, RewiringSummary};
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluationOrigin, PreparedEvaluationOutcome,
};

use super::super::execution::task_reporting::record_execution_failure;
use super::super::execution::StageSlice;
use super::super::precompute::{PreparedTaskPatch, StageExecutionData};
#[cfg(feature = "parallel")]
use super::super::precompute::executor_pool::PlannerExecutorPool;
use super::super::semantic::{
    finalize_stage_batch, segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch,
    StageSemanticIdentity,
};
use super::super::stage_precompute::StagePrecomputeResult;
use super::super::types::{
    ApplyFootprint, ConcurrentApplyPlan, DisjointApplyGroup, EligibleTask, ExecutionReport,
    LoweredApplyPlan, LoweredStagePlan, LoweredTask, LoweredTaskExecution, PlanSummary,
    ReductionOrderingContract, ReductionWorkClass, SerialApplyPlan, StageExecutionRecord,
    StageExecutor,
};
#[cfg(feature = "parallel")]
use super::super::types::{
    ApplyPlanSerialFallbackReason, ConcurrentApplyReductionPlan, DisjointApplyProof,
    MutationDomain, ParallelAdmissionReason, ParallelExecutionKind, SharedSurfacePolicy,
};
use super::workspace::{
    reduce_group_local_apply_packets, GroupLocalApplyPacket, StageScratch,
};
#[cfg(feature = "parallel")]
use super::workspace::{
    ConcurrentApplyGroupInput, ConcurrentWorkerInput, GroupLocalTaskCommit, GroupedApplyFailure,
};
#[cfg(feature = "parallel")]
use super::groups::build_stage_apply_groups;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

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
        comparator_resolver,
        executor,
    )?;
    if let Some(dirty) = lowered.dirty_delta.dirty.as_ref() {
        graph.telemetry_mut().invalidation.dirty_delta_breadth +=
            dirty.touched_nodes.len() as u64;
        graph.telemetry_mut().storage.structural_delta_size +=
            dirty.changed_regions.as_slice().len() as u64 + dirty.touched_nodes.len() as u64;
    }
    graph.telemetry_mut().execution.apply_group_width_total += lowered
        .apply_groups()
        .iter()
        .map(|group| group.task_indices.len() as u64)
        .sum::<u64>();
    graph.telemetry_mut().execution.max_apply_group_width = graph
        .telemetry()
        .execution
        .max_apply_group_width
        .max(
            lowered
                .apply_groups()
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .max()
                .unwrap_or(0),
        );
    graph.telemetry_mut().execution.apply_group_disjoint_count +=
        lowered
            .apply_groups()
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
    comparator_resolver: &impl ComparatorPolicyResolver,
    executor: StageExecutor,
) -> Result<LoweredStagePlan, SignalError> {
    let lowered_tasks = stage_execution
        .into_patches(stage_tasks)
        .into_iter()
        .map(|patch| lower_task_patch(graph, patch, comparator_resolver))
        .collect::<Result<Vec<_>, SignalError>>()?;
    let resolved_policy =
        SignalRuntimePolicy::for_tier(graph.diagnostics_profile()).resolve_performance_policy();
    let lowered_apply_plan = build_lowered_apply_plan(stage_index, &lowered_tasks, executor);
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
        lowered_apply_plan,
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
            || !lowered.apply_groups().is_empty(),
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
    match lowered.lowered_apply_plan {
        LoweredApplyPlan::Serial(plan) => run_serial_lowered_apply_pass(
            graph,
            summary,
            lowered.stage_index,
            lowered.tasks,
            plan,
            comparator_resolver,
            executor,
            stage_identities,
            stage_record,
        ),
        LoweredApplyPlan::GroupedConcurrent(plan) => run_grouped_concurrent_apply_pass(
            graph,
            summary,
            lowered.stage_index,
            lowered.tasks,
            plan,
            comparator_resolver,
            stage_identities,
            stage_record,
        ),
    }
}

fn run_serial_lowered_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    tasks: Vec<LoweredTask>,
    plan: SerialApplyPlan,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    #[cfg(feature = "parallel")]
    {
        stage_record.apply_mode = Some(crate::logic::planner::ParallelApplyMode::SerialApply);
        stage_record.apply_group_count = plan.groups.len() as u32;
        stage_record.serial_apply_rejection_reason = plan.rejection_reason;
        stage_record.serial_fallback_group_count = u32::from(plan.rejection_reason.is_some());
    }
    #[cfg(not(feature = "parallel"))]
    let _ = stage_record;

    let mut reconcile_batch = Vec::with_capacity(tasks.len());
    for task in &tasks {
        reconcile_batch.push((task.node, task.dependency_inputs.as_slice()));
    }
    graph.reconcile_dependencies_batch_borrowed(&reconcile_batch)?;
    let mut dependency_inputs = collect_effect_dependency_inputs_iter(
        graph,
        tasks.iter().map(|task| task.node),
    )?
        .into_iter()
        .map(Some)
        .collect::<Vec<_>>();
    let mut lowered_tasks = tasks.into_iter().map(Some).collect::<Vec<_>>();
    let mut group_local_packets = Vec::with_capacity(plan.groups.len());

    for (group_index, group) in plan.groups.iter().enumerate() {
        let mut group_semantic_batch = StageSemanticBatch::default();
        let mut group_pending_snapshots = Vec::with_capacity(group.task_indices.len());
        #[cfg(feature = "parallel")]
        {
            stage_record.serial_apply_task_count += group.task_indices.len() as u32;
        }
        for &task_index in &group.task_indices {
            let lowered_task = lowered_tasks[task_index]
                .take()
                .expect("lowered apply task should be consumed exactly once");
            group_semantic_batch.push_segment(segment_for_single_update(apply_lowered_task(
                graph,
                summary,
                stage_index,
                lowered_task,
                dependency_inputs[task_index]
                    .take()
                    .expect("dependency inputs should align with lowered tasks"),
                comparator_resolver,
                executor,
                stage_identities,
                &mut group_pending_snapshots,
            )?));
        }
        group_local_packets.push(GroupLocalApplyPacket {
            group_index,
            task_count: group.task_indices.len(),
            task_commits: Vec::new(),
            semantic_batch: group_semantic_batch,
            pending_snapshots: group_pending_snapshots,
        });
    }

    graph.telemetry_mut().execution.group_local_packet_breadth += group_local_packets
        .iter()
        .map(|packet| packet.packet_breadth() as u64)
        .sum::<u64>();
    graph.telemetry_mut().execution.reduction_packet_breadth += group_local_packets.len() as u64;
    graph.telemetry_mut().execution.reduction_group_count += group_local_packets.len() as u64;
    graph.telemetry_mut().execution.shared_surface_publication_breadth += group_local_packets
        .iter()
        .map(|packet| packet.publication_breadth() as u64)
        .sum::<u64>();

    Ok(reduce_group_local_apply_packets(
        group_local_packets,
        ReductionOrderingContract::StageTaskIndexOrder,
        ReductionWorkClass::DeterministicPublicationOnly,
    ))
}

#[cfg(feature = "parallel")]
fn run_grouped_concurrent_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    tasks: Vec<LoweredTask>,
    plan: ConcurrentApplyPlan,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    stage_record.apply_mode =
        Some(crate::logic::planner::ParallelApplyMode::GroupedConcurrentApply);
    stage_record.outcome = crate::logic::planner::StageExecutionOutcome::CompletedParallel;
    stage_record.parallel_kind = Some(ParallelExecutionKind::FullParallel);
    stage_record.parallel_admission_reason =
        Some(ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent);
    stage_record.apply_group_count = plan.groups.len() as u32;
    stage_record.serial_apply_rejection_reason = None;
    stage_record.concurrent_apply_task_count = plan
        .groups
        .iter()
        .map(|group| group.task_indices.len() as u32)
        .sum();
    graph.telemetry_mut().execution.parallel_stage_dispatch_count += 1;

    let dependency_inputs =
        collect_effect_dependency_inputs_iter(graph, tasks.iter().map(|task| task.node))?;
    let task_count = tasks.len();
    let group_inputs = build_concurrent_apply_group_inputs(
        tasks,
        dependency_inputs,
        &plan.groups,
        stage_identities,
    );
    let worker_count = task_count.max(1).min(plan.groups.len().max(1));
    let pool = PlannerExecutorPool::shared(worker_count)?;
    let graph_ref = &*graph;
    let group_packets = pool.install(|| {
        group_inputs
            .into_par_iter()
            .map(|group| {
                let mut task_commits = Vec::with_capacity(group.worker_inputs.len());
                for worker_input in group.worker_inputs {
                    let commit_packet = build_prepared_apply_commit_packet(
                        graph_ref,
                        worker_input.node,
                        worker_input.prepared,
                        worker_input.comparator_policy,
                        None,
                        worker_input.dependency_updates,
                        worker_input.dependency_inputs,
                        true,
                    )
                    .map_err(|error| {
                        grouped_apply_failure_from_build_error(
                            worker_input.node,
                            worker_input.identity.record_id,
                            error,
                        )
                    })?;
                    task_commits.push(GroupLocalTaskCommit {
                        task_index: worker_input.task_index,
                        node: worker_input.node,
                        identity: worker_input.identity,
                        before_state: worker_input.before_state,
                        before_artifact_state: worker_input.before_artifact_state,
                        dependency_updates: worker_input.dependency_updates,
                        recomputed: worker_input.recomputed,
                        partition_aware: worker_input.partition_aware,
                        rewiring: worker_input.rewiring,
                        commit_packet: commit_packet.try_into().map_err(|error| {
                            GroupedApplyFailure {
                                node: worker_input.node,
                                record_id: worker_input.identity.record_id,
                                error,
                                reuse_failure: None,
                            }
                        })?,
                    });
                }
                Ok::<GroupLocalApplyPacket, GroupedApplyFailure>(GroupLocalApplyPacket {
                    group_index: group.group_index,
                    task_count: task_commits.len(),
                    task_commits,
                    semantic_batch: StageSemanticBatch::default(),
                    pending_snapshots: Vec::new(),
                })
            })
            .collect::<Result<Vec<_>, GroupedApplyFailure>>()
    });
    let group_packets = match group_packets {
        Ok(packets) => packets,
        Err(failure) => {
            record_grouped_apply_failure(graph, summary, stage_index, &failure);
            return Err(failure.error);
        }
    };

    graph.telemetry_mut().execution.group_local_packet_breadth += group_packets
        .iter()
        .map(|packet| packet.packet_breadth() as u64)
        .sum::<u64>();
    graph.telemetry_mut().execution.reduction_packet_breadth += group_packets.len() as u64;
    graph.telemetry_mut().execution.reduction_group_count += group_packets.len() as u64;

    reduce_grouped_concurrent_packets(graph, summary, stage_index, group_packets, plan.reduction)
}

#[cfg(not(feature = "parallel"))]
fn run_grouped_concurrent_apply_pass(
    _graph: &mut SignalGraph,
    _summary: &PlanSummary,
    _stage_index: u32,
    _tasks: Vec<LoweredTask>,
    _plan: ConcurrentApplyPlan,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
    _stage_identities: &[StageSemanticIdentity],
    _stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    Err(SignalError::internal(
        "grouped concurrent apply requires the `parallel` feature",
    ))
}

#[cfg(feature = "parallel")]
fn reduce_grouped_concurrent_packets(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    mut packets: Vec<GroupLocalApplyPacket>,
    reduction: ConcurrentApplyReductionPlan,
) -> Result<StageScratch, SignalError> {
    debug_assert!(
        matches!(reduction.allowed_work, ReductionWorkClass::DeterministicPublicationOnly),
        "grouped concurrent reduction may only perform deterministic publication"
    );
    match reduction.ordering_contract {
        ReductionOrderingContract::StageTaskIndexOrder => {
            packets.sort_by_key(|packet| packet.group_index);
        }
    }

    let mut semantic_batch = StageSemanticBatch::default();
    let mut pending_snapshots = Vec::new();
    for packet in packets {
        for commit in packet.task_commits {
            let update =
                match publish_group_local_task_commit(graph, commit, &mut pending_snapshots) {
                    Ok(update) => update,
                    Err(failure) => {
                        record_grouped_apply_failure(graph, summary, stage_index, &failure);
                        return Err(failure.error);
                    }
                };
            semantic_batch.push_segment(segment_for_single_update(update));
        }
    }
    graph.telemetry_mut().execution.shared_surface_publication_breadth +=
        semantic_batch.segment_count() as u64 + pending_snapshots.len() as u64;
    Ok(StageScratch {
        semantic_batch: crate::data::proof::SingleConsumer::new(semantic_batch),
        pending_snapshots,
    })
}

#[cfg(feature = "parallel")]
fn publish_group_local_task_commit(
    graph: &mut SignalGraph,
    commit: GroupLocalTaskCommit,
    pending_snapshots: &mut Vec<crate::logic::evaluation::PendingDependencySnapshot>,
) -> Result<SemanticTaskUpdate, GroupedApplyFailure> {
    let GroupLocalTaskCommit {
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
        commit_packet,
    } = commit;
    let (report, pending_snapshot) = graph
        .publish_suppression_free_apply_commit_packet(commit_packet)
        .map_err(|error| GroupedApplyFailure {
            node,
            record_id: identity.record_id,
            error,
            reuse_failure: None,
        })?;
    if let Some(snapshot) = pending_snapshot {
        pending_snapshots.push(snapshot);
    }
    let after_state = graph.get_state(node).map_err(|error| GroupedApplyFailure {
        node,
        record_id: identity.record_id,
        error,
        reuse_failure: None,
    })?;
    let memoized_origin = graph
        .get_entry(node)
        .map_err(|error| GroupedApplyFailure {
            node,
            record_id: identity.record_id,
            error,
            reuse_failure: None,
        })?
        .get_runtime_artifact_state()
        .map(|trace| trace.memoized_origin)
        .unwrap_or(crate::data::output::MemoizedResultOrigin::DirectCompute);
    let reuse_basis = graph
        .get_entry(node)
        .map_err(|error| GroupedApplyFailure {
            node,
            record_id: identity.record_id,
            error,
            reuse_failure: None,
        })?
        .get_runtime_artifact_state()
        .map(|trace| trace.reuse_basis.clone())
        .unwrap_or(crate::data::reuse::ReuseBasis::fresh_compute());
    Ok(SemanticTaskUpdate {
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        after_state,
        dependency_updates,
        recomputed,
        partition_aware,
        rewiring,
        verdict: report.verdict,
        memoized_origin,
        reuse_basis,
    })
}

#[cfg(feature = "parallel")]
fn grouped_apply_failure_from_build_error(
    node: crate::data::handle::NodeId,
    record_id: crate::logic::planner::ExecutionRecordId,
    error: ApplyCommitBuildError,
) -> GroupedApplyFailure {
    GroupedApplyFailure {
        node,
        record_id,
        reuse_failure: error.reuse_failure(),
        error: error.into_signal(),
    }
}

#[cfg(feature = "parallel")]
fn record_grouped_apply_failure(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    failure: &GroupedApplyFailure,
) {
    if let Some(reuse_failure) = failure.reuse_failure {
        record_reuse_rejection_telemetry(graph, &reuse_failure);
    }
    record_execution_failure(
        graph,
        ExecutionFailureContext::new(
            ExecutionFailurePhase::Apply,
            Some(stage_index),
            Some(failure.node),
            Some(StageExecutor::full_parallel(1)),
            Some(failure.record_id),
            Some(*summary),
            failure.error.to_string(),
        ),
    );
}

fn apply_lowered_task(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    task: LoweredTask,
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
        dependency_updates,
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
        .map(|trace| trace.reuse_basis.clone())
        .unwrap_or(crate::data::reuse::ReuseBasis::fresh_compute());
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

fn build_lowered_apply_plan(
    stage_index: u32,
    tasks: &[LoweredTask],
    _executor: StageExecutor,
) -> LoweredApplyPlan {
    #[cfg(not(feature = "parallel"))]
    let _ = stage_index;

    let serial_groups = || {
        tasks
            .iter()
            .enumerate()
            .map(|(task_index, task)| DisjointApplyGroup {
                task_indices: vec![task_index],
                footprint: task.footprint.clone(),
            })
            .collect::<Vec<_>>()
    };

    #[cfg(feature = "parallel")]
    if _executor.is_full_parallel() {
        if let Some(policy) = _executor.parallel_policy() {
            let groups = build_stage_apply_groups(tasks, policy);
            if can_lower_true_grouped_concurrent(tasks, &groups) {
                let group_footprints = groups
                    .iter()
                    .map(|group| group.footprint.clone())
                    .collect();
                return LoweredApplyPlan::GroupedConcurrent(ConcurrentApplyPlan {
                    groups,
                    proof: DisjointApplyProof {
                        stage_index,
                        mutation_domain: MutationDomain::LoweredStage,
                        group_footprints,
                        shared_surface_policy: SharedSurfacePolicy::ReductionOnly,
                    },
                    reduction: ConcurrentApplyReductionPlan {
                        ordering_contract: ReductionOrderingContract::StageTaskIndexOrder,
                        allowed_work: ReductionWorkClass::DeterministicPublicationOnly,
                    },
                });
            }
            return LoweredApplyPlan::Serial(SerialApplyPlan {
                groups,
                rejection_reason: Some(
                    ApplyPlanSerialFallbackReason::FullParallelUnsupportedByMutableEngine,
                ),
            });
        }
    }

    #[cfg(feature = "parallel")]
    if _executor.parallel_policy().is_some() {
        return LoweredApplyPlan::Serial(SerialApplyPlan {
            groups: serial_groups(),
            rejection_reason: None,
        });
    }

    LoweredApplyPlan::Serial(SerialApplyPlan {
        groups: serial_groups(),
        rejection_reason: None,
    })
}

#[cfg(feature = "parallel")]
fn build_concurrent_apply_group_inputs(
    tasks: Vec<LoweredTask>,
    dependency_inputs: Vec<crate::logic::evaluation::EffectDependencyInputs>,
    groups: &[DisjointApplyGroup],
    stage_identities: &[StageSemanticIdentity],
) -> Vec<ConcurrentApplyGroupInput> {
    let mut task_slots = tasks.into_iter().map(Some).collect::<Vec<_>>();
    let mut dependency_input_slots = dependency_inputs.into_iter().map(Some).collect::<Vec<_>>();
    let mut group_inputs = Vec::with_capacity(groups.len());

    for (group_index, group) in groups.iter().enumerate() {
        let mut worker_inputs = Vec::with_capacity(group.task_indices.len());
        for &task_index in &group.task_indices {
            let lowered_task = task_slots[task_index]
                .take()
                .expect("grouped concurrent task should be consumed exactly once");
            let dependency_input = dependency_input_slots[task_index]
                .take()
                .expect("grouped concurrent dependency inputs should align with tasks");
            worker_inputs.push(lowered_task.into_concurrent_worker_input(
                stage_identities[task_index],
                dependency_input,
            ));
        }
        group_inputs.push(ConcurrentApplyGroupInput {
            group_index,
            worker_inputs,
        });
    }

    group_inputs
}

#[cfg(feature = "parallel")]
fn can_lower_true_grouped_concurrent(
    tasks: &[LoweredTask],
    groups: &[DisjointApplyGroup],
) -> bool {
    !groups.is_empty()
        && tasks.iter().all(|task| {
            task.execution.dependency_updates == 0
                && task.execution.rewiring.is_none()
                && !matches!(task.comparator_policy, VersionComparatorPolicy::OutputIdentity)
        })
}

fn lower_task_patch(
    graph: &mut SignalGraph,
    patch: PreparedTaskPatch,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<LoweredTask, SignalError> {
    #[cfg(not(feature = "parallel"))]
    let _ = comparator_resolver;
    graph.refresh_runtime_dependencies_of(patch.node)?;
    let current_dependencies =
        CanonicalDependencies::from_slice(graph.current_runtime_dependencies_of(patch.node)?);
    let next_dependencies = CanonicalDependencies::new(build_prepared_dependency_edges(
        graph,
        &patch.prepared.dependencies,
    )?);
    let before_entry = graph.get_entry(patch.node)?;
    let before_state = *before_entry.get_state();
    let before_artifact_state = before_entry.get_runtime_artifact_state().cloned();
    let contract = graph.get_contract(patch.node)?;
    #[cfg(feature = "parallel")]
    let comparator_policy = comparator_resolver.policy_for_node(
        patch.node,
        graph.get_entry(patch.node)?.get_eval_config().comparator.as_ref(),
    );
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
    let produced_aspects = contract.semantics.produces;
    let path_class = contract.execution.path_class;
    let authority_policy = contract.authority.policy;
    let dependency_updates =
        count_dependency_updates(current_dependencies.as_slice(), next_dependencies.as_slice());

    Ok(LoweredTask {
        task_index: patch.task_index,
        node: patch.node,
        produced_aspects,
        dependency_inputs: next_dependencies,
        #[cfg(feature = "parallel")]
        comparator_policy,
        path_class,
        authority_policy,
        footprint,
        execution: LoweredTaskExecution {
            prepared: patch.prepared,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        },
    })
}

impl LoweredTask {
    #[cfg(feature = "parallel")]
    fn into_concurrent_worker_input(
        self,
        identity: StageSemanticIdentity,
        dependency_inputs: crate::logic::evaluation::EffectDependencyInputs,
    ) -> ConcurrentWorkerInput {
        let LoweredTask {
            task_index,
            node,
            #[cfg(feature = "parallel")]
            comparator_policy,
            execution,
            ..
        } = self;
        let LoweredTaskExecution {
            prepared,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        } = execution;
        ConcurrentWorkerInput {
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
            comparator_policy,
            prepared,
            dependency_inputs,
        }
    }
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
        changed_aspects = changed_aspects | task.produced_aspects;
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
    Ok(capture
        .as_slice()
        .iter()
        .map(|dependency| {
            graph.build_dependency_edge(
                dependency.source,
                dependency.aspect,
                dependency.scope.clone(),
            )
        })
        .collect())
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
    let mut current_index = 0usize;
    let mut next_index = 0usize;
    let mut added = Vec::new();
    let mut removed = Vec::new();

    while current_index < current_dependencies.len() && next_index < next_dependencies.len() {
        match compare_dependency_edges(
            &current_dependencies[current_index],
            &next_dependencies[next_index],
        ) {
            std::cmp::Ordering::Less => {
                let edge = &current_dependencies[current_index];
                removed.push(RewiringDependency {
                    source: edge.source(),
                    aspect: edge.aspect(),
                    subscription: edge.scope_ref().cloned(),
                });
                current_index += 1;
            }
            std::cmp::Ordering::Greater => {
                let edge = &next_dependencies[next_index];
                added.push(RewiringDependency {
                    source: edge.source(),
                    aspect: edge.aspect(),
                    subscription: edge.scope_ref().cloned(),
                });
                next_index += 1;
            }
            std::cmp::Ordering::Equal => {
                current_index += 1;
                next_index += 1;
            }
        }
    }

    while current_index < current_dependencies.len() {
        let edge = &current_dependencies[current_index];
        removed.push(RewiringDependency {
            source: edge.source(),
            aspect: edge.aspect(),
            subscription: edge.scope_ref().cloned(),
        });
        current_index += 1;
    }

    while next_index < next_dependencies.len() {
        let edge = &next_dependencies[next_index];
        added.push(RewiringDependency {
            source: edge.source(),
            aspect: edge.aspect(),
            subscription: edge.scope_ref().cloned(),
        });
        next_index += 1;
    }

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

