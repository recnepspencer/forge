use worth_ui::facade::interaction::{
    UiActivateInteraction, UiHostInteractionIngressOutcome, UiInteractionBatchReceipt,
    UiInteractionStop, UiInteractionTransition, UiPointerGestureStopReason, UiSemanticInteraction,
};

use super::model::{ModelStop, ModelVerdict};

pub(super) fn actual_verdict(outcome: &UiHostInteractionIngressOutcome) -> ModelVerdict {
    match outcome {
        UiHostInteractionIngressOutcome::Applied(receipt) => applied_verdict(receipt),
        UiHostInteractionIngressOutcome::Denied(denial) => ModelVerdict {
            stops: denial.settlement().stops().len(),
            stop: common_stop(denial.settlement().stops()),
            active: denial.settlement().final_state().active_gestures(),
            ..Default::default()
        },
        UiHostInteractionIngressOutcome::Duplicate(_) => ModelVerdict::default(),
        UiHostInteractionIngressOutcome::Quarantined(quarantine) => ModelVerdict {
            stops: quarantine.settlement().stops().len(),
            stop: common_stop(quarantine.settlement().stops()),
            active: quarantine.settlement().final_state().active_gestures(),
            ..Default::default()
        },
    }
}

pub(super) fn applied(outcome: UiHostInteractionIngressOutcome) -> UiInteractionBatchReceipt {
    match outcome {
        UiHostInteractionIngressOutcome::Applied(receipt) => receipt,
        other => panic!("expected applied interaction batch, got {other:?}"),
    }
}

pub(super) fn take_pointer_activation(
    outcome: UiHostInteractionIngressOutcome,
) -> UiActivateInteraction {
    let receipt = applied(outcome);
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(activation)) => {
                Some(activation)
            }
            _ => None,
        })
        .expect("the complete pointer pair seals one activation")
}

fn applied_verdict(receipt: &UiInteractionBatchReceipt) -> ModelVerdict {
    let mut verdict = ModelVerdict {
        active: receipt.state().active_gestures(),
        ..Default::default()
    };
    for transition in receipt.transitions() {
        match transition {
            UiInteractionTransition::PointerPressed(_) => verdict.pressed += 1,
            UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(_)) => {
                verdict.semantics += 1
            }
            UiInteractionTransition::Stopped(UiInteractionStop::PointerGesture(stop)) => {
                verdict.stops += 1;
                verdict.stop = Some(model_stop(stop.reason()));
            }
            _ => {}
        }
    }
    verdict
}

fn common_stop(stops: &[worth_ui::facade::interaction::UiPointerGestureStop]) -> Option<ModelStop> {
    let first = stops.first().map(|stop| model_stop(stop.reason()))?;
    assert!(stops.iter().all(|stop| model_stop(stop.reason()) == first));
    Some(first)
}

fn model_stop(reason: UiPointerGestureStopReason) -> ModelStop {
    match reason {
        UiPointerGestureStopReason::CapacityExceeded { .. } => ModelStop::Capacity,
        UiPointerGestureStopReason::CaptureChanged { .. } => ModelStop::CaptureChanged,
        UiPointerGestureStopReason::FocusLost => ModelStop::FocusLost,
        UiPointerGestureStopReason::NoActiveGesture => ModelStop::NoActiveGesture,
        UiPointerGestureStopReason::Targeting(_) => ModelStop::Targeting,
        UiPointerGestureStopReason::SurfaceChanged
        | UiPointerGestureStopReason::BindingChanged
        | UiPointerGestureStopReason::MountedIncarnationChanged
        | UiPointerGestureStopReason::TargetChangedWithinPresentation => ModelStop::TargetChanged,
        other => panic!("independent trace did not predict stop {other:?}"),
    }
}
