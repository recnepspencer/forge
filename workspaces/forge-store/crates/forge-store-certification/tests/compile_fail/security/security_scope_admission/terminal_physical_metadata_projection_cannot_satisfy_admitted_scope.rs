use forge_store_security::{StoreAdmittedSecurityScope, StoreRawSecurityMetadataProjection};

fn requires_admitted_scope(_: StoreAdmittedSecurityScope) {}

fn main() {
    let projection: StoreRawSecurityMetadataProjection = todo!();
    requires_admitted_scope(projection);
}
