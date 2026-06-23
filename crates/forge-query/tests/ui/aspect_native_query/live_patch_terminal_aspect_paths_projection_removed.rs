use forge_query::facade::ForgeQueryLivePatch;

fn assert_no_terminal_path_projection(patch: &ForgeQueryLivePatch) {
    let _ = patch.terminal_aspect_paths_projection();
}

fn main() {}
