use forge_store_physical_format::{
    PhysicalAuthenticityIdentity, PhysicalSecurityMetadataDeclaration,
};
use forge_store_security::StoreAuthenticityResult;

fn declare_physical_security_metadata(_: PhysicalSecurityMetadataDeclaration) {}

fn main() {
    let result: StoreAuthenticityResult<PhysicalAuthenticityIdentity> = todo!();
    declare_physical_security_metadata(result);
}
