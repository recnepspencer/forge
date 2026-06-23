use forge_query::facade::ForgeQueryBatchWriteReceiptInspection;

fn assert_no_neutral_touched_path_alias(inspection: &ForgeQueryBatchWriteReceiptInspection) {
    let _ = inspection.touched_aspect_paths();
}

fn main() {}
