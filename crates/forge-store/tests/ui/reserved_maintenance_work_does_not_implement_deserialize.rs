use forge_store::ReservedMaintenanceWork;

fn main() {
    let _: ReservedMaintenanceWork = serde_json::from_str("{}").unwrap();
}
