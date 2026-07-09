use worth_store::{CompactionPlan, WORTHStoreBuilder};

fn main() {
    let mut store = WORTHStoreBuilder::new()
        .local_file("compile-fail-maintenance-a")
        .build()
        .unwrap();
    let plan: CompactionPlan = panic!("no plan");
    let _ = store.start_maintenance_declaration(&plan);
}
