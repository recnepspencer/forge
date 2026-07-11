use forge_store_security::{
    StoreAdmittedSecurityScope, StoreRawSecurityMetadataDeclaration,
    StoreRawSecurityMetadataProjection,
};

fn requires_admitted_scope(_: StoreAdmittedSecurityScope) {}

fn main() {
    let declaration: StoreRawSecurityMetadataDeclaration = todo!();
    let projection = StoreRawSecurityMetadataProjection::serde_loaded(declaration);
    requires_admitted_scope(projection);
}
