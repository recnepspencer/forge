#![cfg(feature = "parallel")]

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::MemoizedResultOrigin;
use crate::data::output_equivalence::OutputEquivalencePolicy;
use crate::data::proof::SnapshotBatchCommit;
use crate::data::reuse::ReuseBasis;
use crate::logic::evaluation::{
    build_prepared_apply_commit_packet, record_reuse_rejection_telemetry, ApplyCommitBuildError,
    EffectDependencyInputs,
};
use crate::logic::planner::semantic::StageSemanticIdentity;
use crate::logic::planner::semantic::{
    segment_for_single_update, SemanticTaskUpdate, StageSemanticBatch,
};
use crate::logic::planner::types::{
    ConcurrentApplyReductionPlan, DisjointApplyGroup, LoweredTask, PlanSummary,
    ReductionOrderingContract, ReductionWorkClass, StageExecutor,
};

use crate::logic::planner::apply::workspace::{
    ConcurrentApplyGroupInput, ConcurrentWorkerInput, GroupLocalApplyPacket, GroupLocalTaskCommit,
    GroupedApplyFailure, StageFinalizeWork, StageScratch,
};

#[cfg(feature = "parallel")]
pub(super) use crate::logic::planner::apply::groups::build_stage_apply_groups;

#[cfg(feature = "parallel")]
pub(super) fn build_group_packet(
    graph: &SignalGraph,
    group: ConcurrentApplyGroupInput,
) -> Result<GroupLocalApplyPacket, GroupedApplyFailure> {
    let (group_index, worker_inputs) = group.into_parts();
    let mut task_commits = Vec::with_capacity(worker_inputs.len());
    for worker_input in worker_inputs {
        let (
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
        ) = worker_input.into_parts();
        let commit_packet = build_prepared_apply_commit_packet(
            graph,
            node,
            prepared,
            comparator_policy,
            None,
            dependency_updates,
            dependency_inputs,
            false,
        )
        .map_err(|error| grouped_apply_failure_from_build_error(node, identity.record_id, error))?;
        task_commits.push(GroupLocalTaskCommit::new(
            task_index,
            node,
            identity,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
            commit_packet.into(),
        ));
    }
    Ok(GroupLocalApplyPacket::new(group_index, task_commits))
}

#[cfg(feature = "parallel")]
pub(super) fn reduce_grouped_concurrent_packets(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    mut packets: Vec<GroupLocalApplyPacket>,
    reduction: ConcurrentApplyReductionPlan,
    comparator_resolver: &mut impl crate::data::comparator::ComparatorPolicyResolver,
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
            packets.sort_by_key(|packet| packet.group_index());
        }
    }

    let mut semantic_batch = StageSemanticBatch::default();
    let mut pending_snapshots = Vec::new();
    let mut commits = packets
        .into_iter()
        .flat_map(GroupLocalApplyPacket::into_task_commits)
        .collect::<Vec<_>>();
    commits.sort_by_key(GroupLocalTaskCommit::task_index);
    for commit in commits {
        let update = match publish_group_local_task_commit(
            graph,
            commit,
            &mut pending_snapshots,
            comparator_resolver,
        ) {
            Ok(update) => update,
            Err(failure) => {
                record_grouped_apply_failure(graph, summary, stage_index, &failure);
                return Err(failure.error);
            }
        };
        semantic_batch.push_segment(segment_for_single_update(update));
    }
    graph
        .telemetry_mut()
        .execution
        .shared_surface_publication_breadth +=
        semantic_batch.segment_count() as u64 + pending_snapshots.len() as u64;
    Ok(StageScratch::new(
        StageFinalizeWork::Parallel(crate::data::proof::SingleConsumer::new(semantic_batch)),
        SnapshotBatchCommit::from_unique_pending_snapshots_in_stage_order(pending_snapshots)
            .classify(),
    ))
}

#[cfg(feature = "parallel")]
fn publish_group_local_task_commit(
    graph: &mut SignalGraph,
    commit: GroupLocalTaskCommit,
    pending_snapshots: &mut Vec<crate::logic::evaluation::PendingDependencySnapshot>,
    comparator_resolver: &mut impl crate::data::comparator::ComparatorPolicyResolver,
) -> Result<SemanticTaskUpdate, GroupedApplyFailure> {
    let (
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
    ) = commit.into_parts();
    let (report, pending_snapshot) = graph
        .publish_prepared_parallel_apply_commit_packet(commit_packet, comparator_resolver)
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
    let after_trace = graph
        .node_runtime_artifact_operational_summary(node)
        .map_err(|error| GroupedApplyFailure {
            node,
            record_id: identity.record_id,
            error,
            reuse_failure: None,
        })?;
    let memoized_origin = after_trace
        .as_ref()
        .map(|trace| trace.memoized_origin)
        .unwrap_or(MemoizedResultOrigin::DirectCompute);
    let reuse_basis = after_trace
        .map(|trace| trace.reuse_basis)
        .unwrap_or_else(ReuseBasis::fresh_compute);
    Ok(SemanticTaskUpdate::new(
        task_index,
        node,
        identity,
        before_state,
        before_artifact_state,
        after_state,
        dependency_updates,
        recomputed,
        partition_aware,
        report.temporal_eligibility,
        rewiring,
        report.verdict,
        memoized_origin,
        reuse_basis,
    ))
}

#[cfg(feature = "parallel")]
fn grouped_apply_failure_from_build_error(
    node: NodeId,
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
pub(super) fn record_grouped_apply_failure(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    failure: &GroupedApplyFailure,
) {
    if let Some(reuse_failure) = failure.reuse_failure {
        record_reuse_rejection_telemetry(graph, &reuse_failure);
    }
    crate::logic::planner::execution::task_reporting::record_execution_failure(
        graph,
        crate::diagnostics::failure::ExecutionFailureContext::new(
            crate::diagnostics::failure::ExecutionFailurePhase::Apply,
            Some(stage_index),
            Some(failure.node),
            Some(StageExecutor::full_parallel(1)),
            Some(failure.record_id),
            Some(*summary),
            failure.error.to_string(),
        ),
    );
}

#[cfg(feature = "parallel")]
pub(super) fn build_concurrent_apply_group_inputs(
    tasks: Vec<LoweredTask>,
    dependency_inputs: Vec<EffectDependencyInputs>,
    groups: &[DisjointApplyGroup],
    stage_identities: &[StageSemanticIdentity],
) -> Result<Vec<ConcurrentApplyGroupInput>, SignalError> {
    let mut task_slots = tasks.into_iter().map(Some).collect::<Vec<_>>();
    let mut dependency_input_slots = dependency_inputs.into_iter().map(Some).collect::<Vec<_>>();
    let mut group_inputs = Vec::with_capacity(groups.len());
    for (group_index, group) in groups.iter().enumerate() {
        let mut worker_inputs = Vec::with_capacity(group.task_indices.len());
        for &task_index in &group.task_indices {
            let lowered_task = take_slot(
                &mut task_slots[task_index],
                "grouped concurrent lowered task slot was consumed more than once",
            )?;
            let dependency_input = take_slot(
                &mut dependency_input_slots[task_index],
                "grouped concurrent dependency inputs no longer align with lowered tasks",
            )?;
            worker_inputs.push(
                lowered_task
                    .into_concurrent_worker_input(stage_identities[task_index], dependency_input),
            );
        }
        group_inputs.push(ConcurrentApplyGroupInput::new(group_index, worker_inputs));
    }
    Ok(group_inputs)
}

#[cfg(feature = "parallel")]
fn take_slot<T>(slot: &mut Option<T>, context: &'static str) -> Result<T, SignalError> {
    slot.take().ok_or_else(|| SignalError::internal(context))
}

#[cfg(feature = "parallel")]
pub(super) fn can_lower_true_grouped_concurrent(
    graph: &SignalGraph,
    tasks: &[LoweredTask],
    groups: &[DisjointApplyGroup],
) -> bool {
    !groups.is_empty()
        && tasks.iter().all(|task| {
            task.execution().dependency_updates() == 0
                && task.execution().rewiring().is_none()
                && matches!(
                    graph
                        .node_eval_config(task.node())
                        .map(|config| &config.output_equivalence),
                    Ok(OutputEquivalencePolicy::ExactAspectVersion
                        | OutputEquivalencePolicy::OutputIdentity
                        | OutputEquivalencePolicy::AspectVersionTolerance { .. })
                )
        })
}

#[cfg(feature = "parallel")]
impl LoweredTask {
    fn into_concurrent_worker_input(
        self,
        identity: StageSemanticIdentity,
        dependency_inputs: EffectDependencyInputs,
    ) -> ConcurrentWorkerInput {
        let comparator_policy = self.comparator_policy();
        let (
            task_index,
            node,
            _produced_aspects,
            _dependency_inputs,
            _path_class,
            _authority_policy,
            _footprint,
            execution,
        ) = self.into_parts();
        let (
            prepared,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        ) = execution.into_parts();
        ConcurrentWorkerInput::new(
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
        )
    }
}
