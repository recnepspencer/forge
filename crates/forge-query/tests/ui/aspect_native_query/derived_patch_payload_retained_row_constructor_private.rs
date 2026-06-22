use forge_query::facade::{ForgeQueryDerivedPatchPayload, ForgeQueryRetainedMaterializedRow};

fn main() {
    let _ = ForgeQueryDerivedPatchPayload::from_retained_row(retained_row());
}

fn retained_row() -> ForgeQueryRetainedMaterializedRow {
    loop {}
}
