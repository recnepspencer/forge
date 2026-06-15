use super::binding_app_fixture::admitted_app;
use super::binding_phase_fixture::bound_artifact_input;
use crate::source::WorthUiBoundArtifactInputNode;

#[test]
fn nested_command_and_surface_semantics_preserve_typed_identity() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let bound = bound_artifact_input(snapshot);

    let module = bound.module(bound.module_ids().first().unwrap()).unwrap();
    let surface = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiBoundArtifactInputNode::Surface(surface) => Some(surface),
            _ => None,
        })
        .unwrap();
    let surface_semantics = surface.semantics();
    let command = surface_semantics.command_slots().first().unwrap();

    assert_eq!(
        surface_semantics.icon().unwrap().icon().id().as_str(),
        "workspace.icon.surface.inspector"
    );
    assert_eq!(
        command.semantics().icon().unwrap().icon().id().as_str(),
        "workspace.icon.inspect"
    );
    assert_eq!(
        command
            .semantics()
            .projection_eligibility()
            .unwrap()
            .command_projection()
            .id()
            .as_str(),
        "workspace.command_projection.inspect_actions"
    );
    assert_eq!(
        command.semantics().readiness().strongest_status().as_str(),
        "deferred"
    );
    assert_eq!(
        command
            .semantics()
            .runtime_intent_binding()
            .unwrap()
            .intent_key(),
        "workspace.runtime.inspect"
    );
}

#[test]
fn query_bound_view_reference_preserves_query_owned_posture() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let bound = bound_artifact_input(snapshot);

    let module = bound.module(bound.module_ids().first().unwrap()).unwrap();
    let binding = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiBoundArtifactInputNode::Binding(binding) => Some(binding),
            _ => None,
        })
        .unwrap();
    let semantics = binding.view_binding_reference().query_semantics();
    let descriptor = binding.view_binding_reference().entry().descriptor();

    assert!(semantics.query_capability().is_admitted());
    assert!(!semantics.query_composition_profile_digest().is_empty());
    assert_eq!(
        semantics.query_capability(),
        descriptor.query_capability().unwrap()
    );
    assert_eq!(semantics.view_shape(), descriptor.view_shape().unwrap());
    assert_eq!(
        semantics.result_shape().digest_basis(),
        descriptor.result_shape().unwrap().digest_basis()
    );
    assert_eq!(
        semantics.basis_posture().digest_basis(),
        descriptor.basis_posture().unwrap().digest_basis()
    );
    assert_eq!(
        semantics.live_compatibility().digest_basis(),
        descriptor.live_compatibility().unwrap().digest_basis()
    );
    assert_eq!(
        semantics.denial_presentation(),
        descriptor.denial_presentation().unwrap()
    );
}

#[test]
fn theme_token_resolution_preserves_frozen_target_identity() {
    let app = admitted_app();
    let snapshot = app.capabilities();
    let bound = bound_artifact_input(snapshot);

    let module = bound.module(bound.module_ids().first().unwrap()).unwrap();
    let token = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiBoundArtifactInputNode::Token(token) => Some(token),
            _ => None,
        })
        .unwrap();

    assert_eq!(
        token
            .semantics()
            .resolved_target_theme_token()
            .id()
            .as_str(),
        "theme.text.primary"
    );
    assert_eq!(
        token
            .semantics()
            .resolved_target_entry()
            .descriptor()
            .id()
            .as_str(),
        "theme.text.primary"
    );
}
