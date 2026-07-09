use worth_store_physical_integrity::PhysicalIntegrityEvidenceBundle;
use worth_store_security::StoreAuthenticityWitnessInput;

fn require_authenticity_witness(_: StoreAuthenticityWitnessInput) {}

fn main() {
    let evidence: PhysicalIntegrityEvidenceBundle = todo!();
    require_authenticity_witness(evidence);
}
