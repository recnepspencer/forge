use worth_query::facade::runtime::WorthQueryBatchWriteReceiptInspection;

fn assert_no_neutral_touched_path_alias(inspection: &WorthQueryBatchWriteReceiptInspection) {
    let _ = inspection.touched_aspect_paths();
}

fn main() {}
