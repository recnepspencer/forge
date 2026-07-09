use worth_query::facade::{
    WorthQueryBridgeBackedVerificationSupportRow,
    WorthQueryBridgeBackedVerificationSupportStatus,
};

fn main() {
    let _ = WorthQueryBridgeBackedVerificationSupportRow {
        operation_family: String::from("verify_existing"),
        target_binding_family: String::from("direct_entity_identity"),
        current_posture_status: WorthQueryBridgeBackedVerificationSupportStatus::Admitted,
        scaffold_profile_supported: true,
        primary_bridge_backed_runtime_supported: false,
        denial_class_when_unsupported: None,
        row_digest: String::new(),
    };
}
