use worth_query::facade::WorthQueryVerifiedAssumptionSet;

fn assert_no_terminal_asserted_path_projection(assumptions: &WorthQueryVerifiedAssumptionSet) {
    let _ = assumptions.terminal_asserted_aspect_paths_projection();
}

fn main() {}
