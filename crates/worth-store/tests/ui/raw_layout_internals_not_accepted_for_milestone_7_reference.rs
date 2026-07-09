use worth_store::{AspectLayoutSliceId, WORTHStoreBuilder};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let _ = store.admit_milestone_7_independent_layout_reference(AspectLayoutSliceId::new("slice-a"));
}
