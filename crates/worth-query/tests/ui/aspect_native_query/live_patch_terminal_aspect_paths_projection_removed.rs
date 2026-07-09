use worth_query::facade::WorthQueryLivePatch;

fn assert_no_terminal_path_projection(patch: &WorthQueryLivePatch) {
    let _ = patch.terminal_aspect_paths_projection();
}

fn main() {}
