use forge_query::facade::ForgeQueryBatchWriteReceipt;

fn assert_no_terminal_touched_path_projection(receipt: &ForgeQueryBatchWriteReceipt) {
    let _ = receipt.terminal_touched_aspect_paths_projection();
}

fn main() {}
