use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureDenialKind,
};

use super::contract_subject::boolean_readiness_receipt;
use super::runtime_handles::motion_posture_handle;

#[test]
fn planar_motion_posture_denies_exact_cancellation_without_rotation_basis() {
    let denial = match PlanarMotionPosture::from_boolean_readiness(boolean_readiness_receipt(
        "motion-cancellation-without-rotation",
    ))
    .after_exact_translation("motion:translation-only")
    .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
    .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
        "motion-cancellation-without-rotation",
    ))) {
        Ok(_) => panic!("exact cancellation must not compile without an exact rotation basis"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarMotionPostureDenialKind::ExactCancellationMissingRotation
    );
}
