use worth_ui_host_contract::{
    UiHostObservationSequence, UiHostPointerButton, UiHostPointerCaptureEpoch,
    UiHostPointerIdentity,
};

use super::model::{UiPointerGesturePressReceipt, UiTargetedPointerGesture};
use crate::runtime::interaction::targeting::{
    UiPointerGestureContinuityKind, UiPresentedInteractionTargetView,
};

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

    pub const fn target(&self) -> UiPresentedInteractionTargetView {
        self.target
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

    pub const fn pressed_target(&self) -> UiPresentedInteractionTargetView {
        self.pressed.view()
    }

    pub const fn released_target(&self) -> UiPresentedInteractionTargetView {
        self.released.view()
    }

    pub const fn continuity(&self) -> UiPointerGestureContinuityKind {
        self.continuity
    }

    pub const fn continuity_witness_digest(&self) -> u64 {
        self.continuity_witness_digest
    }
}
