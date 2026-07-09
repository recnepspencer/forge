use worth_store::{WORTHStore, WORTHStoreBuilder};

fn main() {
    let export = WORTHStoreBuilder::new()
        .in_memory()
        .build()
        .unwrap()
        .export_authoritative_records();
    let _ = WORTHStore::restore_from_authoritative_export(export);
}
