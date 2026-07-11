use forge_store_security::{
    StoreCurrentTenantScopeWitness, StoreSecurityScopeIdentity, StoreTenantScope,
};

fn main() {
    let _forged = StoreCurrentTenantScopeWitness {
        identity: unimplemented!(),
        tenant_scope: StoreTenantScope::TenantPhysicalBoundary,
    };
}
