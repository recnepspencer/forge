use super::artifact_fixture_support::{
    artifact_binding_node, artifact_component_node, artifact_surface_node, artifact_token_node,
    assembled_artifact_from_modules, canonical_identity_seeded_declaration_input,
    handle_round_trip, identity_seeded_from_modules, imported_modules,
    reordered_identity_seeded_declaration_input, reordered_modules, structureful_component_module,
};

#[test]
fn equivalent_seeded_bound_input_produces_equivalent_canonical_artifact() {
    let (first, _) = assembled_artifact_from_modules(imported_modules());
    let (second, _) = assembled_artifact_from_modules(imported_modules());

    assert!(first.equivalent_shape(&second));
    assert_eq!(first, second);
}

#[test]
fn reordered_module_iteration_produces_equivalent_canonical_artifact() {
    let (left, _) = assembled_artifact_from_modules(imported_modules());
    let (right, _) = assembled_artifact_from_modules(reordered_modules());

    assert!(left.equivalent_shape(&right));
    assert_eq!(left.module_ids(), right.module_ids());
}

#[test]
fn canonical_artifact_normalizes_child_ordering() {
    let (canonical, canonical_metrics) =
        crate::source::WorthUiCanonicalArtifactAssembler::assemble_with_metrics(
            &canonical_identity_seeded_declaration_input(),
        )
        .expect("phase 8 canonical artifact assembly should succeed");
    let (reordered, reordered_metrics) =
        crate::source::WorthUiCanonicalArtifactAssembler::assemble_with_metrics(
            &reordered_identity_seeded_declaration_input(),
        )
        .expect("phase 8 reordered artifact assembly should succeed");

    assert!(canonical.equivalent_shape(&reordered));
    let module = canonical
        .module(canonical.module_ids().first().expect("module id"))
        .expect("artifact module");
    let kinds = module
        .nodes()
        .iter()
        .map(|node| node.handle().kind())
        .collect::<Vec<_>>();

    assert_eq!(
        kinds,
        vec![
            crate::source::WorthUiArtifactNodeKind::Component,
            crate::source::WorthUiArtifactNodeKind::Surface,
            crate::source::WorthUiArtifactNodeKind::Binding,
            crate::source::WorthUiArtifactNodeKind::Token,
        ]
    );
    assert_eq!(canonical_metrics.modules_with_reordered_nodes(), 0);
    assert_eq!(reordered_metrics.modules_with_reordered_nodes(), 1);
}

#[test]
fn artifact_nodes_preserve_identity_seed_and_durable_state_eligibility() {
    let identity_seeded = identity_seeded_from_modules(structureful_component_module());
    let artifact = crate::source::WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact assembly should succeed");

    let seeded_component = identity_seeded
        .module(identity_seeded.module_ids().first().expect("module id"))
        .expect("seeded module")
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiIdentitySeededArtifactInputNode::Component(node) => Some(node),
            _ => None,
        })
        .expect("seeded component");
    let artifact_component = artifact_component_node(&artifact, "workspace.component.dashboard");

    assert_eq!(
        artifact_component.identity_seed(),
        seeded_component.identity_seed()
    );
    assert_eq!(
        artifact_component.durable_state_eligibility(),
        seeded_component.durable_state_eligibility()
    );
}

#[test]
fn artifact_assembly_preserves_query_and_theme_runtime_handles() {
    let (artifact, _) = assembled_artifact_from_modules(imported_modules());
    let surface = artifact_surface_node(&artifact, "workspace.surface.inspector");
    let binding = artifact_binding_node(&artifact, "workspace.view_binding.selection");
    let token = artifact_token_node(&artifact, "theme.text.default");

    assert_eq!(
        binding
            .view_binding_reference()
            .view_binding()
            .id()
            .as_str(),
        "workspace.view_binding.selection"
    );
    assert_eq!(
        token
            .semantics()
            .resolved_target_theme_token()
            .id()
            .as_str(),
        "theme.text.primary"
    );
    assert!(handle_round_trip(&artifact, surface.handle()));
    assert!(handle_round_trip(&artifact, binding.handle()));
    assert!(handle_round_trip(&artifact, token.handle()));
}

#[test]
fn artifact_assembly_preserves_descriptor_and_entry_authority() {
    let identity_seeded = identity_seeded_from_modules(imported_modules());
    let artifact = crate::source::WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact assembly should succeed");
    let module = identity_seeded
        .module(identity_seeded.module_ids().first().expect("module id"))
        .expect("seeded module");
    let seeded_component = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiIdentitySeededArtifactInputNode::Component(node) => Some(node),
            _ => None,
        })
        .expect("seeded component");
    let seeded_surface = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiIdentitySeededArtifactInputNode::Surface(node) => Some(node),
            _ => None,
        })
        .expect("seeded surface");
    let seeded_token = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            crate::source::WorthUiIdentitySeededArtifactInputNode::Token(node) => Some(node),
            _ => None,
        })
        .expect("seeded token");
    let artifact_component = artifact_component_node(&artifact, "workspace.component.dashboard");
    let artifact_surface = artifact_surface_node(&artifact, "workspace.surface.inspector");
    let artifact_token = artifact_token_node(&artifact, "theme.text.default");

    assert_eq!(
        artifact_component.descriptor(),
        seeded_component.bound_node().descriptor()
    );
    assert_eq!(
        artifact_surface.descriptor(),
        seeded_surface.bound_node().descriptor()
    );
    assert_eq!(artifact_token.entry(), seeded_token.bound_node().entry());
}

#[test]
fn artifact_assembly_consumes_proven_inputs_only() {
    let identity_seeded = identity_seeded_from_modules(imported_modules());
    let (artifact, metrics) =
        crate::source::WorthUiCanonicalArtifactAssembler::assemble_with_metrics(&identity_seeded)
            .expect("phase 8 artifact assembly should succeed");

    let component = artifact_component_node(&artifact, "workspace.component.dashboard");
    assert!(handle_round_trip(&artifact, component.handle()));
    assert_eq!(metrics.re_resolved_capability_count(), 0);
    assert_eq!(metrics.rechecked_legality_count(), 0);
}
