use worth_store_physical_certification::PhysicalCertificationEvidenceBundle;

struct SameRunComparison;

fn requires_evidence(_: PhysicalCertificationEvidenceBundle) {}

fn main() {
    requires_evidence(SameRunComparison);
}
