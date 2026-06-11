use super::contract_subject::{
    boolean_readiness_receipt, bundle_transform_basis, structural_identity_receipt,
};
use super::runtime_handles::structural_identity_handle;

#[test]
fn planar_structural_identity_converges_for_canonical_transform_equivalence() {
    let first = structural_identity_receipt("structural-equivalence", "topology:first");
    let second = structural_identity_receipt("structural-equivalence", "topology:renamed");

    assert_ne!(
        first.basis().topology_identity(),
        second.basis().topology_identity()
    );
    assert_eq!(
        first.canonical_transform_basis_digest(),
        second.canonical_transform_basis_digest()
    );
    assert_eq!(first.declaration_digest(), second.declaration_digest());
    assert_eq!(first.envelope_digest(), second.envelope_digest());
    assert_eq!(
        first.structural_identity_digest(),
        second.structural_identity_digest()
    );
}

#[test]
fn planar_structural_identity_rejects_coordinate_only_identity_basis() {
    let readiness = boolean_readiness_receipt("structural-coordinate-denial");
    let denial =
        worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityBasis::builder()
            .boolean_readiness_receipt(readiness)
            .canonical_transform_basis(bundle_transform_basis())
            .topology_identity("topology:stable")
            .persistent_name("name:stable")
            .binding_identity("binding:stable")
            .lineage_identity("lineage:stable")
            .final_coordinate_digest_only("coordinate:digest")
            .build()
            .expect_err("coordinate-only identity basis must deny");
    assert_eq!(
        denial.kind(),
        worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityDenialKind::CoordinateOnlyIdentityBasis
    );
    assert_eq!(denial.counters().rejected_coordinate_only_rows(), 1);
}

#[test]
fn planar_structural_identity_rejects_non_equivalent_transform_history() {
    let readiness = boolean_readiness_receipt("structural-transform-denial");
    let transform = worth_spatial::facade::planar_structural_identity::CanonicalPlanarTransformBasis::builder()
        .local_frame("frame:bundle")
        .movement_rotation_posture(crate::public_api_planar_contract_bundle::proof_fixture::MOVEMENT)
        .transform_chain_digest("transform:close-coordinate-but-different-history")
        .orientation_policy(worth_spatial::facade::planar_structural_identity::PlanarOrientationPolicy::Preserve)
        .build()
        .expect("transform basis");

    let denial = match worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentity::from_boolean_readiness(readiness)
        .with_topology_identity("topology:stable")
        .with_persistent_name("name:stable")
        .with_binding_identity("binding:stable")
        .with_lineage_identity("lineage:stable")
        .with_canonical_transform_basis(transform)
        .compile(&worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityContracts::new(
            structural_identity_handle("structural-transform-denial"),
        ))
    {
        Ok(_) => panic!("non-equivalent transform history must deny before certification"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        worth_spatial::facade::planar_structural_identity::PlanarStructuralIdentityDenialKind::BundleTransformMismatch
    );
}
