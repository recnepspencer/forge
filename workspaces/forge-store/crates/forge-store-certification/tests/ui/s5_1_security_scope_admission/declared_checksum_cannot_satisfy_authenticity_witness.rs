use forge_store_physical_integrity::DeclaredPhysicalChecksum;
use forge_store_physical_format::PhysicalAuthenticityIdentity;
use forge_store_security::StoreAuthenticityWitnessInput;

fn require_authenticity_witness(_: StoreAuthenticityWitnessInput<PhysicalAuthenticityIdentity>) {}

fn main() {
    let checksum = DeclaredPhysicalChecksum::new(7);
    require_authenticity_witness(checksum);
}
