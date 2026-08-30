use std::collections::BTreeMap;

use worth_ui_host_contract::{
    UiHostObservationSequence, UiHostObservationTimeBasis, UiHostPointerButton,
    UiHostPointerCaptureEpoch, UiHostPointerIdentity,
};

use super::super::UiPointerGestureStop;
use crate::runtime::interaction::targeting::{
    UiPointerGestureContinuityKind, UiPresentedInteractionTarget,
};

pub const UI_ACTIVE_POINTER_GESTURE_LIMIT: usize = 16;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiPointerGestureLifecycleCounters {
    pub(crate) button_reports: u64,
    pub(crate) gestures_started: u64,
    pub(crate) gestures_completed: u64,
    pub(crate) stop_outcomes: u64,
    pub(crate) active_gestures_settled: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiPointerGestureStateSnapshot {
    pub(crate) active_gestures: usize,
    pub(crate) counters: UiPointerGestureLifecycleCounters,
}

#[derive(Debug)]
pub struct UiPointerGesturePressReceipt {
    pub(super) pointer: UiHostPointerIdentity,
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) sequence: UiHostObservationSequence,
    pub(super) time_basis: UiHostObservationTimeBasis,
    pub(super) position: worth_ui_host_contract::UiHostSurfacePosition,
    pub(super) target: crate::runtime::interaction::UiPresentedInteractionTargetView,
}

#[derive(Debug)]
pub struct UiTargetedPointerGesture {
    pub(super) pointer: UiHostPointerIdentity,
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) press_sequence: UiHostObservationSequence,
    pub(super) press_time_basis: UiHostObservationTimeBasis,
    pub(super) release_sequence: UiHostObservationSequence,
    pub(super) release_time_basis: UiHostObservationTimeBasis,
    pub(super) pressed: UiPresentedInteractionTarget,
    pub(super) released: UiPresentedInteractionTarget,
    pub(super) continuity: UiPointerGestureContinuityKind,
    pub(super) continuity_witness_digest: u64,
}

#[derive(Debug)]
pub(crate) enum UiPointerGestureOutcome {
    Pressed(UiPointerGesturePressReceipt),
    Completed(UiTargetedPointerGesture),
    Stopped(UiPointerGestureStop),
}

pub(crate) struct UiPointerGestureRuntimeState {
    pub(super) active: BTreeMap<UiHostPointerIdentity, UiActivePointerGesture>,
    pub(super) counters: UiPointerGestureLifecycleCounters,
}

pub(super) struct UiActivePointerGesture {
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) press_sequence: UiHostObservationSequence,
    pub(super) press_time_basis: UiHostObservationTimeBasis,
    pub(super) target: UiPresentedInteractionTarget,
}
