use worth_ui_query_binding::WorthUiPresentationRecoveryRequiredReceipt;

fn serialized_material_cannot_reconstruct_recovery_authority() {
    fn require_serialize<T: serde::Serialize>() {}
    require_serialize::<WorthUiPresentationRecoveryRequiredReceipt>();
}

fn main() {}
