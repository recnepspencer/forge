use forge_query::facade::ForgeQueryAdmittedGraphReadAccessPlan;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_closeout;

fn main() {
    let plan: &ForgeQueryAdmittedGraphReadAccessPlan =
        panic!("raw Query access plan cannot seed Phase 8");
    let _ = current_worth_graph_read_access_plan_adoption_closeout(plan);
}
