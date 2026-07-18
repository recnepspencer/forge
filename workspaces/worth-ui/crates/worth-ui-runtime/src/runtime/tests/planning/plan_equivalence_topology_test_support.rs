use crate::runtime::{WorthUiExecutionPlan, WorthUiPlanTopology};

pub(super) fn execution_plan_with_topology(
    plan: &WorthUiExecutionPlan,
    topology: WorthUiPlanTopology,
) -> WorthUiExecutionPlan {
    WorthUiExecutionPlan::new(
        plan.handle_receipt(),
        topology,
        plan.lane_partitions().to_vec(),
        plan.lookup_index().clone(),
        plan.counters(),
    )
}
