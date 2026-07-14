use worth_store_security::{StoreApplicationOrgIdClaim, StoreTenantScope};

fn require_tenant_scope(_: StoreTenantScope) {}

fn main() {
    require_tenant_scope(StoreApplicationOrgIdClaim::raw("org-123"));
}
