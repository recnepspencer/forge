use forge_store_security::StoreTenantScope;

fn require_tenant_scope(_: StoreTenantScope) {}

fn main() {
    let serde_projection: serde_json::Value =
        serde_json::from_str(r#"{"tenant_scope":"tenant-a"}"#).unwrap();
    require_tenant_scope(serde_projection);
}
