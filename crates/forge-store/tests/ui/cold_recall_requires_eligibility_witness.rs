use forge_store::{ColdRecallLease, ForgeStoreBuilder};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let lease: ColdRecallLease = panic!();
    let _ = store.execute_cold_recall(lease);
}
