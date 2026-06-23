use worth_ui::facade::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily,
};

fn forged_query_binding_identity() -> WorthUiQueryBindingIdentity {
    panic!("fixture should not run")
}

fn forged_query_binding_posture() -> WorthUiQueryBindingPosture {
    panic!("fixture should not run")
}

fn forged_query_binding_drift_families() -> Vec<WorthUiQueryBindingPostureDriftFamily> {
    panic!("fixture should not run")
}

fn main() {
    let _forged_denial = WorthUiQueryBindingDriftDenial {
        identity: forged_query_binding_identity(),
        active_posture: Some(forged_query_binding_posture()),
        candidate_posture: Some(forged_query_binding_posture()),
        drift_families: forged_query_binding_drift_families(),
        reason: WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted,
    };
}
