use worth_ui::facade::interaction::{
    UiActivateInteractionSource, UiHostInteractionIngressOutcome, UiInteractionBatchReceipt,
    UiInteractionStop, UiInteractionTargetingDenial, UiInteractionTransition,
    UiPointerGestureContinuityKind, UiPointerGestureStop, UiPointerGestureStopReason,
    UiSemanticInteraction, UiTargetedPointerGesture,
};

use super::oracle::ExpectedTarget;

pub(super) fn assert_rank(outcome: UiHostInteractionIngressOutcome, expected: ExpectedTarget) {
    let receipt = applied(outcome);
    let press = match &receipt.transitions()[0] {
        UiInteractionTransition::PointerPressed(press) => press,
        other => panic!("expected a targeted press, got {other:?}"),
    };
    match expected {
        ExpectedTarget::Rank(rank) => assert_eq!(press.target().hit_test_order(), rank),
        ExpectedTarget::None => panic!("the independent oracle expected no target"),
    }
}

pub(super) fn assert_completion(
    outcome: UiHostInteractionIngressOutcome,
    expected: UiPointerGestureContinuityKind,
) {
    let receipt = applied(outcome);
    let completed = match &receipt.transitions()[0] {
        UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(activation)) => {
            pointer_gesture(activation.source())
        }
        other => panic!("expected a completed gesture, got {other:?}"),
    };
    assert_eq!(completed.continuity(), expected);
    assert_eq!(receipt.state().active_gestures(), 0);
    assert!(
        receipt.state().counters().active_gestures_settled()
            >= receipt.state().counters().gestures_completed()
    );
}

pub(super) fn assert_targeting_stop(
    outcome: UiHostInteractionIngressOutcome,
    expected: UiInteractionTargetingDenial,
) {
    assert_stop(outcome, UiPointerGestureStopReason::Targeting(expected));
}

pub(super) fn assert_stop(
    outcome: UiHostInteractionIngressOutcome,
    expected: UiPointerGestureStopReason,
) {
    let receipt = applied(outcome);
    assert_eq!(stopped(&receipt).reason(), expected);
    assert_eq!(receipt.state().active_gestures(), 0);
}

pub(super) fn stopped(receipt: &UiInteractionBatchReceipt) -> &UiPointerGestureStop {
    match &receipt.transitions()[0] {
        UiInteractionTransition::Stopped(UiInteractionStop::PointerGesture(stop)) => stop,
        other => panic!("expected a typed stop, got {other:?}"),
    }
}

pub(super) fn pointer_gesture(source: &UiActivateInteractionSource) -> &UiTargetedPointerGesture {
    match source {
        UiActivateInteractionSource::Pointer(gesture) => gesture,
        UiActivateInteractionSource::Keyboard(_) => {
            panic!("expected pointer activation, got keyboard activation")
        }
    }
}

pub(super) fn applied(outcome: UiHostInteractionIngressOutcome) -> UiInteractionBatchReceipt {
    match outcome {
        UiHostInteractionIngressOutcome::Applied(receipt) => receipt,
        other => panic!("expected an applied interaction batch, got {other:?}"),
    }
}

pub(super) fn denied(
    outcome: UiHostInteractionIngressOutcome,
) -> worth_ui::facade::interaction::UiInteractionObservationDenial {
    match outcome {
        UiHostInteractionIngressOutcome::Denied(denial) => denial,
        other => panic!("expected an observation denial, got {other:?}"),
    }
}
