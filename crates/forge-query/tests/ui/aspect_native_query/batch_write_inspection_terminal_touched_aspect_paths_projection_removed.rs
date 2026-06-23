use forge_query::facade::ForgeQueryBatchWriteReceiptInspection;

fn assert_no_terminal_touched_path_projection(
    inspection: &ForgeQueryBatchWriteReceiptInspection,
) {
    let _ = inspection.terminal_touched_aspect_paths_projection();
}

fn main() {}
