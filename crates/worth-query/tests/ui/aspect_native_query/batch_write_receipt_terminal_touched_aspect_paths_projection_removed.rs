use worth_query::facade::runtime::WorthQueryBatchWriteReceipt;

fn assert_no_terminal_touched_path_projection(receipt: &WorthQueryBatchWriteReceipt) {
    let _ = receipt.terminal_touched_aspect_paths_projection();
}

fn main() {}
