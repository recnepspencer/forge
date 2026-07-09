use worth_store::{ColdRecallLease, WORTHStoreBuilder};

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let lease: ColdRecallLease = panic!();
    let _ = store.execute_cold_recall(lease);
}
