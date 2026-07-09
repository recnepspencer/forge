use worth_store::{WORTHStoreBuilder, MaintenanceDeclarationId};

fn main() {
    let mut store = WORTHStoreBuilder::new()
        .local_file("compile-fail-maintenance-id")
        .build()
        .unwrap();
    let declaration_id: MaintenanceDeclarationId = panic!("no admitted declaration");
    let _ = store.start_maintenance_declaration(&declaration_id);
}
