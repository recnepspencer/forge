use worth_store::BackendCapabilityMatrixRow;

fn main() {
    let _row: BackendCapabilityMatrixRow = serde_json::from_str("{}").unwrap();
}
