use forge_query::facade::ForgeQueryGraphCompositionResolutionEntry;

fn assert_terminal_projection_removed(entry: &ForgeQueryGraphCompositionResolutionEntry) {
    let _ = entry.terminal_aspect_path_projection();
}

fn main() {}
