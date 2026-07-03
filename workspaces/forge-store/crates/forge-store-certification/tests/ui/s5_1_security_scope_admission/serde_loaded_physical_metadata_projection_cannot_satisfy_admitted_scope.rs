use forge_store_security::{
    StoreAdmittedSecurityScope, StoreRawPhysicalSecurityMetadataDeclaration,
    StoreRawPhysicalSecurityMetadataProjection,
};

fn requires_admitted_scope(_: StoreAdmittedSecurityScope) {}

fn main() {
    let declaration: StoreRawPhysicalSecurityMetadataDeclaration = todo!();
    let projection = StoreRawPhysicalSecurityMetadataProjection::serde_loaded(declaration);
    requires_admitted_scope(projection);
}
