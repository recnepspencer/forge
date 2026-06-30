use super::dependency_fixture_support::{
    binding_handle, dependency_basis_from_artifact, imported_artifact, inspector_module_id,
};

#[test]
fn dependency_narrowing_does_not_require_full_tree_scan() {
    let artifact = imported_artifact();
    let basis = dependency_basis_from_artifact(&artifact);

    let impact = basis
        .impact_metadata()
        .impact_for_module(&inspector_module_id());

    assert!(impact.requires_less_than_full_artifact_scan());
    assert!(!impact.impacted_handles().is_empty());
    assert!(impact.impacted_handles().len() < artifact_handle_count(&artifact));
    assert_eq!(impact.lookup_count(), 1);
    assert_eq!(
        impact.full_artifact_handle_count(),
        artifact_handle_count(&artifact)
    );
}

#[test]
fn subtree_impact_lookup_uses_canonical_handle_index() {
    let artifact = imported_artifact();
    let basis = dependency_basis_from_artifact(&artifact);
    let binding = binding_handle(&artifact, "workspace.view_binding.selection");

    let digest = basis
        .dependency_graph()
        .subtree_digest(&binding)
        .expect("binding subtree digest");
    let impact = basis.impact_metadata().impact_for_subtree(&binding);

    assert_ne!(digest.raw(), 0);
    assert_eq!(impact.impacted_handles(), &[binding]);
    assert!(impact.requires_less_than_full_artifact_scan());
    assert_eq!(impact.lookup_count(), 1);
}

fn artifact_handle_count(artifact: &crate::source::WorthUiArtifact) -> usize {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .map(|module| module.nodes().len())
        .sum()
}
