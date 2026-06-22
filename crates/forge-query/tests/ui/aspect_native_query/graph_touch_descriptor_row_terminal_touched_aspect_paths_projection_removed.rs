use forge_query::facade::ForgeQueryGraphTouchDescriptorRow;

fn assert_no_terminal_touched_path_projection(row: &ForgeQueryGraphTouchDescriptorRow) {
    let _ = row.terminal_touched_aspect_paths_projection();
}

fn main() {}
