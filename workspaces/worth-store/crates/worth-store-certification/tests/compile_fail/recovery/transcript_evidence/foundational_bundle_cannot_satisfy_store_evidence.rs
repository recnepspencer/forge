use worth_store_physical_certification::{
    FoundationalPhysicalCertificationEvidenceBundle, PhysicalCertificationEvidenceBundle,
};

fn requires_store_evidence(_: PhysicalCertificationEvidenceBundle) {}

fn main() {
    let foundational: FoundationalPhysicalCertificationEvidenceBundle = panic!("type-check only");
    requires_store_evidence(foundational);
}
