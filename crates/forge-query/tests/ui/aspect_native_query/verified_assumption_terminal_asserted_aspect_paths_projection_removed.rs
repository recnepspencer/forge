use forge_query::facade::ForgeQueryVerifiedAssumptionSet;

fn assert_no_terminal_asserted_path_projection(assumptions: &ForgeQueryVerifiedAssumptionSet) {
    let _ = assumptions.terminal_asserted_aspect_paths_projection();
}

fn main() {}
