use worth_store_security::StoreTenantScope;

fn require_tenant_scope(_: StoreTenantScope) {}

fn main() {
    require_tenant_scope("tenant-a");
}
