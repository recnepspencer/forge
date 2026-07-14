use worth_store_layout_indexes::{degraded_scan_runtime, SelectedIndexedAccessPlan};

fn cross_owner(selected: SelectedIndexedAccessPlan) {
    let _ = degraded_scan_runtime().lower(selected);
}
