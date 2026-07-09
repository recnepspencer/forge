use worth_store_physical_integrity::DeclaredPhysicalChecksum;
use worth_store_security::StoreAuthenticityWitnessInput;

fn require_authenticity_witness(_: StoreAuthenticityWitnessInput) {}

fn main() {
    let checksum = DeclaredPhysicalChecksum::new(7);
    require_authenticity_witness(checksum);
}
