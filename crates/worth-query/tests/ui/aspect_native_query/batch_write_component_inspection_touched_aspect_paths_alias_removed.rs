use worth_query::facade::WorthQueryBatchWriteComponentInspection;

fn assert_no_neutral_touched_path_alias(inspection: &WorthQueryBatchWriteComponentInspection) {
    let _ = inspection.touched_aspect_paths();
}

fn main() {}
