use forge_query::facade::{
    ForgeQueryBridgeBackedVerificationSupportRow,
    ForgeQueryBridgeBackedVerificationSupportStatus,
};

fn main() {
    let _ = ForgeQueryBridgeBackedVerificationSupportRow {
        operation_family: String::from("verify_existing"),
        target_binding_family: String::from("direct_entity_identity"),
        current_posture_status: ForgeQueryBridgeBackedVerificationSupportStatus::Admitted,
        compatibility_runtime_supported: true,
        primary_bridge_backed_runtime_supported: false,
        denial_class_when_unsupported: None,
        row_digest: String::new(),
    };
}
