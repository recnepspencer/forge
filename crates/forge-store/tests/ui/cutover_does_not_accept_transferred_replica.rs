use forge_store::{ForgeStoreBuilder, TransferredTierReplica};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let transferred: TransferredTierReplica = panic!();
    let _ = store.cutover_tier_replica(transferred);
}
