use forge_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};

fn main() {
    let serde_projection: serde_json::Value =
        serde_json::from_str(r#"{"tenant_scope":"tenant-a"}"#).unwrap();
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::security_foundation(),
        serde_projection,
    );
}
