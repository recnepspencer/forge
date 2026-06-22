use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionPosture, PlanarMotionPostureContracts, PlanarMotionPostureDenialKind,
    PlanarReorientation,
};

use super::contract_subject::boolean_readiness_receipt;
use super::runtime_handles::motion_posture_handle;

#[test]
fn planar_motion_posture_denies_orientation_flip_before_projection_consumption() {
    let denial = match PlanarMotionPosture::from_boolean_readiness(boolean_readiness_receipt(
        "motion-orientation-flip",
    ))
    .after_exact_translation("motion:translate")
    .after_reorientation(PlanarReorientation::ReversesHandedness)
    .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
        "motion-orientation-flip",
    ))) {
        Ok(_) => panic!("orientation flip must deny before retained projection consumption"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarMotionPostureDenialKind::OrientationFlipInvalidatesPlanarBasis
    );
    assert_eq!(denial.counters().rejected_orientation_flip_rows(), 1);
}

#[test]
fn planar_motion_posture_rejects_coordinate_only_reconstruction() {
    let denial = match PlanarMotionPosture::from_boolean_readiness(boolean_readiness_receipt(
        "motion-coordinate-denial",
    ))
    .after_exact_translation("motion:translate")
    .with_final_coordinate_digest_only("coordinate:digest")
    .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
        "motion-coordinate-denial",
    ))) {
        Ok(_) => panic!("coordinate-only motion reconstruction must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        PlanarMotionPostureDenialKind::CoordinateOnlyMotionBasis
    );
    assert_eq!(denial.counters().rejected_coordinate_only_rows(), 1);
}
