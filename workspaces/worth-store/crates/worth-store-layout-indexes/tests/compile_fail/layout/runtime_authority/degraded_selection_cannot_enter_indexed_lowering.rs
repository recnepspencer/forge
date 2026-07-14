use worth_store_layout_indexes::{btree_lookup_runtime, SelectedDegradedExactScan};

fn cross_owner(selected: SelectedDegradedExactScan) {
    let _ = btree_lookup_runtime().execute(selected, panic!(), panic!());
}
