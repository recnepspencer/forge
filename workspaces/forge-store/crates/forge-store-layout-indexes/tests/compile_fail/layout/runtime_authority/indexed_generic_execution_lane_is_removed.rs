use forge_store_layout_indexes::{indexed_access_runtime, BTreeLookupReady};

fn bypass(ready: BTreeLookupReady) {
    let _ = indexed_access_runtime().execute(ready, panic!());
}

fn main() {}
