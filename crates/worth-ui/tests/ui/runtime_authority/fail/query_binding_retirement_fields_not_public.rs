use worth_ui::facade::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryBindingRetirement,
    WorthUiQueryBindingRetirementReason,
};

fn forged_query_binding_identity() -> WorthUiQueryBindingIdentity {
    panic!("fixture should not run")
}

fn forged_query_binding_posture() -> WorthUiQueryBindingPosture {
    panic!("fixture should not run")
}

fn main() {
    let _forged_retirement = WorthUiQueryBindingRetirement {
        identity: forged_query_binding_identity(),
        active_posture: forged_query_binding_posture(),
        reason: WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding,
    };
}
