use worth_ui_host_contract::{
    UiHostObservationCanonicalCore, UiHostObservationPayload, UiHostObservationSequence,
    UiHostPointerButton, UiHostPointerButtonTransition, UiHostPointerCaptureEpoch,
    UiHostPointerIdentity,
};

use super::model::{
    UiActivePointerGesture, UiInteractionRuntimeState, UiPointerGesturePressReceipt,
    UiPointerGestureTransition, UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
use super::next;
use crate::runtime::interaction::gesture::{UiPointerGestureStop, UiPointerGestureStopReason};
use crate::runtime::interaction::targeting::{
    issue_continuity, resolve_presented_target, UiPointerGestureContinuityDenial,
};

impl UiInteractionRuntimeState {
    pub(super) fn process_report(
        &mut self,
        core: UiHostObservationCanonicalCore,
        report: &worth_ui_host_contract::UiHostObservationReport,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> Vec<UiPointerGestureTransition> {
        match report.payload() {
            UiHostObservationPayload::PointerButton {
                pointer,
                capture_epoch,
                button,
                transition,
                position,
            } => {
                self.bump_button_reports();
                vec![match transition {
                    UiHostPointerButtonTransition::Pressed => self.press(
                        core,
                        report.sequence(),
                        *pointer,
                        *capture_epoch,
                        *button,
                        *position,
                        mounted,
                    ),
                    UiHostPointerButtonTransition::Released => self.release(
                        core,
                        report.sequence(),
                        *pointer,
                        *capture_epoch,
                        *button,
                        *position,
                        mounted,
                    ),
                }]
            }
            UiHostObservationPayload::PointerMotion {
                pointer,
                capture_epoch,
                ..
            } => self.capture_change(report.sequence(), *pointer, *capture_epoch),
            UiHostObservationPayload::Focus { focused: false } => {
                self.focus_loss(report.sequence())
            }
            _ => Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn press(
        &mut self,
        core: UiHostObservationCanonicalCore,
        sequence: UiHostObservationSequence,
        pointer: UiHostPointerIdentity,
        capture_epoch: UiHostPointerCaptureEpoch,
        button: UiHostPointerButton,
        position: worth_ui_host_contract::UiHostSurfacePosition,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> UiPointerGestureTransition {
        if button != UiHostPointerButton::Primary {
            return self.failed_stop(
                pointer,
                capture_epoch,
                button,
                sequence,
                UiPointerGestureStopReason::UnsupportedButton(button),
            );
        }
        if let Some(active) = self.active.remove(&pointer) {
            let reason = capture_change_reason(&active, capture_epoch)
                .unwrap_or(UiPointerGestureStopReason::DuplicatePress);
            return self.active_stop(pointer, active, sequence, reason);
        }
        if self.active.len() >= UI_ACTIVE_POINTER_GESTURE_LIMIT {
            return self.failed_stop(
                pointer,
                capture_epoch,
                button,
                sequence,
                UiPointerGestureStopReason::CapacityExceeded {
                    limit: UI_ACTIVE_POINTER_GESTURE_LIMIT,
                },
            );
        }
        let target = match resolve_presented_target(mounted, core.presentation(), position) {
            Ok(target) => target,
            Err(denial) => {
                return self.failed_stop(
                    pointer,
                    capture_epoch,
                    button,
                    sequence,
                    UiPointerGestureStopReason::Targeting(denial),
                )
            }
        };
        let active = UiActivePointerGesture {
            capture_epoch,
            button,
            press_sequence: sequence,
            target: target.clone(),
        };
        self.active.insert(pointer, active);
        self.counters.gestures_started = next(self.counters.gestures_started);
        UiPointerGestureTransition::Pressed(UiPointerGesturePressReceipt {
            pointer,
            capture_epoch,
            button,
            sequence,
            target,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn release(
        &mut self,
        core: UiHostObservationCanonicalCore,
        sequence: UiHostObservationSequence,
        pointer: UiHostPointerIdentity,
        capture_epoch: UiHostPointerCaptureEpoch,
        button: UiHostPointerButton,
        position: worth_ui_host_contract::UiHostSurfacePosition,
        mounted: &crate::mounting::WorthUiMountedSessionState,
    ) -> UiPointerGestureTransition {
        let Some(active) = self.active.remove(&pointer) else {
            return self.failed_stop(
                pointer,
                capture_epoch,
                button,
                sequence,
                UiPointerGestureStopReason::NoActiveGesture,
            );
        };
        if let Some(reason) = capture_change_reason(&active, capture_epoch) {
            return self.active_stop(pointer, active, sequence, reason);
        }
        if active.button != button {
            let reason = UiPointerGestureStopReason::ButtonChanged {
                expected: active.button,
                observed: button,
            };
            return self.active_stop(pointer, active, sequence, reason);
        }
        let released = match resolve_presented_target(mounted, core.presentation(), position) {
            Ok(target) => target,
            Err(denial) => {
                return self.active_stop(
                    pointer,
                    active,
                    sequence,
                    UiPointerGestureStopReason::Targeting(denial),
                )
            }
        };
        let witness = match issue_continuity(&active.target, &released) {
            Ok(witness) => witness,
            Err(denial) => {
                return self.active_stop(pointer, active, sequence, map_continuity_denial(denial))
            }
        };
        self.counters.gestures_completed = next(self.counters.gestures_completed);
        self.counters.active_gestures_settled = next(self.counters.active_gestures_settled);
        UiPointerGestureTransition::Completed(UiTargetedPointerGesture {
            pointer,
            capture_epoch,
            button,
            press_sequence: active.press_sequence,
            release_sequence: sequence,
            pressed: active.target,
            released,
            continuity: witness.kind(),
            continuity_witness_digest: witness.digest(),
        })
    }

    fn capture_change(
        &mut self,
        sequence: UiHostObservationSequence,
        pointer: UiHostPointerIdentity,
        observed: UiHostPointerCaptureEpoch,
    ) -> Vec<UiPointerGestureTransition> {
        let Some(active) = self.active.get(&pointer) else {
            return Vec::new();
        };
        if active.capture_epoch == observed {
            return Vec::new();
        }
        let active = self
            .active
            .remove(&pointer)
            .expect("the active pointer was just observed");
        let reason = UiPointerGestureStopReason::CaptureChanged {
            expected: active.capture_epoch,
            observed,
        };
        vec![self.active_stop(pointer, active, sequence, reason)]
    }

    fn focus_loss(
        &mut self,
        sequence: UiHostObservationSequence,
    ) -> Vec<UiPointerGestureTransition> {
        let active = std::mem::take(&mut self.active);
        active
            .into_iter()
            .map(|(pointer, active)| {
                self.active_stop(
                    pointer,
                    active,
                    sequence,
                    UiPointerGestureStopReason::FocusLost,
                )
            })
            .collect()
    }

    fn failed_stop(
        &mut self,
        pointer: UiHostPointerIdentity,
        capture_epoch: UiHostPointerCaptureEpoch,
        button: UiHostPointerButton,
        sequence: UiHostObservationSequence,
        reason: UiPointerGestureStopReason,
    ) -> UiPointerGestureTransition {
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        UiPointerGestureTransition::Stopped(UiPointerGestureStop::new(
            pointer,
            capture_epoch,
            button,
            Some(sequence),
            false,
            reason,
        ))
    }

    fn active_stop(
        &mut self,
        pointer: UiHostPointerIdentity,
        active: UiActivePointerGesture,
        sequence: UiHostObservationSequence,
        reason: UiPointerGestureStopReason,
    ) -> UiPointerGestureTransition {
        self.counters.stop_outcomes = next(self.counters.stop_outcomes);
        self.counters.active_gestures_settled = next(self.counters.active_gestures_settled);
        UiPointerGestureTransition::Stopped(UiPointerGestureStop::new(
            pointer,
            active.capture_epoch,
            active.button,
            Some(sequence),
            true,
            reason,
        ))
    }
}

fn capture_change_reason(
    active: &UiActivePointerGesture,
    observed: UiHostPointerCaptureEpoch,
) -> Option<UiPointerGestureStopReason> {
    (active.capture_epoch != observed).then_some(UiPointerGestureStopReason::CaptureChanged {
        expected: active.capture_epoch,
        observed,
    })
}

fn map_continuity_denial(denial: UiPointerGestureContinuityDenial) -> UiPointerGestureStopReason {
    match denial {
        UiPointerGestureContinuityDenial::PresentationDidNotAdvance => {
            UiPointerGestureStopReason::PresentationDidNotAdvance
        }
        UiPointerGestureContinuityDenial::SurfaceChanged => {
            UiPointerGestureStopReason::SurfaceChanged
        }
        UiPointerGestureContinuityDenial::BindingChanged => {
            UiPointerGestureStopReason::BindingChanged
        }
        UiPointerGestureContinuityDenial::MountedIncarnationChanged => {
            UiPointerGestureStopReason::MountedIncarnationChanged
        }
        UiPointerGestureContinuityDenial::TargetChangedWithinPresentation => {
            UiPointerGestureStopReason::TargetChangedWithinPresentation
        }
    }
}
