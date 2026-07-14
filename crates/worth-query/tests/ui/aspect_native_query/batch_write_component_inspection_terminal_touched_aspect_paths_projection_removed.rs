use worth_query::facade::runtime::WorthQueryBatchWriteComponentInspection;

fn assert_no_terminal_touched_path_projection(
    inspection: &WorthQueryBatchWriteComponentInspection,
) {
    let _ = inspection.terminal_touched_aspect_paths_projection();
}

fn main() {}
