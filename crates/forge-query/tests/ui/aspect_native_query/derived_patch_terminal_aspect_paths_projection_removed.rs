use forge_query::facade::ForgeQueryDerivedPatch;

fn assert_no_terminal_path_projection(patch: &ForgeQueryDerivedPatch) {
    let _ = patch.terminal_aspect_paths_projection();
}

fn main() {}
