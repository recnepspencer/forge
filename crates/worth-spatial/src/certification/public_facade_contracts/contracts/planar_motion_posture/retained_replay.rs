use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionContinuationKind, PlanarMotionPosture,
    PlanarMotionPostureContracts, PlanarMotionPostureReceipt, PlanarReorientation,
};

use super::contract_subject::{boolean_readiness_receipt, cancellation_motion_receipt};
use super::runtime_handles::motion_posture_handle;

#[test]
fn planar_motion_posture_preserves_translation_rotation_reorientation_and_cancellation() {
    let checkpoint = cancellation_motion_receipt("motion-cancellation");
    let replay = cancellation_motion_receipt("motion-cancellation");

    assert_eq!(checkpoint.declaration_digest(), replay.declaration_digest());
    assert_eq!(checkpoint.envelope_digest(), replay.envelope_digest());
    assert_eq!(
        checkpoint.retained_motion_digest(),
        replay.retained_motion_digest()
    );
    assert_eq!(
        checkpoint.continuation().kind(),
        PlanarMotionContinuationKind::ExactCancellationReplay
    );
    assert_eq!(checkpoint.counters().motion_step_rows_inspected(), 4);
    assert_eq!(checkpoint.counters().cancellation_rows_inspected(), 1);
    assert_eq!(
        checkpoint.counters().signal_compatibility_rows_inspected(),
        1
    );
}

#[test]
fn planar_motion_posture_canonicalizes_exact_cancellation_regrouping() {
    let first_authoring_order =
        exact_cancellation_motion_receipt("motion-canonical-cancellation-first", |motion| {
            motion
                .after_exact_translation("motion:translate-out")
                .after_exact_rotation("motion:quarter-turn")
                .after_exact_rotation("motion:quarter-turn-inverse")
                .after_reorientation(PlanarReorientation::PreservesHandedness)
        });
    let regrouped_authoring_order =
        exact_cancellation_motion_receipt("motion-canonical-cancellation-first", |motion| {
            motion
                .after_exact_rotation("motion:quarter-turn-inverse")
                .after_reorientation(PlanarReorientation::PreservesHandedness)
                .after_exact_translation("motion:translate-out")
                .after_exact_rotation("motion:quarter-turn")
        });

    assert_eq!(
        first_authoring_order.declaration_digest(),
        regrouped_authoring_order.declaration_digest()
    );
    assert_eq!(
        first_authoring_order.retained_motion_digest(),
        regrouped_authoring_order.retained_motion_digest()
    );
    assert_eq!(
        first_authoring_order
            .continuation()
            .retained_motion_digest(),
        regrouped_authoring_order
            .continuation()
            .retained_motion_digest()
    );
}

fn exact_cancellation_motion_receipt(
    world: &'static str,
    add_motion_steps: impl FnOnce(PlanarMotionPosture) -> PlanarMotionPosture,
) -> PlanarMotionPostureReceipt {
    add_motion_steps(PlanarMotionPosture::from_boolean_readiness(
        boolean_readiness_receipt(world),
    ))
    .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
    .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
        world,
    )))
    .expect("motion posture plan")
    .certify()
    .expect("motion posture receipt")
}
