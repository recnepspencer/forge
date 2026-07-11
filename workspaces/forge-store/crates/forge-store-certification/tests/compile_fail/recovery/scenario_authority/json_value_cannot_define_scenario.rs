use forge_store_physical_certification::physical_scenario;

fn main() {
    let json = serde_json::json!({"scenario": "store.physical.s5.readiness"});
    let _scenario = physical_scenario(json);
}
