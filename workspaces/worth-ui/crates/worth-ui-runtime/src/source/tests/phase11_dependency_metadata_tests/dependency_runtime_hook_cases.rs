use crate::{
    capability::QueryDenialPresentation,
    source::{
        WorthUiArtifactDependencyDeriver, WorthUiRuntimeDependencyHookKind,
        WorthUiRuntimeQuerySurface,
    },
};

use super::dependency_fixture_support::{
    assert_hook_surface, binding_handle, dependency_basis_from_artifact, imported_artifact,
    surface_handle, token_handle,
};

#[test]
fn dependency_metadata_preserves_runtime_graph_hooks() {
    let artifact = imported_artifact();
    let basis = dependency_basis_from_artifact(&artifact);
    let binding = binding_handle(&artifact, "workspace.view_binding.selection");
    let surface = surface_handle(&artifact, "workspace.surface.inspector");

    assert_hook_surface(&basis, &binding, WorthUiRuntimeQuerySurface::LiveView);
    assert_hook_surface(
        &basis,
        &binding,
        WorthUiRuntimeQuerySurface::RegionScopedLiveInvalidation,
    );
    assert_hook_surface(
        &basis,
        &binding,
        WorthUiRuntimeQuerySurface::SignalCompatibilityAndContinuation,
    );
    assert_hook_surface(
        &basis,
        &binding,
        WorthUiRuntimeQuerySurface::AsyncResourcesAndResultState,
    );
    assert_hook_surface(&basis, &surface, WorthUiRuntimeQuerySurface::LiveView);

    let hook = basis
        .dependency_graph()
        .runtime_hooks_for(&binding)
        .first()
        .expect("binding runtime hook");
    assert_eq!(hook.kind(), WorthUiRuntimeDependencyHookKind::QueryLiveView);
    assert_eq!(
        hook.view_binding_id().as_str(),
        "workspace.view_binding.selection"
    );
    assert!(hook.query_capability().is_admitted());
    assert!(hook.basis_posture().is_admitted());
    assert!(hook.live_compatibility().is_admitted());
    assert_eq!(
        hook.denial_presentation(),
        &QueryDenialPresentation::structured_status()
    );
    assert_eq!(
        hook.view_shape().family(),
        worth_query::facade::ViewShapeFamily::Table
    );
    assert!(hook
        .result_shape()
        .family()
        .eq(&worth_query::facade::ResultShapeFamily::Collection));
}

#[test]
fn dependency_metadata_does_not_invent_query_hooks_for_non_query_nodes() {
    let artifact = imported_artifact();
    let basis = dependency_basis_from_artifact(&artifact);
    let token = token_handle(&artifact, "theme.text.default");

    assert!(basis
        .dependency_graph()
        .runtime_hooks_for(&token)
        .is_empty());
}

#[test]
fn dependency_report_metrics_explain_derivation_work() {
    let artifact = imported_artifact();
    let report = WorthUiArtifactDependencyDeriver::derive_with_report(&artifact);
    let metrics = report.metrics();

    assert_eq!(metrics.nodes_indexed(), 6);
    assert_eq!(metrics.dependency_edges_recorded(), 9);
    assert_eq!(metrics.subtree_digests_recorded(), 6);
    assert_eq!(metrics.runtime_hooks_recorded(), 8);
}
