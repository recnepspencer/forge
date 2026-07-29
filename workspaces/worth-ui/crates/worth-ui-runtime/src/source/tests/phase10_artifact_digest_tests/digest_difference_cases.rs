use crate::source::{
    WorthUiArtifactDifference, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactEquivalenceComparator,
};

use super::digest_fixture_support::{
    artifact_from_rust_modules, imported_modules, same_shape_but_different_surface_modules,
    token_difference_modules,
};

#[test]
fn meaningful_artifact_difference_changes_digest() {
    let left = artifact_from_rust_modules(imported_modules());
    let right = artifact_from_rust_modules(same_shape_but_different_surface_modules());

    let left_digest =
        WorthUiArtifactDigestor::digest(&left, WorthUiArtifactEquivalenceBasis::semantic());
    let right_digest =
        WorthUiArtifactDigestor::digest(&right, WorthUiArtifactEquivalenceBasis::semantic());
    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_ne!(left_digest, right_digest);
    assert!(!equivalence.is_equivalent());
    assert!(equivalence.differences().iter().any(|difference| matches!(
        difference,
        WorthUiArtifactDifference::NodeRetired { node_identity, .. }
            if node_identity.contains("workspace.surface.inspector")
    )));
    assert!(equivalence.differences().iter().any(|difference| matches!(
        difference,
        WorthUiArtifactDifference::NodeCreated { node_identity, .. }
            if node_identity.contains("workspace.surface.main")
    )));
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}

#[test]
fn semantically_different_token_artifact_breaks_equivalence() {
    let left = artifact_from_rust_modules(imported_modules());
    let right = artifact_from_rust_modules(token_difference_modules());

    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert!(!equivalence.is_equivalent());
    assert!(equivalence.differences().iter().any(|difference| matches!(
        difference,
        WorthUiArtifactDifference::NodeRetired { node_identity, .. }
            if node_identity.contains("theme.text.default")
    )));
    assert!(equivalence.differences().iter().any(|difference| matches!(
        difference,
        WorthUiArtifactDifference::NodeCreated { node_identity, .. }
            if node_identity.contains("theme.text.primary")
    )));
}
