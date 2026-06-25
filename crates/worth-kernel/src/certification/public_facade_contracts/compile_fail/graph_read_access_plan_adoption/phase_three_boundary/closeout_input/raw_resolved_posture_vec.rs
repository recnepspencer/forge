use worth_kernel::graph_read_access_plan_adoption::{
    current_worth_graph_read_access_posture_matrix_closeout, WorthGraphReadAccessResolvedPosture,
};

fn main() {
    let raw_rows: Vec<WorthGraphReadAccessResolvedPosture> = Vec::new();
    let _ = current_worth_graph_read_access_posture_matrix_closeout(&raw_rows);
}
