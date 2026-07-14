use forge_store_physical_certification::CertifiedPhysicalScenario;

fn requires_certified_scenario(_: CertifiedPhysicalScenario) {}

fn main() {
    let raw_json = br#"{ "scenario": "shortcut" }"#;
    requires_certified_scenario(raw_json);
}
