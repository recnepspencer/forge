fn main() {
    let _id: forge_store::MaintenanceDeclarationId =
        serde_json::from_str("\"maintenance:1\"")
            .expect("maintenance declaration ids should not deserialize");
}
