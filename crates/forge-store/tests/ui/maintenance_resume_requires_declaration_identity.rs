use forge_store::ForgeStoreBuilder;

fn main() {
    let mut store = ForgeStoreBuilder::new()
        .local_file("compile-fail-maintenance-c")
        .build()
        .unwrap();
    let _ = store.resume_maintenance_declaration("not-a-declaration-id");
}
