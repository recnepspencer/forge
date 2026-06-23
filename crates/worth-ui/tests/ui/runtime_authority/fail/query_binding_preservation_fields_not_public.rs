use worth_ui::facade::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryBindingPreservation,
};

fn forged_query_binding_identity() -> WorthUiQueryBindingIdentity {
    panic!("fixture should not run")
}

fn forged_query_binding_posture() -> WorthUiQueryBindingPosture {
    panic!("fixture should not run")
}

fn main() {
    let _forged_preservation = WorthUiQueryBindingPreservation {
        identity: forged_query_binding_identity(),
        preserved_posture: forged_query_binding_posture(),
        preservation_receipt: "query-live-preserve:forged".to_owned(),
    };
}
