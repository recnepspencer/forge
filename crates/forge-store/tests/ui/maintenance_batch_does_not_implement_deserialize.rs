fn main() {
    let _batch: forge_store::MaintenanceBatch =
        serde_json::from_str("{}").expect("maintenance batches should not deserialize");
}
