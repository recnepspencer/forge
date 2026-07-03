use forge_store_security::{
    StoreAdmittedSecurityScope, StoreRawPhysicalSecurityMetadataProjection,
};

fn requires_admitted_scope(_: StoreAdmittedSecurityScope) {}

fn main() {
    let projection: StoreRawPhysicalSecurityMetadataProjection = todo!();
    requires_admitted_scope(projection);
}
