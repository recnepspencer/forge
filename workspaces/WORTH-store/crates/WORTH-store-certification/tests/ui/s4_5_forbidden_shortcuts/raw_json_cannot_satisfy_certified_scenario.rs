use worth_store_physical_certification::CertifiedPhysicalScenario;

fn requires_certified_scenario(_: CertifiedPhysicalScenario) {}

fn main() {
    requires_certified_scenario(serde_json::json!({ "scenario": "shortcut" }));
}
