use forge_store::{ForgeStoreBuilder, ReclaimEligibilityWitness};

fn main() {
    let mut store = ForgeStoreBuilder::new()
        .local_file("compile-fail-maintenance-b")
        .build()
        .unwrap();
    let witness: ReclaimEligibilityWitness = panic!("no witness");
    let _ = store.admit_maintenance_batch(witness);
}
