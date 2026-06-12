use worth_ui::facade::{ViewBindingDescriptor, ViewBindingFamily, ViewBindingId};

fn main() {
    let _descriptor = ViewBindingDescriptor {
        id: ViewBindingId::new("workspace.view_binding.tasks").unwrap(),
        family: ViewBindingFamily::collection(),
        query_capability: None,
        query_composition_profile_digest: None,
        view_shape: None,
        result_shape: None,
        basis_posture: None,
        live_compatibility: None,
        visible_state_bindings: Vec::new(),
        denial_presentation: None,
        local_pseudo_query_claim: None,
    };
}
