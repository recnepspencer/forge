use std::time::Instant;

use crate::data::aspect::AspectMask;
use crate::data::comparator::ComparatorPolicyResolver;
#[cfg(feature = "parallel")]
use crate::data::comparator::VersionComparatorPolicy;
use crate::data::dependency::CanonicalDependencies;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::output::CanonicalChangedRegions;
use crate::data::performance::{
    AuthorityPolicy, PathClass, ResolvedExecutionStrategy, ResolvedMaintenanceStrategy,
};
use crate::data::proof::{
    DedupedNodeBatch, DirtyDelta, PartitionScopeSet, SortedSourceBatch, StructuralDelta,
    TouchedScopeSummary,
};
#[cfg(feature = "parallel")]
use crate::diagnostics::failure::{ExecutionFailureContext, ExecutionFailurePhase};
use crate::diagnostics::SignalRuntimePolicy;
#[cfg(feature = "parallel")]
use crate::logic::evaluation::collect_effect_dependency_inputs_iter;
#[cfg(feature = "parallel")]
use crate::logic::evaluation::{
    build_prepared_apply_commit_packet, record_reuse_rejection_telemetry, ApplyCommitBuildError,
};
use crate::logic::prepared::{PreparedEvaluationOrigin, PreparedEvaluationOutcome};

#[cfg(feature = "parallel")]
use super::super::execution::task_reporting::record_execution_failure;
use super::super::execution::StageSlice;
#[cfg(feature = "parallel")]
use super::super::precompute::executor_pool::PlannerExecutorPool;
use super::super::precompute::{PreparedTaskPatch, StageExecutionData};
#[cfg(feature = "parallel")]
use super::super::semantic::finalize_stage_batch;
use super::super::semantic::{finalize_serial_stage_batch, StageSemanticIdentity};
#[cfg(feature = "parallel")]
use super::super::semantic::{segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch};
use super::super::stage_precompute::StagePrecomputeResult;
use super::super::types::{
    ApplyFootprint, ConcurrentApplyPlan, DisjointApplyGroup, EligibleTask, ExecutionReport,
    LoweredApplyPlan, LoweredStagePlan, LoweredTask, LoweredTaskExecution, PlanSummary,
    SerialApplyPlan, StageExecutionRecord, StageExecutor,
};
#[cfg(feature = "parallel")]
use super::super::types::{
    ApplyPlanSerialFallbackReason, ConcurrentApplyReductionPlan, DisjointApplyProof,
    MutationDomain, ParallelAdmissionReason, ParallelExecutionKind, ReductionOrderingContract,
    ReductionWorkClass, SharedSurfacePolicy,
};
#[cfg(feature = "parallel")]
use super::groups::build_stage_apply_groups;
use super::lowering_support::{
    build_prepared_dependency_edges, count_dependency_updates, rewiring_summary_from_lowered_edges,
};
use super::serial_batch::{LoweredSerialStage, PreparedSerialStageBatch};
#[cfg(feature = "parallel")]
use super::workspace::{
    ConcurrentApplyGroupInput, ConcurrentWorkerInput, GroupLocalApplyPacket, GroupLocalTaskCommit,
    GroupedApplyFailure,
};
use super::workspace::{StageFinalizeWork, StageScratch};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
fn take_lowered_slot<T>(slot: &mut Option<T>, context: &'static str) -> Result<T, SignalError> {
    slot.take().ok_or_else(|| SignalError::internal(context))
}

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
    let lowered = build_stage_execution_form(
        graph,
        stage.index,
        stage.tasks,
        precomputed.execution,
        comparator_resolver,
        executor,
        stage_identities,
    )?;
    record_stage_lowering_metrics(graph, &lowered);
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
        graph.apply_snapshot_batch_commit(stage_scratch.pending_snapshots)?;
    }
    let semantic_finalize_start = Instant::now();
    match stage_scratch.finalize_work {
        StageFinalizeWork::Serial(batch) => {
            let ready = batch.into_ready_for_finalize()?;
            finalize_serial_stage_batch(graph, ready, report, stage_record)?
                .record_into(report, stage_record);
        }
        #[cfg(feature = "parallel")]
        StageFinalizeWork::Parallel(batch) => {
            finalize_stage_batch(graph, stage.tasks, batch.into_inner(), report, stage_record)?;
        }
    }
    stage_record.semantic_finalize_duration_nanos = semantic_finalize_start.elapsed().as_nanos();
    Ok(())
}

enum LoweredStageExecutionForm {
    Serial(LoweredSerialStage),
    Generic(LoweredStagePlan),
}

fn build_stage_execution_form(
    graph: &mut SignalGraph,
    stage_index: u32,
    stage_tasks: &[EligibleTask],
    stage_execution: StageExecutionData,
    comparator_resolver: &impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
) -> Result<LoweredStageExecutionForm, SignalError> {
    let prepared_patches = stage_execution.into_patches(stage_tasks);
    let resolved_policy =
        SignalRuntimePolicy::for_tier(graph.diagnostics_profile()).resolve_performance_policy();

    if should_lower_direct_serial(executor) {
        return Ok(LoweredStageExecutionForm::Serial(
            LoweredSerialStage::from_prepared_patches(
                graph,
                stage_index,
                stage_tasks,
                prepared_patches,
                resolved_policy.maintenance_strategy,
                resolved_policy.authority_policy,
                stage_identities,
            )?,
        ));
    }

    let lowered_tasks = prepared_patches
        .into_iter()
        .map(|patch| lower_task_patch(graph, patch, comparator_resolver))
        .collect::<Result<Vec<_>, SignalError>>()?;
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

    let lowered_stage = LoweredStagePlan {
        stage_index,
        tasks: lowered_tasks,
        lowered_apply_plan,
        dirty_delta: StructuralDelta::new(Some(dirty_delta), Some(touched_scope)),
        execution_strategy: resolved_policy.execution_strategy,
        maintenance_strategy: resolved_policy.maintenance_strategy,
        authority_policy,
    };

    if let LoweredApplyPlan::Serial(plan) = lowered_stage.lowered_apply_plan {
        let LoweredStagePlan {
            stage_index,
            tasks,
            dirty_delta,
            authority_policy,
            ..
        } = lowered_stage;
        #[cfg(not(feature = "parallel"))]
        let _ = plan;
        return Ok(LoweredStageExecutionForm::Serial(
            LoweredSerialStage::from_lowered_tasks(
                stage_index,
                stage_tasks,
                authority_policy,
                dirty_delta,
                resolved_policy.maintenance_strategy,
                #[cfg(feature = "parallel")]
                plan.rejection_reason,
                tasks,
                stage_identities,
            ),
        ));
    }

    Ok(LoweredStageExecutionForm::Generic(lowered_stage))
}

#[cfg(feature = "parallel")]
fn should_lower_direct_serial(executor: StageExecutor) -> bool {
    !executor.is_full_parallel()
}

#[cfg(not(feature = "parallel"))]
fn should_lower_direct_serial(_executor: StageExecutor) -> bool {
    true
}

fn record_stage_lowering_metrics(graph: &mut SignalGraph, lowered: &LoweredStageExecutionForm) {
    let (dirty_delta, maintenance_strategy) = match lowered {
        LoweredStageExecutionForm::Serial(stage) => {
            (stage.dirty_delta(), stage.maintenance_strategy())
        }
        LoweredStageExecutionForm::Generic(stage) => {
            (&stage.dirty_delta, stage.maintenance_strategy)
        }
    };
    if let Some(dirty) = dirty_delta.dirty.as_ref() {
        graph.telemetry_mut().invalidation.dirty_delta_breadth += dirty.touched_nodes.len() as u64;
        graph.telemetry_mut().storage.structural_delta_size +=
            dirty.changed_regions.as_slice().len() as u64 + dirty.touched_nodes.len() as u64;
    }
    match lowered {
        LoweredStageExecutionForm::Serial(stage) => {
            let batch_width = stage.stage_width() as u64;
            graph.telemetry_mut().execution.apply_group_width_total += batch_width;
            graph.telemetry_mut().execution.max_apply_group_width = graph
                .telemetry()
                .execution
                .max_apply_group_width
                .max(batch_width);
        }
        LoweredStageExecutionForm::Generic(stage) => {
            let apply_groups = stage.apply_groups();
            graph.telemetry_mut().execution.apply_group_width_total += apply_groups
                .iter()
                .map(|group| group.task_indices.len() as u64)
                .sum::<u64>();
            graph.telemetry_mut().execution.max_apply_group_width =
                graph.telemetry().execution.max_apply_group_width.max(
                    apply_groups
                        .iter()
                        .map(|group| group.task_indices.len() as u64)
                        .max()
                        .unwrap_or(0),
                );
            graph.telemetry_mut().execution.apply_group_disjoint_count += apply_groups
                .iter()
                .map(|group| group.task_indices.len().saturating_sub(1) as u64)
                .sum::<u64>();
        }
    }
    match maintenance_strategy {
        ResolvedMaintenanceStrategy::Incremental | ResolvedMaintenanceStrategy::DensityAdaptive => {
            graph.telemetry_mut().planner.incremental_strategy_count += 1;
        }
        ResolvedMaintenanceStrategy::Rebuild => {
            graph.telemetry_mut().planner.rebuild_strategy_count += 1;
        }
    }
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
    lowered: LoweredStageExecutionForm,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    match lowered {
        LoweredStageExecutionForm::Serial(lowered) => {
            stage_record.authority_policy = Some(lowered.authority_policy());
            run_serial_lowered_apply_pass(
                graph,
                summary,
                lowered,
                comparator_resolver,
                executor,
                stage_record,
            )
        }
        LoweredStageExecutionForm::Generic(lowered) => {
            validate_lowered_stage_plan(&lowered);
            stage_record.authority_policy = Some(lowered.authority_policy);
            let LoweredStagePlan {
                stage_index,
                tasks,
                lowered_apply_plan,
                ..
            } = lowered;
            let LoweredApplyPlan::GroupedConcurrent(plan) = lowered_apply_plan else {
                return Err(SignalError::internal(
                    "generic stage dispatch received a serial apply plan after serial lowering",
                ));
            };
            run_grouped_concurrent_apply_pass(
                graph,
                summary,
                stage_index,
                tasks,
                plan,
                comparator_resolver,
                stage_identities,
                stage_record,
            )
        }
    }
}

fn run_serial_lowered_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    lowered: super::serial_batch::LoweredSerialStage,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    #[cfg(not(feature = "parallel"))]
    let _ = stage_record;
    let prepared = PreparedSerialStageBatch::prepare(graph, lowered, stage_record)?;
    let applied = prepared.apply(graph, summary, comparator_resolver, executor)?;
    let (applied, pending_snapshots) = applied.split_pending_snapshots();

    Ok(StageScratch {
        finalize_work: StageFinalizeWork::Serial(applied),
        pending_snapshots,
    })
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
    // TODO(perf-m1-parallel): the grouped-concurrent lane still reduces through the
    // legacy semantic-batch/segment path. Milestone 1 intentionally keeps that
    // compatibility surface intact while the serial lane moves to the proof-typed
    // batch substrate. A later pass should converge this branch onto the same
    // batch-native finalize contract without reintroducing generic packet costs
    // into serial execution.
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
    graph
        .telemetry_mut()
        .execution
        .parallel_stage_dispatch_count += 1;

    let dependency_input_start = Instant::now();
    let dependency_inputs =
        collect_effect_dependency_inputs_iter(graph, tasks.iter().map(|task| task.node))?;
    graph.telemetry_mut().execution.dependency_input_build_nanos +=
        dependency_input_start.elapsed().as_nanos();
    let task_count = tasks.len();
    let group_inputs = build_concurrent_apply_group_inputs(
        tasks,
        dependency_inputs,
        &plan.groups,
        stage_identities,
    )?;
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
        matches!(
            reduction.allowed_work,
            ReductionWorkClass::DeterministicPublicationOnly
        ),
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
    graph
        .telemetry_mut()
        .execution
        .shared_surface_publication_breadth +=
        semantic_batch.segment_count() as u64 + pending_snapshots.len() as u64;
    Ok(StageScratch {
        finalize_work: StageFinalizeWork::Parallel(crate::data::proof::SingleConsumer::new(
            semantic_batch,
        )),
        pending_snapshots: SnapshotBatchCommit::from_unique_pending_snapshots_in_stage_order(
            pending_snapshots,
        ),
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
    let entry = graph.get_entry(node).map_err(|error| GroupedApplyFailure {
        node,
        record_id: identity.record_id,
        error,
        reuse_failure: None,
    })?;
    let after_state = *entry.get_state();
    let after_trace = entry.get_runtime_artifact_state();
    let memoized_origin = after_trace
        .map(|trace| trace.memoized_origin)
        .unwrap_or(crate::data::output::MemoizedResultOrigin::DirectCompute);
    let reuse_basis = after_trace
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
                let group_footprints = groups.iter().map(|group| group.footprint.clone()).collect();
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
) -> Result<Vec<ConcurrentApplyGroupInput>, SignalError> {
    let mut task_slots = tasks.into_iter().map(Some).collect::<Vec<_>>();
    let mut dependency_input_slots = dependency_inputs.into_iter().map(Some).collect::<Vec<_>>();
    let mut group_inputs = Vec::with_capacity(groups.len());

    for (group_index, group) in groups.iter().enumerate() {
        let mut worker_inputs = Vec::with_capacity(group.task_indices.len());
        for &task_index in &group.task_indices {
            let lowered_task = take_lowered_slot(
                &mut task_slots[task_index],
                "grouped concurrent lowered task slot was consumed more than once",
            )?;
            let dependency_input = take_lowered_slot(
                &mut dependency_input_slots[task_index],
                "grouped concurrent dependency inputs no longer align with lowered tasks",
            )?;
            worker_inputs.push(
                lowered_task
                    .into_concurrent_worker_input(stage_identities[task_index], dependency_input),
            );
        }
        group_inputs.push(ConcurrentApplyGroupInput {
            group_index,
            worker_inputs,
        });
    }

    Ok(group_inputs)
}

#[cfg(feature = "parallel")]
fn can_lower_true_grouped_concurrent(tasks: &[LoweredTask], groups: &[DisjointApplyGroup]) -> bool {
    !groups.is_empty()
        && tasks.iter().all(|task| {
            task.execution.dependency_updates == 0
                && task.execution.rewiring.is_none()
                && !matches!(
                    task.comparator_policy,
                    VersionComparatorPolicy::OutputIdentity
                )
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
        graph
            .get_entry(patch.node)?
            .get_eval_config()
            .comparator
            .as_ref(),
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
    let dependency_updates = count_dependency_updates(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );

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
