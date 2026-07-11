use forge_store_physical_format::PhysicalAuthenticityIdentity;
use forge_store_physical_integrity::DeclaredPhysicalChecksum;
use forge_store_security::StoreAuthenticityResult;

fn require_authenticity_result(_: StoreAuthenticityResult<PhysicalAuthenticityIdentity>) {}

fn main() {
    let checksum = DeclaredPhysicalChecksum::new(7);
    require_authenticity_result(checksum);
}
