use forge_query::facade::ForgeQueryBatchWriteComponentInspection;

fn assert_no_terminal_touched_path_projection(
    inspection: &ForgeQueryBatchWriteComponentInspection,
) {
    let _ = inspection.terminal_touched_aspect_paths_projection();
}

fn main() {}
