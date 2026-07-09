use worth_query::facade::WorthQueryDerivedPatch;

fn assert_no_terminal_path_projection(patch: &WorthQueryDerivedPatch) {
    let _ = patch.terminal_aspect_paths_projection();
}

fn main() {}
