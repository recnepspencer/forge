use worth_query::facade::{WorthQueryDerivedPatchPayload, WorthQueryRetainedMaterializedRow};

fn main() {
    let _ = WorthQueryDerivedPatchPayload::from_retained_row(retained_row());
}

fn retained_row() -> WorthQueryRetainedMaterializedRow {
    loop {}
}
