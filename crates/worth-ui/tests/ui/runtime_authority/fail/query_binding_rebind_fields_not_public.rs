use worth_ui::facade::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingRebind,
    WorthUiQueryBindingRebindReason, WorthUiQueryRebindRequiredSurface,
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

fn forged_required_query_surfaces() -> Vec<WorthUiQueryRebindRequiredSurface> {
    panic!("fixture should not run")
}

fn main() {
    let _forged_rebind = WorthUiQueryBindingRebind {
        identity: forged_query_binding_identity(),
        candidate_posture: forged_query_binding_posture(),
        reason: WorthUiQueryBindingRebindReason::FreshCandidateBinding,
        drift_families: forged_query_binding_drift_families(),
        required_query_surfaces: forged_required_query_surfaces(),
    };
}
