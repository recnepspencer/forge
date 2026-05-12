use forge_runtime_bridge::facade::{
    BridgeCausalEvidenceBinding, BridgeCausalEvidenceBindingClass, BridgeCausalEvidenceFamily,
    BridgeCausalEvidenceOwner,
};

fn main() {
    let _ = BridgeCausalEvidenceBinding {
        owner: BridgeCausalEvidenceOwner::RuntimeBridge,
        family: BridgeCausalEvidenceFamily::BridgeRoute,
        binding_class: BridgeCausalEvidenceBindingClass::RetainedBridgeRecord,
        reference_identity: "route".into(),
        retained_record_digest: Some("record".into()),
        binding_digest: "binding".into(),
    };
}
