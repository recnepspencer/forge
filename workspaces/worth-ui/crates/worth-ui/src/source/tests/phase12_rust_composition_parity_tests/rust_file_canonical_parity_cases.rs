use super::rust_composition_fixture_support::{
    artifact_from_composition, artifact_node_count, equivalent_file_artifact,
    equivalent_rust_composition, reordered_rust_composition, semantic_digest, semantic_equivalence,
};

#[test]
fn rust_and_file_authored_equivalent_ui_produce_equivalent_canonical_artifacts() {
    let rust_artifact = artifact_from_composition(&equivalent_rust_composition());
    let file_artifact = equivalent_file_artifact();
    let equivalence = semantic_equivalence(&rust_artifact, &file_artifact);

    assert!(
        equivalence.is_equivalent(),
        "first difference: {:?}",
        equivalence.first_difference()
    );
    assert_eq!(
        semantic_digest(&rust_artifact),
        semantic_digest(&file_artifact)
    );
    assert_eq!(
        artifact_node_count(&rust_artifact),
        artifact_node_count(&file_artifact)
    );
}

#[test]
fn rust_composition_reordering_preserves_canonical_artifact_digest() {
    let baseline = artifact_from_composition(&equivalent_rust_composition());
    let reordered = artifact_from_composition(&reordered_rust_composition());
    let equivalence = semantic_equivalence(&baseline, &reordered);

    assert!(
        equivalence.is_equivalent(),
        "first difference: {:?}",
        equivalence.first_difference()
    );
    assert_eq!(semantic_digest(&baseline), semantic_digest(&reordered));
}
