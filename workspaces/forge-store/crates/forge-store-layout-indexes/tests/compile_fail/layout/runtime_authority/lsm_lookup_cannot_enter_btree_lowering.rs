use forge_store_layout_indexes::{btree_lookup_runtime, SelectedLsmLookup};

fn wrong_machine(selected: SelectedLsmLookup) {
    let _ = btree_lookup_runtime().execute(selected, panic!(), panic!());
}
