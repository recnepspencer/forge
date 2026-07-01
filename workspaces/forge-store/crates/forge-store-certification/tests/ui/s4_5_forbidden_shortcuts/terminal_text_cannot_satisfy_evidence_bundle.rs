use forge_store_physical_certification::PhysicalCertificationEvidenceBundle;

fn requires_store_evidence(_: PhysicalCertificationEvidenceBundle) {}

fn main() {
    requires_store_evidence(String::from("terminal projection text"));
}
