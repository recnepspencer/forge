#[cfg(feature = "parallel")]
use crate::logic::planner::types::{
    ApplyPlanSerialFallbackReason, ConcurrentApplyPlan, ConcurrentApplyReductionPlan,
    DisjointApplyProof, MutationDomain, ReductionOrderingContract, ReductionWorkClass,
    SharedSurfacePolicy,
};
use crate::logic::planner::types::{
    DisjointApplyGroup, LoweredApplyPlan, LoweredTask, SerialApplyPlan, StageExecutor,
};

pub(super) fn build_lowered_apply_plan(
    graph: &crate::data::graph::SignalGraph,
    stage_index: u32,
    tasks: &[LoweredTask],
    executor: StageExecutor,
) -> LoweredApplyPlan {
    #[cfg(not(feature = "parallel"))]
    let _ = (graph, stage_index, executor);

    let serial_groups = || {
        tasks
            .iter()
            .enumerate()
            .map(|(task_index, task)| DisjointApplyGroup {
                task_indices: vec![task_index],
                footprint: task.footprint().clone(),
            })
            .collect::<Vec<_>>()
    };

    #[cfg(feature = "parallel")]
    if executor.is_full_parallel() {
        if let Some(policy) = executor.parallel_policy() {
            let installed_threshold = graph.installed_runtime_policy().full_parallel_min_tasks();
            if tasks.len() < installed_threshold {
                return LoweredApplyPlan::Serial(SerialApplyPlan {
                    groups: serial_groups(),
                    rejection_reason: Some(
                        ApplyPlanSerialFallbackReason::BelowFullParallelThreshold,
                    ),
                });
            }
            let groups = super::concurrent_packets::build_stage_apply_groups(tasks, policy);
            if super::concurrent_packets::can_lower_true_grouped_concurrent(graph, tasks, &groups) {
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

    LoweredApplyPlan::Serial(SerialApplyPlan {
        groups: serial_groups(),
        rejection_reason: None,
    })
}
