use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionContinuationKind, PlanarMotionPosture,
    PlanarMotionPostureContracts, PlanarReorientation,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};

use super::super::bundle_closeout::contract_bundle::readiness_receipt;
use super::super::bundle_closeout::runtime_handles::{
    motion_posture_handle, structural_identity_handle,
};

#[test]
fn kernel_consumes_planar_motion_posture_without_coordinate_or_candidate_synthesis() {
    let readiness = readiness_receipt();
    let motion = PlanarMotionPosture::from_boolean_readiness(readiness.clone())
        .after_exact_translation("motion:kernel-translate")
        .after_exact_rotation("motion:kernel-rotation")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt");

    assert_eq!(
        motion.continuation().kind(),
        PlanarMotionContinuationKind::ExactCancellationReplay
    );
    assert_eq!(motion.counters().motion_step_rows_inspected(), 3);
    assert_eq!(motion.counters().signal_compatibility_rows_inspected(), 1);

    let identity = PlanarStructuralIdentity::from_boolean_readiness(readiness)
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:kernel-motion")
        .with_persistent_name("name:kernel-motion")
        .with_binding_identity("binding:kernel-motion")
        .with_lineage_identity("lineage:kernel-motion")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");

    assert_eq!(
        identity
            .basis()
            .motion_posture_receipt()
            .expect("typed motion receipt")
            .retained_motion_digest(),
        motion.retained_motion_digest()
    );
}
