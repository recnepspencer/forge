use crate::source::{
    WorthUiArtifactDifference, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiArtifactEquivalenceComparator, WorthUiArtifactNode, WorthUiArtifactNodeKind,
};

use super::digest_fixture_support::{
    artifact_from_rust_modules, artifact_from_rust_modules_with_app,
    component_descriptor_variant_app, imported_modules, surface_descriptor_variant_app,
    theme_token_alias_chain_variant_app,
};

#[test]
fn component_descriptor_change_changes_digest_for_same_authored_input() {
    let left = artifact_from_rust_modules(imported_modules());
    let right =
        artifact_from_rust_modules_with_app(imported_modules(), component_descriptor_variant_app());
    let left_digest = digest(&left);
    let right_digest = digest(&right);

    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_ne!(left_digest, right_digest);
    assert!(!equivalence.is_equivalent());
    assert!(matches!(
        equivalence.first_difference(),
        Some(WorthUiArtifactDifference::NodeSemanticMismatch {
            node_kind: WorthUiArtifactNodeKind::Component,
            ..
        })
    ));
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}

#[test]
fn surface_descriptor_change_changes_digest_for_same_authored_input() {
    let left = artifact_from_rust_modules(imported_modules());
    let right =
        artifact_from_rust_modules_with_app(imported_modules(), surface_descriptor_variant_app());
    let left_digest = digest(&left);
    let right_digest = digest(&right);

    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_ne!(left_digest, right_digest);
    assert!(!equivalence.is_equivalent());
    assert!(matches!(
        equivalence.first_difference(),
        Some(WorthUiArtifactDifference::NodeSemanticMismatch {
            node_kind: WorthUiArtifactNodeKind::Surface,
            ..
        })
    ));
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}

#[test]
fn token_entry_change_changes_digest_when_resolved_target_stays_the_same() {
    let left = artifact_from_rust_modules(imported_modules());
    let right = artifact_from_rust_modules_with_app(
        imported_modules(),
        theme_token_alias_chain_variant_app(),
    );
    let left_digest = digest(&left);
    let right_digest = digest(&right);
    let left_token = artifact_token_node(&left, "theme.text.default");
    let right_token = artifact_token_node(&right, "theme.text.default");

    let equivalence = WorthUiArtifactEquivalenceComparator::compare(
        &left,
        &right,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );

    assert_eq!(
        left_token
            .semantics()
            .resolved_target_theme_token()
            .id()
            .as_str(),
        right_token
            .semantics()
            .resolved_target_theme_token()
            .id()
            .as_str()
    );
    assert_eq!(
        left_token
            .semantics()
            .resolved_target_entry()
            .resolved_target_id(),
        right_token
            .semantics()
            .resolved_target_entry()
            .resolved_target_id()
    );
    assert_ne!(
        left_token.entry().key().projection_basis(),
        right_token.entry().key().projection_basis()
    );
    assert_ne!(left_digest, right_digest);
    assert!(!equivalence.is_equivalent());
    assert!(matches!(
        equivalence.first_difference(),
        Some(WorthUiArtifactDifference::NodeSemanticMismatch {
            node_kind: WorthUiArtifactNodeKind::Token,
            ..
        })
    ));
    assert_eq!(equivalence.metrics().broad_scans(), 0);
}

fn digest(artifact: &crate::source::WorthUiArtifact) -> crate::source::WorthUiArtifactDigest {
    WorthUiArtifactDigestor::digest(artifact, WorthUiArtifactEquivalenceBasis::semantic())
}

fn artifact_token_node<'a>(
    artifact: &'a crate::source::WorthUiArtifact,
    token_id: &str,
) -> &'a crate::source::WorthUiArtifactThemeTokenNode {
    artifact
        .module_ids()
        .iter()
        .filter_map(|module_id| artifact.module(module_id))
        .flat_map(|module| module.nodes().iter())
        .find_map(|node| match node {
            WorthUiArtifactNode::Token(node) if node.theme_token().id().as_str() == token_id => {
                Some(node)
            }
            _ => None,
        })
        .expect("artifact theme token node should exist")
}
