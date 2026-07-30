use worth_ui::facade::interaction::{
    UiPointerGestureStopReason, UiPointerGestureTransition, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
use worth_ui::facade::observation_report::{
    UiHostObservationFamily, UiHostObservationReportDenial, UiHostPointerButtonTransition,
    UiHostPointerCaptureEpoch,
};
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;
use worth_ui_test_support::{
    WorthUiMountedIdentityCertificationExt, WorthUiMountedInteractionLifecycleCertificationExt,
};

use super::assertions::{applied, assert_rank, assert_stop, denied};
use super::oracle::ExpectedTarget;
use super::world::GestureWorld;

#[test]
fn capture_loss_overflow_rebind_and_shutdown_each_settle_once() {
    let mut world = GestureWorld::canonical();
    assert_rank(
        world.button(1, 1, UiHostPointerButtonTransition::Pressed, [20, 20]),
        ExpectedTarget::Rank(0),
    );
    assert_stop(
        world.motion(1, 2, [20, 20]),
        UiPointerGestureStopReason::CaptureChanged {
            expected: UiHostPointerCaptureEpoch::new(1),
            observed: UiHostPointerCaptureEpoch::new(2),
        },
    );
    assert_rank(
        world.button(2, 1, UiHostPointerButtonTransition::Pressed, [20, 20]),
        ExpectedTarget::Rank(0),
    );
    assert_stop(world.focus_loss(), UiPointerGestureStopReason::FocusLost);
    assert_rank(
        world.button(3, 1, UiHostPointerButtonTransition::Pressed, [20, 20]),
        ExpectedTarget::Rank(0),
    );
    let overflow = denied(world.pointer_button_overflow());
    assert_eq!(
        overflow.denial(),
        UiHostObservationReportDenial::LosslessOverflow(UiHostObservationFamily::PointerButton)
    );
    assert!(matches!(
        overflow.settlement().stops()[0].reason(),
        UiPointerGestureStopReason::PointerButtonLoss { .. }
    ));
    assert_eq!(overflow.settlement().final_state().active_gestures(), 0);
    assert_eq!(
        overflow
            .settlement()
            .final_state()
            .counters()
            .active_gestures_settled(),
        3
    );
    let _ = world.session.shutdown();

    let mut rebound = GestureWorld::canonical();
    assert_rank(
        rebound.button(1, 1, UiHostPointerButtonTransition::Pressed, [20, 20]),
        ExpectedTarget::Rank(0),
    );
    let binding = rebound
        .session
        .inspect_mounted_identity()
        .surface_bindings()
        .iter()
        .find(|candidate| candidate.binding_generation() == rebound.binding)
        .copied()
        .expect("the presented binding remains live");
    let receipt = rebound
        .session
        .rebind_host_surface_with_interaction_receipt(
            rebound.binding,
            UiHostSurfacePresentationMode::RecordOnly,
            binding.profile(),
        )
        .unwrap();
    assert_eq!(receipt.interaction().stops().len(), 1);
    assert_eq!(
        receipt.interaction().stops()[0].reason(),
        UiPointerGestureStopReason::SurfaceRebound
    );
    let shutdown = rebound.session.shutdown();
    assert_eq!(shutdown.interaction().cancelled_gestures(), 0);

    let mut shutting_down = GestureWorld::canonical();
    assert_rank(
        shutting_down.button(1, 1, UiHostPointerButtonTransition::Pressed, [20, 20]),
        ExpectedTarget::Rank(0),
    );
    let shutdown = shutting_down.session.shutdown();
    let settlement = shutdown
        .interaction()
        .settlement()
        .expect("session shutdown emits an interaction settlement");
    assert_eq!(settlement.stops().len(), 1);
    assert_eq!(
        settlement.stops()[0].reason(),
        UiPointerGestureStopReason::Shutdown
    );
    assert_eq!(settlement.final_state().active_gestures(), 0);
}

#[test]
fn active_pointer_capacity_stops_plus_one_without_disturbing_owned_slots() {
    let mut world = GestureWorld::canonical();
    for pointer in 1..=UI_ACTIVE_POINTER_GESTURE_LIMIT {
        let receipt = applied(world.button(
            u64::try_from(pointer).unwrap(),
            1,
            UiHostPointerButtonTransition::Pressed,
            [20, 20],
        ));
        assert_eq!(receipt.state().active_gestures(), pointer);
    }

    let overflow = applied(world.button(
        u64::try_from(UI_ACTIVE_POINTER_GESTURE_LIMIT + 1).unwrap(),
        1,
        UiHostPointerButtonTransition::Pressed,
        [20, 20],
    ));
    let stop = match &overflow.transitions()[0] {
        UiPointerGestureTransition::Stopped(stop) => stop,
        other => panic!("capacity plus one must stop, got {other:?}"),
    };
    assert_eq!(
        stop.reason(),
        UiPointerGestureStopReason::CapacityExceeded {
            limit: UI_ACTIVE_POINTER_GESTURE_LIMIT
        }
    );
    assert_eq!(
        overflow.state().active_gestures(),
        UI_ACTIVE_POINTER_GESTURE_LIMIT
    );

    let shutdown = world.session.shutdown();
    let settlement = shutdown.interaction().settlement().unwrap();
    assert_eq!(settlement.stops().len(), UI_ACTIVE_POINTER_GESTURE_LIMIT);
    assert!(settlement
        .stops()
        .iter()
        .all(|stop| stop.reason() == UiPointerGestureStopReason::Shutdown));
    assert_eq!(settlement.final_state().active_gestures(), 0);
}
