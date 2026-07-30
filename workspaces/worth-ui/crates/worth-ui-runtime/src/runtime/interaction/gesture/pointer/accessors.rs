use worth_ui_host_contract::{
    UiHostObservationCanonicalCore, UiHostObservationSequence, UiHostPointerButton,
    UiHostPointerCaptureEpoch, UiHostPointerIdentity,
};

use super::model::{
    UiInteractionBatchReceipt, UiInteractionLifecycleCounters, UiInteractionObservationDenial,
    UiInteractionShutdownReport, UiInteractionStateSnapshot, UiPointerGesturePressReceipt,
    UiTargetedPointerGesture,
};
use crate::runtime::interaction::targeting::{
    UiPointerGestureContinuityKind, UiPresentedInteractionTarget,
};

impl UiInteractionLifecycleCounters {
    pub const fn button_reports(self) -> u64 {
        self.button_reports
    }

    pub const fn gestures_started(self) -> u64 {
        self.gestures_started
    }

    pub const fn gestures_completed(self) -> u64 {
        self.gestures_completed
    }

    pub const fn stop_outcomes(self) -> u64 {
        self.stop_outcomes
    }

    pub const fn active_gestures_settled(self) -> u64 {
        self.active_gestures_settled
    }
}

impl UiInteractionStateSnapshot {
    pub const fn active_gestures(self) -> usize {
        self.active_gestures
    }

    pub const fn counters(self) -> UiInteractionLifecycleCounters {
        self.counters
    }
}

impl UiPointerGesturePressReceipt {
    pub const fn pointer(&self) -> UiHostPointerIdentity {
        self.pointer
    }

    pub const fn capture_epoch(&self) -> UiHostPointerCaptureEpoch {
        self.capture_epoch
    }

    pub const fn button(&self) -> UiHostPointerButton {
        self.button
    }

    pub const fn sequence(&self) -> UiHostObservationSequence {
        self.sequence
    }

    pub const fn target(&self) -> &UiPresentedInteractionTarget {
        &self.target
    }
}

impl UiTargetedPointerGesture {
    pub const fn pointer(&self) -> UiHostPointerIdentity {
        self.pointer
    }

    pub const fn capture_epoch(&self) -> UiHostPointerCaptureEpoch {
        self.capture_epoch
    }

    pub const fn button(&self) -> UiHostPointerButton {
        self.button
    }

    pub const fn press_sequence(&self) -> UiHostObservationSequence {
        self.press_sequence
    }

    pub const fn release_sequence(&self) -> UiHostObservationSequence {
        self.release_sequence
    }

    pub const fn pressed_target(&self) -> &UiPresentedInteractionTarget {
        &self.pressed
    }

    pub const fn released_target(&self) -> &UiPresentedInteractionTarget {
        &self.released
    }

    pub const fn continuity(&self) -> UiPointerGestureContinuityKind {
        self.continuity
    }

    pub const fn continuity_witness_digest(&self) -> u64 {
        self.continuity_witness_digest
    }
}

impl UiInteractionBatchReceipt {
    pub const fn canonical_core(&self) -> UiHostObservationCanonicalCore {
        self.core
    }

    pub const fn frame_relation(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationFrameRelation {
        self.frame_relation
    }

    pub const fn disposition(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationBatchDisposition {
        self.disposition
    }

    pub fn transitions(&self) -> &[super::model::UiPointerGestureTransition] {
        &self.transitions
    }

    pub const fn ignored_reports(&self) -> usize {
        self.ignored_reports
    }

    pub const fn state(&self) -> UiInteractionStateSnapshot {
        self.state
    }
}

impl UiInteractionObservationDenial {
    pub(crate) const fn new(
        denial: crate::facade::observation_report::UiHostObservationReportDenial,
        settled_gestures: usize,
    ) -> Self {
        Self {
            denial,
            settled_gestures,
        }
    }

    pub const fn denial(self) -> crate::facade::observation_report::UiHostObservationReportDenial {
        self.denial
    }

    pub const fn settled_gestures(self) -> usize {
        self.settled_gestures
    }
}

impl UiInteractionShutdownReport {
    pub const fn cancelled_gestures(self) -> usize {
        self.cancelled_gestures
    }

    pub const fn final_state(self) -> Option<UiInteractionStateSnapshot> {
        self.final_state
    }
}
