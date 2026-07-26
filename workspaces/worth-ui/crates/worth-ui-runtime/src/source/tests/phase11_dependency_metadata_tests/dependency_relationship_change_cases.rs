use std::collections::BTreeMap;
use std::path::Path;
use worth_ui_dsl::WorthUiSourceModuleId;

use crate::source::{
    WorthUiArtifactComponentHandle, WorthUiArtifactDependencyEdge,
    WorthUiArtifactDependencyEdgeKind, WorthUiArtifactDependencyGraph,
    WorthUiArtifactDependencyTarget, WorthUiArtifactHandle, WorthUiArtifactSubtreeDigest,
    WorthUiArtifactSurfaceHandle,
};

use super::dependency_fixture_support::{
    dependency_basis_from_artifact, import_removed_artifact, imported_artifact,
    primary_only_mount_artifact, standard_mount_artifact, surface_handle,
};

#[test]
fn dependency_metadata_changes_when_meaningful_upstream_relationships_change() {
    let with_import = dependency_basis_from_artifact(&imported_artifact());
    let without_import = dependency_basis_from_artifact(&import_removed_artifact());

    assert_ne!(
        with_import.dependency_graph().module_dependencies(),
        without_import.dependency_graph().module_dependencies()
    );
    assert_ne!(with_import, without_import);
}

#[test]
fn module_import_edges_name_source_handle_and_target_module() {
    let basis = dependency_basis_from_artifact(&imported_artifact());
    let import_edge = basis
        .dependency_graph()
        .edges()
        .iter()
        .find(|edge| edge.kind() == WorthUiArtifactDependencyEdgeKind::ModuleImport)
        .expect("module import dependency edge");

    assert_eq!(import_edge.source().module_id().as_str(), "app/main.wui");
    assert_eq!(
        import_edge.target(),
        &WorthUiArtifactDependencyTarget::Module(
            WorthUiSourceModuleId::from_relative_path(std::path::Path::new(
                "app/panels/inspector.wui"
            ))
            .unwrap()
        )
    );
}

#[test]
fn mosaic_mount_changes_are_explicit_dependency_metadata_changes() {
    let standard_artifact = standard_mount_artifact();
    let primary_only_artifact = primary_only_mount_artifact();
    let standard = dependency_basis_from_artifact(&standard_artifact);
    let primary_only = dependency_basis_from_artifact(&primary_only_artifact);
    let overlay_surface = surface_handle(&standard_artifact, "workspace.surface.overlay");

    let overlay_mount_edges = standard
        .dependency_graph()
        .edges()
        .iter()
        .filter(|edge| {
            edge.kind() == WorthUiArtifactDependencyEdgeKind::MosaicMount
                && edge.target()
                    == &WorthUiArtifactDependencyTarget::Artifact(overlay_surface.clone())
        })
        .count();

    assert_eq!(overlay_mount_edges, 1);
    assert_ne!(
        standard.dependency_graph().edges(),
        primary_only.dependency_graph().edges()
    );
    assert_ne!(
        standard
            .impact_metadata()
            .impact_for_subtree(&overlay_surface),
        primary_only
            .impact_metadata()
            .impact_for_subtree(&overlay_surface)
    );
}

#[test]
fn dependency_graph_constructor_canonicalizes_boundary_vectors() {
    let source_module = module_id("app/main.wui");
    let target_module = module_id("app/panels/inspector.wui");
    let component = component_handle(source_module.clone(), 0);
    let surface = surface_handle_for_module(source_module.clone(), 1);
    let edge = WorthUiArtifactDependencyEdge::new(
        component.clone(),
        WorthUiArtifactDependencyTarget::Artifact(surface.clone()),
        WorthUiArtifactDependencyEdgeKind::MosaicMount,
    );
    let mut module_dependencies = BTreeMap::new();
    module_dependencies.insert(
        source_module,
        vec![target_module.clone(), target_module.clone()],
    );

    let graph = WorthUiArtifactDependencyGraph::new(
        vec![edge.clone(), edge.clone()],
        module_dependencies,
        BTreeMap::from([(component, WorthUiArtifactSubtreeDigest::new(42))]),
        BTreeMap::new(),
    );

    assert_eq!(graph.edges(), &[edge]);
    assert_eq!(
        graph.module_dependencies().values().next().unwrap(),
        &vec![target_module]
    );
}

fn module_id(relative_path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(relative_path)).unwrap()
}

fn component_handle(module_id: WorthUiSourceModuleId, node_index: usize) -> WorthUiArtifactHandle {
    WorthUiArtifactHandle::Component(WorthUiArtifactComponentHandle::new(module_id, node_index))
}

fn surface_handle_for_module(
    module_id: WorthUiSourceModuleId,
    node_index: usize,
) -> WorthUiArtifactHandle {
    WorthUiArtifactHandle::Surface(WorthUiArtifactSurfaceHandle::new(module_id, node_index))
}
