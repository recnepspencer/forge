use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiArtifactInputResolver, WorthUiResolvedArtifactInputNode,
};

use super::resolution_fixture_support::{admitted_app, standard_artifact_input};

#[test]
fn same_artifact_input_and_same_snapshot_produce_equivalent_resolution() {
    let app = admitted_app();
    let artifact_input = standard_artifact_input();

    let first = WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
        .expect("resolution should succeed");
    let second = WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
        .expect("resolution should succeed");

    assert_eq!(first, second);
    assert!(first.equivalent_shape(&second));

    let main_module = first
        .module(&first.module_ids()[0])
        .expect("main module should exist");
    assert_eq!(main_module.module_id().as_str(), "app/main.wui");
    assert_eq!(main_module.components().len(), 1);
}

#[test]
fn reordered_module_iteration_produces_equivalent_resolution() {
    let app = admitted_app();
    let left = standard_artifact_input();
    let right = crate::source::WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &crate::source::WorthUiRustAuthoredArtifactInput::from_modules([
            crate::source::WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
                .with_component("workspace.component.inspector_panel"),
            crate::source::WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_import("app/panels/inspector.wui")
                .with_component("workspace.component.dashboard")
                .with_surface("workspace.surface.inspector")
                .with_binding("workspace.view_binding.selection")
                .with_token("theme.text.default", "theme.text.primary"),
        ]),
    );

    let left = WorthUiArtifactInputResolver::resolve(&left, app.capabilities())
        .expect("resolution should succeed");
    let right = WorthUiArtifactInputResolver::resolve(&right, app.capabilities())
        .expect("resolution should succeed");

    assert!(left.equivalent_shape(&right));
}

#[test]
fn view_binding_and_theme_token_resolution_preserve_frozen_entry_identity() {
    let app = admitted_app();
    let artifact_input = standard_artifact_input();
    let resolved = WorthUiArtifactInputResolver::resolve(&artifact_input, app.capabilities())
        .expect("resolution should succeed");
    let main_module = resolved
        .module(&resolved.module_ids()[0])
        .expect("main module should exist");

    let binding_node = main_module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiResolvedArtifactInputNode::Binding(binding_node) => Some(binding_node),
            _ => None,
        })
        .expect("binding node should resolve");
    let token_node = main_module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiResolvedArtifactInputNode::Token(token_node) => Some(token_node),
            _ => None,
        })
        .expect("token node should resolve");
    let component_node = main_module
        .components()
        .into_iter()
        .next()
        .expect("component node should resolve");
    let surface_node = main_module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiResolvedArtifactInputNode::Surface(surface_node) => Some(surface_node),
            _ => None,
        })
        .expect("surface node should resolve");

    let frozen_binding_entry = app
        .capabilities()
        .view_bindings()
        .get_entry(binding_node.view_binding().id())
        .expect("frozen binding entry should exist");
    let frozen_token_entry = app
        .capabilities()
        .theme_tokens()
        .get_entry(token_node.theme_token().id())
        .expect("frozen theme token entry should exist");

    assert_eq!(binding_node.entry(), frozen_binding_entry);
    assert_eq!(token_node.entry(), frozen_token_entry);
    assert_eq!(
        component_node.descriptor().id().as_str(),
        "workspace.component.dashboard"
    );
    assert_eq!(
        surface_node.descriptor().id().as_str(),
        "workspace.surface.inspector"
    );
    assert!(matches!(
        component_node.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
    assert!(matches!(
        surface_node.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
    assert!(matches!(
        binding_node.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
    assert!(matches!(
        token_node.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
}
