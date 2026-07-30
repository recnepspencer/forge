use std::collections::BTreeMap;

use worth_ui_host_contract::{
    UiHostObservationCanonicalCore, UiHostObservationSequence, UiHostPointerButton,
    UiHostPointerCaptureEpoch, UiHostPointerIdentity,
};

use super::super::UiPointerGestureStop;
use crate::runtime::interaction::targeting::{
    UiPointerGestureContinuityKind, UiPresentedInteractionTarget,
};

pub const UI_ACTIVE_POINTER_GESTURE_LIMIT: usize = 16;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiInteractionLifecycleCounters {
    pub(super) button_reports: u64,
    pub(super) gestures_started: u64,
    pub(super) gestures_completed: u64,
    pub(super) stop_outcomes: u64,
    pub(super) active_gestures_settled: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInteractionStateSnapshot {
    pub(super) active_gestures: usize,
    pub(super) counters: UiInteractionLifecycleCounters,
}

#[derive(Debug)]
pub struct UiPointerGesturePressReceipt {
    pub(super) pointer: UiHostPointerIdentity,
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) sequence: UiHostObservationSequence,
    pub(super) target: UiPresentedInteractionTarget,
}

#[derive(Debug)]
pub struct UiTargetedPointerGesture {
    pub(super) pointer: UiHostPointerIdentity,
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) press_sequence: UiHostObservationSequence,
    pub(super) release_sequence: UiHostObservationSequence,
    pub(super) pressed: UiPresentedInteractionTarget,
    pub(super) released: UiPresentedInteractionTarget,
    pub(super) continuity: UiPointerGestureContinuityKind,
    pub(super) continuity_witness_digest: u64,
}

#[derive(Debug)]
pub enum UiPointerGestureTransition {
    Pressed(UiPointerGesturePressReceipt),
    Completed(UiTargetedPointerGesture),
    Stopped(UiPointerGestureStop),
}

#[derive(Debug)]
pub struct UiInteractionBatchReceipt {
    pub(super) core: UiHostObservationCanonicalCore,
    pub(super) frame_relation: crate::facade::observation_report::UiHostObservationFrameRelation,
    pub(super) disposition: crate::facade::observation_report::UiHostObservationBatchDisposition,
    pub(super) transitions: Box<[UiPointerGestureTransition]>,
    pub(super) ignored_reports: usize,
    pub(super) state: UiInteractionStateSnapshot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInteractionObservationDenial {
    pub(super) denial: crate::facade::observation_report::UiHostObservationReportDenial,
    pub(super) settled_gestures: usize,
}

#[derive(Debug)]
pub enum UiHostInteractionIngressOutcome {
    Applied(UiInteractionBatchReceipt),
    Duplicate(crate::facade::observation_report::UiDuplicateHostObservationBatch),
    Quarantined(crate::facade::observation_report::UiQuarantinedHostObservationBatch),
    Denied(UiInteractionObservationDenial),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiInteractionShutdownReport {
    pub(super) cancelled_gestures: usize,
    pub(super) final_state: Option<UiInteractionStateSnapshot>,
}

pub(crate) struct UiInteractionRuntimeState {
    pub(super) active: BTreeMap<UiHostPointerIdentity, UiActivePointerGesture>,
    pub(super) counters: UiInteractionLifecycleCounters,
}

pub(super) struct UiActivePointerGesture {
    pub(super) capture_epoch: UiHostPointerCaptureEpoch,
    pub(super) button: UiHostPointerButton,
    pub(super) press_sequence: UiHostObservationSequence,
    pub(super) target: UiPresentedInteractionTarget,
}
