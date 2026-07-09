fn main() {
    let _declaration: worth_store::MaintenanceDeclaration =
        serde_json::from_str("{}").expect("maintenance declarations should not deserialize");
}
