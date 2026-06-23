use forge_query::facade::ForgeQueryBatchWriteComponentInspection;

fn assert_no_neutral_touched_path_alias(inspection: &ForgeQueryBatchWriteComponentInspection) {
    let _ = inspection.touched_aspect_paths();
}

fn main() {}
