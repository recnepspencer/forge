#[cfg(feature = "parallel")]
use crate::clock::RuntimeInstant;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
#[cfg(feature = "parallel")]
use crate::logic::evaluation::collect_effect_dependency_inputs_iter;
use crate::logic::planner::semantic::StageSemanticIdentity;
use crate::logic::planner::types::{
    ConcurrentApplyPlan, LoweredTask, PlanSummary, StageExecutionRecord,
};

use super::super::workspace::StageScratch;
#[cfg(feature = "parallel")]
use super::concurrent_packets;

#[cfg(feature = "parallel")]
use crate::logic::planner::precompute::executor_pool::PlannerExecutorPool;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub(super) fn run_grouped_concurrent_apply_pass(
    graph: &mut SignalGraph,
    summary: &PlanSummary,
    stage_index: u32,
    tasks: Vec<LoweredTask>,
    plan: ConcurrentApplyPlan,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
    stage_identities: &[StageSemanticIdentity],
    stage_record: &mut StageExecutionRecord,
) -> Result<StageScratch, SignalError> {
    #[cfg(not(feature = "parallel"))]
    {
        let _ = (
            graph,
            summary,
            stage_index,
            tasks,
            plan,
            stage_identities,
            stage_record,
        );
        return Err(SignalError::internal(
            "grouped concurrent apply requires the `parallel` feature",
        ));
    }

    #[cfg(feature = "parallel")]
    {
        stage_record.apply_mode =
            Some(crate::logic::planner::ParallelApplyMode::GroupedConcurrentApply);
        stage_record.outcome = crate::logic::planner::StageExecutionOutcome::CompletedParallel;
        stage_record.parallel_kind =
            Some(crate::logic::planner::ParallelExecutionKind::FullParallel);
        stage_record.parallel_admission_reason = Some(
            crate::logic::planner::ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent,
        );
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

        let dependency_input_start = RuntimeInstant::now();
        let dependency_inputs =
            collect_effect_dependency_inputs_iter(graph, tasks.iter().map(|task| task.node()))?;
        graph.telemetry_mut().execution.dependency_input_build_nanos +=
            dependency_input_start.elapsed().as_nanos();
        let task_count = tasks.len();
        let group_inputs = concurrent_packets::build_concurrent_apply_group_inputs(
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
                .map(|group| concurrent_packets::build_group_packet(graph_ref, group))
                .collect::<Result<Vec<_>, _>>()
        });
        let group_packets = match group_packets {
            Ok(packets) => packets,
            Err(failure) => {
                concurrent_packets::record_grouped_apply_failure(
                    graph,
                    summary,
                    stage_index,
                    &failure,
                );
                return Err(failure.error);
            }
        };

        graph.telemetry_mut().execution.group_local_packet_breadth += group_packets
            .iter()
            .map(|packet| packet.packet_breadth() as u64)
            .sum::<u64>();
        graph.telemetry_mut().execution.reduction_packet_breadth += group_packets.len() as u64;
        graph.telemetry_mut().execution.reduction_group_count += group_packets.len() as u64;
        concurrent_packets::reduce_grouped_concurrent_packets(
            graph,
            summary,
            stage_index,
            group_packets,
            plan.reduction,
        )
    }
}
