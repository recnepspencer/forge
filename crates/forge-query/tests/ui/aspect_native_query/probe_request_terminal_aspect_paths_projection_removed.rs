use forge_query::facade::ForgeQueryExistingTruthProbeRequest;

fn assert_no_terminal_path_projection(request: &ForgeQueryExistingTruthProbeRequest) {
    let _ = request.terminal_aspect_paths_projection();
}

fn main() {}
