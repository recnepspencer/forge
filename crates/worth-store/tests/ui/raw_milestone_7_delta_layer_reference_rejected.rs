use worth_store::{BranchDeltaLayerId, WORTHStoreBuilder};

fn main() {
    let store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let layer_id = BranchDeltaLayerId(1);
    let _ = store.read_branch_delta_control_from_milestone_7_reference(layer_id);
}
