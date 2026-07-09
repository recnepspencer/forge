use worth_store_security::{StoreJwtSubjectClaim, StoreTenantScope};

fn require_tenant_scope(_: StoreTenantScope) {}

fn main() {
    require_tenant_scope(StoreJwtSubjectClaim::raw("sub-123"));
}
