use forge_store_layout_indexes::{btree_lookup_runtime, SelectedIndexedAccessPlan};

fn skip_operation_selection(selected: SelectedIndexedAccessPlan) {
    let _ = btree_lookup_runtime().execute(selected, panic!(), panic!());
}
