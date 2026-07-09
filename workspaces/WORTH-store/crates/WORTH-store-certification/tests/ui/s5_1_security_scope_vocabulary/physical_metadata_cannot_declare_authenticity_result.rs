use worth_store_physical_format::PhysicalSecurityMetadataDeclaration;
use worth_store_security::StoreAuthenticityResult;

fn declare_physical_security_metadata(_: PhysicalSecurityMetadataDeclaration) {}

fn main() {
    let result: StoreAuthenticityResult = todo!();
    declare_physical_security_metadata(result);
}
