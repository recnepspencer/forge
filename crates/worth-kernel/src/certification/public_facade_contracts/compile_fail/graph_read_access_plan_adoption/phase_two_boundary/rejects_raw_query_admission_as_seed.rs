use forge_query::facade::ForgeQueryGraphReadAccessAdmission;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_plan_adoption_phase_two_closeout;

fn main() {
    fn misuse(admission: &ForgeQueryGraphReadAccessAdmission) {
        let _ = current_worth_graph_read_access_plan_adoption_phase_two_closeout(admission);
    }

    let _ = misuse;
}
