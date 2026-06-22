use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};

use super::contract_subject::cancellation_motion_receipt;
use super::runtime_handles::structural_identity_handle;

#[test]
fn planar_structural_identity_consumes_typed_motion_posture_receipt() {
    let motion = cancellation_motion_receipt("motion-identity-consumption");
    let receipt = PlanarStructuralIdentity::from_boolean_readiness(
        motion.basis().boolean_readiness_receipt().clone(),
    )
    .with_motion_posture(motion.clone())
    .with_topology_identity("topology:motion")
    .with_persistent_name("name:motion")
    .with_binding_identity("binding:motion")
    .with_lineage_identity("lineage:motion")
    .compile(&PlanarStructuralIdentityContracts::new(
        structural_identity_handle("motion-identity-consumption"),
    ))
    .expect("structural identity plan")
    .certify()
    .expect("structural identity receipt");

    assert_eq!(
        receipt
            .basis()
            .motion_posture_receipt()
            .expect("motion posture receipt")
            .retained_motion_digest(),
        motion.retained_motion_digest()
    );
    assert_ne!(
        receipt.structural_identity_digest(),
        motion.retained_motion_digest()
    );
}
