use forge_store::{ForgeStore, ForgeStoreBuilder};

fn main() {
    let export = ForgeStoreBuilder::new()
        .in_memory()
        .build()
        .unwrap()
        .export_authoritative_records();
    let _ = ForgeStore::restore_from_authoritative_export(export);
}
