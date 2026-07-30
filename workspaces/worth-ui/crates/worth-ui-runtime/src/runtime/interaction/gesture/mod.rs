mod pointer;
mod settlement;
mod stop;

pub use super::targeting::UiPointerGestureContinuityKind;
pub use pointer::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionLifecycleCounters,
    UiInteractionObservationDenial, UiInteractionShutdownReport, UiInteractionStateSnapshot,
    UiPointerGesturePressReceipt, UiPointerGestureTransition, UiQuarantinedHostInteractionBatch,
    UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
pub use settlement::UiInteractionLifecycleSettlementReceipt;
pub use stop::{UiPointerGestureStop, UiPointerGestureStopReason};

pub(crate) use pointer::UiInteractionRuntimeState;
