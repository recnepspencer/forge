use forge_store::DeferredMaintenancePlan;

fn main() {
    let _: DeferredMaintenancePlan = serde_json::from_str("{}").unwrap();
}
