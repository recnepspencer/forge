use forge_store_physical_certification::PhysicalCertificationEvidenceBundle;

fn requires_evidence(_: PhysicalCertificationEvidenceBundle) {}

fn main() {
    let terminal_json = serde_json::json!({ "projection": "terminal", "seed": 8 });
    requires_evidence(terminal_json);
}
