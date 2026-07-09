use worth_query::facade::WorthQueryBatchWriteReceiptInspection;

fn assert_no_terminal_touched_path_projection(
    inspection: &WorthQueryBatchWriteReceiptInspection,
) {
    let _ = inspection.terminal_touched_aspect_paths_projection();
}

fn main() {}
