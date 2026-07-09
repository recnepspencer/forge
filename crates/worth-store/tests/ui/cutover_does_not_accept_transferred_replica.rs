use worth_store::{WORTHStoreBuilder, TransferredTierReplica};

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let transferred: TransferredTierReplica = panic!();
    let _ = store.cutover_tier_replica(transferred);
}
