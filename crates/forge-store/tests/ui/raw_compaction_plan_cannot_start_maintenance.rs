use forge_store::{CompactionPlan, ForgeStoreBuilder};

fn main() {
    let mut store = ForgeStoreBuilder::new()
        .local_file("compile-fail-maintenance-a")
        .build()
        .unwrap();
    let plan: CompactionPlan = panic!("no plan");
    let _ = store.start_maintenance_declaration(&plan);
}
