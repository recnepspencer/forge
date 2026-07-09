fn main() {
    let _id: worth_store::MaintenanceDeclarationId =
        serde_json::from_str("\"maintenance:1\"")
            .expect("maintenance declaration ids should not deserialize");
}
