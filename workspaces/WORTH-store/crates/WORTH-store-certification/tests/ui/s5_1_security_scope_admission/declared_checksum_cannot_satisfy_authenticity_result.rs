use worth_store_physical_integrity::DeclaredPhysicalChecksum;
use worth_store_security::StoreAuthenticityResult;

fn require_authenticity_result(_: StoreAuthenticityResult) {}

fn main() {
    let checksum = DeclaredPhysicalChecksum::new(7);
    require_authenticity_result(checksum);
}
