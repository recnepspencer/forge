use worth_query::facade::WorthQueryBatchWriteReceipt;

fn assert_no_neutral_touched_path_alias(receipt: &WorthQueryBatchWriteReceipt) {
    let _ = receipt.touched_aspect_paths();
}

fn main() {}
