use worth_query::facade::WorthQueryExistingTruthProbeRequest;

fn assert_no_terminal_path_projection(request: &WorthQueryExistingTruthProbeRequest) {
    let _ = request.terminal_aspect_paths_projection();
}

fn main() {}
