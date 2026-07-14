use worth_query::facade::runtime::WorthQueryGraphTouchDescriptorRow;

fn assert_no_terminal_touched_path_projection(row: &WorthQueryGraphTouchDescriptorRow) {
    let _ = row.terminal_touched_aspect_paths_projection();
}

fn main() {}
