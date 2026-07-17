use std::path::Path;

use crate::source::{
    WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactNode, WorthUiBindingSemanticsLowerer,
    WorthUiCanonicalArtifactAssembler, WorthUiIncrementalInvalidationBasis,
    WorthUiRuntimeQuerySurface, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiRustAuthoredToArtifactInputLowerer,
    WorthUiSourceModuleId, WorthUiStructuralLegalityLowerer,
};

use super::super::phase7_identity_seeding_tests::identity_app_fixture::identity_test_app;
use super::super::phase7_identity_seeding_tests::identity_fixture_support::{
    imported_identity_modules, primary_only_component_body_atoms,
    reordered_imported_identity_modules, standard_component_body_atoms,
};

pub(super) fn dependency_basis_from_imported_modules() -> WorthUiIncrementalInvalidationBasis {
    dependency_basis_from_artifact(&artifact_from_rust_modules(imported_modules()))
}

pub(super) fn dependency_basis_from_reordered_modules() -> WorthUiIncrementalInvalidationBasis {
    dependency_basis_from_artifact(&artifact_from_rust_modules(reordered_modules()))
}

pub(super) fn dependency_basis_from_artifact(
    artifact: &WorthUiArtifact,
) -> WorthUiIncrementalInvalidationBasis {
    crate::source::WorthUiArtifactDependencyDeriver::derive(artifact)
}

pub(super) fn imported_artifact() -> WorthUiArtifact {
    artifact_from_rust_modules(imported_modules())
}

pub(super) fn import_removed_artifact() -> WorthUiArtifact {
    artifact_from_rust_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("workspace.component.dashboard")
            .with_surface("workspace.surface.inspector")
            .with_binding("workspace.view_binding.selection")
            .with_token("theme.text.default", "theme.text.primary"),
        WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
            .with_component("workspace.component.inspector_panel"),
    ])
}

pub(super) fn standard_mount_artifact() -> WorthUiArtifact {
    artifact_from_rust_modules([mounted_component_module(standard_component_body_atoms())])
}

pub(super) fn primary_only_mount_artifact() -> WorthUiArtifact {
    artifact_from_rust_modules([mounted_component_module(primary_only_component_body_atoms())])
}

fn artifact_from_rust_modules<const N: usize>(
    modules: [WorthUiRustAuthoredArtifactInputModule; N],
) -> WorthUiArtifact {
    let app = identity_test_app();
    let snapshot = app.capabilities();
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules(modules),
    );
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("phase 4 resolution should succeed");
    let structured = WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot)
        .expect("phase 5 legality should succeed");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("phase 6 binding should succeed");
    let identity_seeded = crate::source::WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("phase 7 identity should succeed")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("phase 8 artifact should succeed")
}

fn imported_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    imported_identity_modules()
}

fn reordered_modules() -> [WorthUiRustAuthoredArtifactInputModule; 2] {
    reordered_imported_identity_modules()
}

fn mounted_component_module(
    body_atoms: Vec<crate::source::WorthUiArtifactInputBodyAtom>,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms("workspace.component.dashboard", body_atoms)
        .with_surface("workspace.surface.main")
        .with_surface("workspace.surface.overlay")
}

pub(super) fn inspector_module_id() -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new("app/panels/inspector.wui"))
        .expect("valid inspector module id")
}

pub(super) fn surface_handle(
    artifact: &WorthUiArtifact,
    surface_id: &str,
) -> WorthUiArtifactHandle {
    artifact_handle(artifact, |node| match node {
        WorthUiArtifactNode::Surface(surface) if surface.surface().id().as_str() == surface_id => {
            Some(surface.handle().clone())
        }
        _ => None,
    })
}

pub(super) fn binding_handle(
    artifact: &WorthUiArtifact,
    binding_id: &str,
) -> WorthUiArtifactHandle {
    artifact_handle(artifact, |node| match node {
        WorthUiArtifactNode::Binding(binding)
            if binding
                .view_binding_reference()
                .view_binding()
                .id()
                .as_str()
                == binding_id =>
        {
            Some(binding.handle().clone())
        }
        _ => None,
    })
}

pub(super) fn token_handle(artifact: &WorthUiArtifact, token_id: &str) -> WorthUiArtifactHandle {
    artifact_handle(artifact, |node| match node {
        WorthUiArtifactNode::Token(token) if token.theme_token().id().as_str() == token_id => {
            Some(token.handle().clone())
        }
        _ => None,
    })
}

pub(super) fn assert_hook_surface(
    basis: &WorthUiIncrementalInvalidationBasis,
    handle: &WorthUiArtifactHandle,
    surface: WorthUiRuntimeQuerySurface,
) {
    assert!(
        basis
            .dependency_graph()
            .runtime_hooks_for(handle)
            .iter()
            .any(|hook| hook.uses_query_surface(surface)),
        "expected runtime hook for {surface:?}"
    );
}

fn artifact_handle(
    artifact: &WorthUiArtifact,
    predicate: impl FnMut(&WorthUiArtifactNode) -> Option<WorthUiArtifactHandle>,
) -> WorthUiArtifactHandle {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(predicate)
        .expect("artifact handle should exist")
}
