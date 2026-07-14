use worth_query::facade::runtime::WorthQueryRetainedRefreshContext;

fn assert_no_terminal_touched_path_projection(refresh: &WorthQueryRetainedRefreshContext) {
    let _ = refresh.terminal_touched_aspect_paths_projection();
}

fn main() {}
