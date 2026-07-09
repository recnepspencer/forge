use worth_store::{WORTHStoreBuilder, ReclaimEligibilityWitness};

fn main() {
    let mut store = WORTHStoreBuilder::new()
        .local_file("compile-fail-maintenance-b")
        .build()
        .unwrap();
    let witness: ReclaimEligibilityWitness = panic!("no witness");
    let _ = store.admit_maintenance_batch(witness);
}
