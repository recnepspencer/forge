use worth_store::WORTHStoreBuilder;

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let raw_cutover = ("snapshot:42".to_string(), "cold".to_string());
    let _ = store.retire_tier_replica(raw_cutover);
}
