use worth_store::{WORTHStoreBuilder, VerifiedTierReplica};

fn main() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let verified: VerifiedTierReplica = panic!();
    let _ = store.retire_tier_replica(verified);
}
