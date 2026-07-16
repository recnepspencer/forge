use super::binding_app_fixture::admitted_app;
use super::binding_phase_fixture::bound_artifact_input;
use crate::capability::CommandReadinessStatus;
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
        command.semantics().readiness().strongest_status(),
        CommandReadinessStatus::Deferred
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
fn query_bound_view_reference_preserves_the_single_admitted_definition() {
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

    assert_eq!(semantics.definition(), descriptor.definition());
    assert_eq!(
        semantics.definition().lifecycle(),
        worth_ui_query_binding::WorthUiQueryViewLifecycle::Snapshot
    );
    assert_eq!(
        semantics.definition().shape(),
        worth_ui_query_binding::WorthUiQueryViewShape::Collection
    );
    assert_eq!(semantics.definition().required_facts().len(), 1);
    assert_eq!(
        semantics.denial_presentation(),
        descriptor.denial_presentation()
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
