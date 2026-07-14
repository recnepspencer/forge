use worth_query::facade::runtime::WorthQueryGraphCompositionResolutionEntry;

fn assert_terminal_projection_removed(entry: &WorthQueryGraphCompositionResolutionEntry) {
    let _ = entry.terminal_aspect_path_projection();
}

fn main() {}
