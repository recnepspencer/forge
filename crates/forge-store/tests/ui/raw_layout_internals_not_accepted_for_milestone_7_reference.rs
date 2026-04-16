use forge_store::{AspectLayoutSliceId, ForgeStoreBuilder};

fn main() {
    let store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let _ = store.admit_milestone_7_independent_layout_reference(AspectLayoutSliceId::new("slice-a"));
}
