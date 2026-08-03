mod pointer;
mod stop;

pub use super::targeting::UiPointerGestureContinuityKind;
pub use pointer::{
    UiPointerGesturePressReceipt, UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
pub use stop::{UiPointerGestureStop, UiPointerGestureStopReason};

pub(crate) use pointer::{
    UiPointerGestureOutcome, UiPointerGestureRuntimeState, UiPointerGestureStateSnapshot,
};
