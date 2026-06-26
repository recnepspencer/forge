use forge_query::facade::ForgeQueryGraphReadAccessPlanConsumption;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_phase_one_closeout;

fn main() {
    fn misuse(plan_consumption: &ForgeQueryGraphReadAccessPlanConsumption) {
        let _ = current_worth_graph_read_access_plan_adoption_phase_one_closeout(plan_consumption);
    }

    let _ = misuse;
}
