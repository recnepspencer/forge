use forge_store_security::{StoreCustodyPosture, StoreTenantScope};

fn require_tenant_scope(_: StoreTenantScope) {}

fn main() {
    require_tenant_scope(StoreCustodyPosture::InternalStoreCustody);
}
