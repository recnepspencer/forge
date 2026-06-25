use forge_query::facade::ForgeQueryGraphReadAccessPlanConsumption;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_closeout;

fn main() {
    let plan_consumption: &ForgeQueryGraphReadAccessPlanConsumption =
        panic!("raw Query plan consumption cannot seed Phase 8");
    let _ = current_worth_graph_read_access_plan_adoption_closeout(plan_consumption);
}
