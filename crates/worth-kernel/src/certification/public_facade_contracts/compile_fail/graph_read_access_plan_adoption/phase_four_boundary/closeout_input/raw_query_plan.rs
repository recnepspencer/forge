use forge_query::facade::ForgeQueryAdmittedGraphReadAccessPlan;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_first_vertical_slice_closeout;

fn main() {
    let raw_plan: Option<ForgeQueryAdmittedGraphReadAccessPlan> = None;
    let _ = current_worth_graph_read_access_first_vertical_slice_closeout(&raw_plan);
}
