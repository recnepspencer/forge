use worth_store::WORTHStoreBuilder;

fn main() {
    let mut store = WORTHStoreBuilder::new()
        .local_file("compile-fail-maintenance-c")
        .build()
        .unwrap();
    let _ = store.resume_maintenance_declaration("not-a-declaration-id");
}
