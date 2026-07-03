use forge_store_physical_integrity::PhysicalIntegrityEvidenceBundle;
use forge_store_security::StoreAuthenticityWitnessInput;

fn require_authenticity_witness(_: StoreAuthenticityWitnessInput) {}

fn main() {
    let evidence: PhysicalIntegrityEvidenceBundle = todo!();
    require_authenticity_witness(evidence);
}
