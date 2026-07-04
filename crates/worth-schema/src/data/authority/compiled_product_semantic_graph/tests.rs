use super::{
    admit_compiled_product_authority_truth_identity,
    admit_compiled_product_authority_truth_identity_with_coordinates,
    admit_compiled_product_equivalence_policy_identity, admit_compiled_product_identity,
    admit_compiled_product_prior_proof_identity, admit_compiled_product_rebuild_denial_identity,
    admit_compiled_product_reuse_decision_identity, CompiledProductAuthorityInstanceCoordinate,
    CompiledProductLocalityFootprintIdentity, CompiledProductPriorProofRole,
    CompiledProductSemanticGraphVocabularyErrorKind,
};

#[test]
fn compiled_product_identity_and_policy_are_rerun_stable() {
    let authority_truth = admit_compiled_product_authority_truth_identity(
        "worth-topo",
        "authority-truth-digest",
        "derived-topology-truth",
    )
    .expect("authority truth");
    let locality = CompiledProductLocalityFootprintIdentity::touched_closure("closure-digest")
        .expect("locality");
    let prior_proof = admit_compiled_product_prior_proof_identity(
        "prior-proof-digest",
        CompiledProductPriorProofRole::ProductShapingBasis,
    )
    .expect("prior proof");

    let first = admit_compiled_product_identity(
        authority_truth.clone(),
        locality.clone(),
        Some(prior_proof.clone()),
        None,
    );
    let second =
        admit_compiled_product_identity(authority_truth, locality, Some(prior_proof), None);

    let canonical = admit_compiled_product_equivalence_policy_identity(
        "topology-derived-equivalence",
        [
            "derived-validation",
            "materialized-topology",
            "interpreted-topology",
        ],
    )
    .expect("policy");
    let reordered = admit_compiled_product_equivalence_policy_identity(
        "topology-derived-equivalence",
        [
            "interpreted-topology",
            "materialized-topology",
            "derived-validation",
        ],
    )
    .expect("reordered policy");

    assert_eq!(first.identity_digest(), second.identity_digest());
    assert_eq!(canonical.identity_digest(), reordered.identity_digest());
}

#[test]
fn authority_instance_coordinates_are_canonicalized_and_deduplicated() {
    let canonical = admit_compiled_product_authority_truth_identity_with_coordinates(
        "worth-topo",
        "authority-truth-digest",
        "derived-topology-truth",
        [
            CompiledProductAuthorityInstanceCoordinate::branch_identity("branch-a")
                .expect("branch coordinate"),
            CompiledProductAuthorityInstanceCoordinate::snapshot_identity("7")
                .expect("snapshot coordinate"),
        ],
    )
    .expect("canonical truth identity");
    let reordered_with_duplicates =
        admit_compiled_product_authority_truth_identity_with_coordinates(
            "worth-topo",
            "authority-truth-digest",
            "derived-topology-truth",
            [
                CompiledProductAuthorityInstanceCoordinate::snapshot_identity("7")
                    .expect("snapshot coordinate"),
                CompiledProductAuthorityInstanceCoordinate::branch_identity("branch-a")
                    .expect("branch coordinate"),
                CompiledProductAuthorityInstanceCoordinate::branch_identity("branch-a")
                    .expect("duplicate branch coordinate"),
            ],
        )
        .expect("reordered truth identity");

    assert_eq!(
        canonical.identity_digest(),
        reordered_with_duplicates.identity_digest()
    );
    assert_eq!(
        canonical.authority_instance_coordinates(),
        reordered_with_duplicates.authority_instance_coordinates()
    );
}

#[test]
fn malformed_authority_instance_coordinates_deny_at_the_schema_boundary() {
    let error = CompiledProductAuthorityInstanceCoordinate::named("", "branch-a")
        .expect_err("blank coordinate kind must deny");
    assert_eq!(
        error.kind(),
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityInstanceKind
    );

    let error = CompiledProductAuthorityInstanceCoordinate::named("branch-identity", "")
        .expect_err("blank coordinate value must deny");
    assert_eq!(
        error.kind(),
        CompiledProductSemanticGraphVocabularyErrorKind::EmptyAuthorityInstanceValue
    );
}
