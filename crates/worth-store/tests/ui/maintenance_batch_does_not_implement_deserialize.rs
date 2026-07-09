fn main() {
    let _batch: worth_store::MaintenanceBatch =
        serde_json::from_str("{}").expect("maintenance batches should not deserialize");
}
