use forge_store_physical_format::PhysicalAuthenticityIdentity;
use forge_store_security::{StoreAuthenticityResult, StoreSecurityMetadata};

fn requires_physical_security_metadata(_: StoreSecurityMetadata) {}

fn main() {
    let result: StoreAuthenticityResult<PhysicalAuthenticityIdentity> = todo!();
    requires_physical_security_metadata(result);
}
