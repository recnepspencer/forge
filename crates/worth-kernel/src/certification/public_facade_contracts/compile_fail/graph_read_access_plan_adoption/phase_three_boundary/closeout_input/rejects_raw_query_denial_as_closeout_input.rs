use forge_query::facade::ForgeQueryGraphReadAccessDenialKind;
use worth_kernel::graph_read_access_plan_adoption::current_worth_graph_read_access_posture_matrix_closeout;

fn main() {
    let denial = ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex;
    let _ = current_worth_graph_read_access_posture_matrix_closeout(&denial);
}
