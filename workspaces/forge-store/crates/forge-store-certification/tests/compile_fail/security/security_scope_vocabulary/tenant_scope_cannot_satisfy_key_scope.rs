use forge_store_security::{StoreKeyScope, StoreTenantScope};

fn require_key_scope(_: StoreKeyScope) {}

fn main() {
    require_key_scope(StoreTenantScope::TenantPhysicalBoundary);
}
