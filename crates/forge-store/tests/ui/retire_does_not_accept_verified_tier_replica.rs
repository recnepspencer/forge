use forge_store::{ForgeStoreBuilder, VerifiedTierReplica};

fn main() {
    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    let verified: VerifiedTierReplica = panic!();
    let _ = store.retire_tier_replica(verified);
}
