use forge_query::facade::ForgeQueryBatchWriteReceipt;

fn assert_no_neutral_touched_path_alias(receipt: &ForgeQueryBatchWriteReceipt) {
    let _ = receipt.touched_aspect_paths();
}

fn main() {}
