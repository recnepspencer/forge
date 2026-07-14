use worth_store_physical_format::PhysicalAuthenticityIdentity;
use worth_store_physical_integrity::DeclaredPhysicalChecksum;
use worth_store_security::StoreAuthenticityResult;

fn require_authenticity_result(_: StoreAuthenticityResult<PhysicalAuthenticityIdentity>) {}

fn main() {
    let checksum = DeclaredPhysicalChecksum::new(7);
    require_authenticity_result(checksum);
}
