use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiArtifactInputNode, WorthUiArtifactInputProvenance,
    WorthUiIdentitySeedingDiagnosticCode, WorthUiRustCompositionInput,
    WorthUiRustCompositionModule, WorthUiRustCompositionToArtifactInputLowerer,
};

use super::rust_composition_fixture_support::{
    artifact_input_from_composition, duplicate_authored_identity_rust_composition,
    equivalent_rust_composition, first_token_node, identity_seeding_report_from_composition,
};

#[test]
fn rust_escape_hatch_remains_authoring_lane_only() {
    let composition = equivalent_rust_composition();
    let report = WorthUiRustCompositionToArtifactInputLowerer::lower_with_report(&composition);
    let artifact_input = report.artifact_input();
    let token_node = first_token_node(artifact_input);

    assert_eq!(report.metrics().modules_declared(), 2);
    assert_eq!(report.metrics().declarations_declared(), 6);
    assert!(matches!(
        token_node.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
    ));
}

#[test]
fn rust_composition_lowering_outputs_shared_ir_nodes() {
    let artifact_input = artifact_input_from_composition(&equivalent_rust_composition());
    let main_module = artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("main artifact input module");

    assert!(main_module
        .nodes()
        .iter()
        .any(|node| matches!(node, WorthUiArtifactInputNode::Import(_))));
    assert!(main_module
        .nodes()
        .iter()
        .any(|node| matches!(node, WorthUiArtifactInputNode::Token(_))));
}

#[test]
fn rust_composition_cannot_bypass_identity_seed_conflicts() {
    let report =
        identity_seeding_report_from_composition(&duplicate_authored_identity_rust_composition());
    let diagnostics = report.diagnostics();

    assert_eq!(diagnostics.len(), 1);
    assert_eq!(
        diagnostics[0].code(),
        WorthUiIdentitySeedingDiagnosticCode::DuplicateAuthoredIdentitySeed
    );
    assert_eq!(diagnostics[0].authored_identity(), "duplicate.identity");
}

#[test]
fn rust_composition_full_authoring_surface_still_lowers_to_shared_ir() {
    let artifact_input =
        artifact_input_from_composition(&full_authoring_surface_rust_composition());
    let nodes = artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("artifact input module")
        .nodes();

    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Component(component)
            if component.authored_identity() == Some("component.identity")
                && component.body_atoms() == [identifier_atom("component.body")]
    )));
    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Surface(surface)
            if surface.authored_identity() == Some("surface.identity")
    )));
    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Surface(surface)
            if surface.body_atoms() == [identifier_atom("surface.body")]
    )));
    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Binding(binding)
            if binding.authored_identity() == Some("binding.identity")
    )));
    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Binding(binding)
            if binding.body_atoms() == [identifier_atom("binding.body")]
    )));
    assert!(nodes.iter().any(|node| matches!(
        node,
        WorthUiArtifactInputNode::Token(token)
            if token.authored_identity() == Some("token.identity")
                && token.value_text() == "theme.text.primary"
    )));
}

fn full_authoring_surface_rust_composition() -> WorthUiRustCompositionInput {
    WorthUiRustCompositionInput::from_modules([WorthUiRustCompositionModule::new("app/main.wui")
        .component_body_atoms_and_authored_identity(
            "workspace.component.dashboard",
            "component.identity",
            [identifier_atom("component.body")],
        )
        .surface_authored_identity("workspace.surface.inspector", "surface.identity")
        .surface_body_atoms("workspace.surface.main", [identifier_atom("surface.body")])
        .binding_authored_identity("workspace.view_binding.selection", "binding.identity")
        .binding_body_atoms(
            "workspace.view_binding.selection",
            [identifier_atom("binding.body")],
        )
        .token_authored_identity("theme.text.default", "token.identity", "theme.text.primary")])
}

fn identifier_atom(text: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(text.to_owned())
}
