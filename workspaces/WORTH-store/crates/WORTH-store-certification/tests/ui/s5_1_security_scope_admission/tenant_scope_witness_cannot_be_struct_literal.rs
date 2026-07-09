use worth_store_security::{
    StoreCurrentTenantScopeWitness, StoreSecurityScopeIdentity, StoreTenantScope,
};

fn main() {
    let _WORTHd = StoreCurrentTenantScopeWitness {
        identity: unimplemented!(),
        tenant_scope: StoreTenantScope::TenantPhysicalBoundary,
    };
}
