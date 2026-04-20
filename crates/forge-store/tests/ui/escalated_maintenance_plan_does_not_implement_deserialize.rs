use forge_store::EscalatedMaintenancePlan;

fn main() {
    let _: EscalatedMaintenancePlan = serde_json::from_str("{}").unwrap();
}
