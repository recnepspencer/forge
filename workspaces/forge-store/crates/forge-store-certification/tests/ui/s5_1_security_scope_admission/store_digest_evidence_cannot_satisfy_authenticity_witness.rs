use forge_store_aspect_native::StoreDigestEvidence;
use forge_store_security::StoreAuthenticityWitnessInput;

fn require_authenticity_witness(_: StoreAuthenticityWitnessInput) {}

fn main() {
    let digest: StoreDigestEvidence = todo!();
    require_authenticity_witness(digest);
}
