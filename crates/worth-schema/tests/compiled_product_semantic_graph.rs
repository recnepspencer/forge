use schema::facade::platform::authority::compiled_product_semantic_graph::{
    admit_compiled_product_authority_truth_identity,
    admit_compiled_product_authority_truth_identity_with_coordinates,
    admit_compiled_product_equivalence_policy_identity, admit_compiled_product_identity,
    admit_compiled_product_prior_proof_identity, admit_compiled_product_rebuild_denial_identity,
    admit_compiled_product_reuse_decision_identity, CompiledProductAuthorityInstanceCoordinate,
    CompiledProductLocalityFootprintIdentity, CompiledProductPriorProofRole,
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
fn wrong_authority_owner_cannot_impersonate_authority_truth_identity() {
    let topology_truth = admit_compiled_product_authority_truth_identity(
        "worth-topo",
        "shared-authority-digest",
        "derived-topology-truth",
    )
    .expect("topology truth");
    let spatial_truth = admit_compiled_product_authority_truth_identity(
        "worth-spatial",
        "shared-authority-digest",
        "derived-topology-truth",
    )
    .expect("spatial truth");
    let locality = CompiledProductLocalityFootprintIdentity::invalidation_closure("closure-digest")
        .expect("locality");

    let topology_product =
        admit_compiled_product_identity(topology_truth, locality.clone(), None, None);
    let spatial_product = admit_compiled_product_identity(spatial_truth, locality, None, None);

    assert_ne!(
        topology_product
            .authority_truth_identity()
            .identity_digest(),
        spatial_product.authority_truth_identity().identity_digest()
    );
    assert_ne!(
        topology_product.identity_digest(),
        spatial_product.identity_digest()
    );
}

#[test]
fn wrong_authority_instance_cannot_impersonate_authority_truth_identity() {
    let branch_a = admit_compiled_product_authority_truth_identity_with_coordinates(
        "worth-topo",
        "shared-authority-digest",
        "derived-topology-truth",
        [
            CompiledProductAuthorityInstanceCoordinate::snapshot_identity("7")
                .expect("snapshot coordinate"),
            CompiledProductAuthorityInstanceCoordinate::branch_identity("branch-a")
                .expect("branch coordinate"),
        ],
    )
    .expect("branch a truth");
    let branch_b = admit_compiled_product_authority_truth_identity_with_coordinates(
        "worth-topo",
        "shared-authority-digest",
        "derived-topology-truth",
        [
            CompiledProductAuthorityInstanceCoordinate::snapshot_identity("7")
                .expect("snapshot coordinate"),
            CompiledProductAuthorityInstanceCoordinate::branch_identity("branch-b")
                .expect("branch coordinate"),
        ],
    )
    .expect("branch b truth");
    let locality = CompiledProductLocalityFootprintIdentity::invalidation_closure("closure-digest")
        .expect("locality");

    let product_a = admit_compiled_product_identity(branch_a, locality.clone(), None, None);
    let product_b = admit_compiled_product_identity(branch_b, locality, None, None);

    assert_ne!(
        product_a.authority_truth_identity().identity_digest(),
        product_b.authority_truth_identity().identity_digest()
    );
    assert_ne!(product_a.identity_digest(), product_b.identity_digest());
}

#[test]
fn rendered_payload_strings_do_not_mint_compiled_product_identity() {
    let authority_truth = admit_compiled_product_authority_truth_identity(
        "worth-spatial",
        "authoritative-evidence-digest",
        "retained-planar-historical-inspection",
    )
    .expect("authority truth");
    let locality = CompiledProductLocalityFootprintIdentity::materialization_target_footprint(
        "projection-digest",
    )
    .expect("locality");

    let first =
        admit_compiled_product_identity(authority_truth.clone(), locality.clone(), None, None);
    let second = admit_compiled_product_identity(authority_truth, locality, None, None);
    let policy = admit_compiled_product_equivalence_policy_identity(
        "retained-replay-parity",
        ["retained-planar-facts", "projection-consumed-facts"],
    )
    .expect("policy");
    let reuse = admit_compiled_product_reuse_decision_identity(&first, &policy, "parity-admitted")
        .expect("reuse identity");
    let denial =
        admit_compiled_product_rebuild_denial_identity(&second, "rendered-payload-mismatch")
            .expect("rebuild denial");

    assert_eq!(first.identity_digest(), second.identity_digest());
    assert_ne!(reuse.identity_digest(), denial.identity_digest());
}
