use crate::runtime::{WorthUiExecutionPlan, WorthUiPlanTopology};

pub(super) fn execution_plan_with_topology(
    plan: &WorthUiExecutionPlan,
    topology: WorthUiPlanTopology,
) -> WorthUiExecutionPlan {
    plan.with_test_parts(
        topology,
        plan.lane_partitions().to_vec(),
        plan.lookup_index().clone(),
        plan.counters(),
    )
}
