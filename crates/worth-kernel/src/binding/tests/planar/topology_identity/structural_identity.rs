use worth_spatial::facade::planar_structural_identity::{
    CanonicalPlanarTransformBasis, PlanarOrientationPolicy, PlanarStructuralIdentity,
    PlanarStructuralIdentityContracts,
};

use super::super::bundle_closeout::contract_bundle::{readiness_receipt, MOVEMENT};
use super::super::bundle_closeout::runtime_handles::structural_identity_handle;

#[test]
fn kernel_consumes_planar_structural_identity_without_topology_name_or_binding_synthesis() {
    let readiness = readiness_receipt();
    let transform = CanonicalPlanarTransformBasis::builder()
        .local_frame("frame:kernel-bundle")
        .movement_rotation_posture(MOVEMENT)
        .transform_chain_digest("transform:kernel-bundle")
        .orientation_policy(PlanarOrientationPolicy::Preserve)
        .build()
        .expect("canonical transform basis");

    let receipt = PlanarStructuralIdentity::from_boolean_readiness(readiness)
        .with_topology_identity("topology:stable-kernel")
        .with_persistent_name("name:stable-kernel")
        .with_binding_identity("binding:stable-kernel")
        .with_lineage_identity("lineage:stable-kernel")
        .with_canonical_transform_basis(transform)
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");

    assert_ne!(
        receipt.structural_identity_digest(),
        receipt.binding_identity()
    );
    assert_eq!(receipt.counters().structural_basis_rows_inspected(), 1);
    assert_eq!(receipt.counters().transform_basis_rows_inspected(), 4);
    assert_eq!(receipt.counters().contrast_identity_rows_inspected(), 4);
}
