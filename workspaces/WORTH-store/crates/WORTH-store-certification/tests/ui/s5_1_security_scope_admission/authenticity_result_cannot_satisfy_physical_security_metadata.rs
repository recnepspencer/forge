use worth_store_security::{
    StoreAuthenticityResult, StorePhysicalSecurityMetadataCarrier,
};

fn requires_physical_security_metadata(_: StorePhysicalSecurityMetadataCarrier) {}

fn main() {
    let result: StoreAuthenticityResult = todo!();
    requires_physical_security_metadata(result);
}
