mod pointer;
mod stop;

pub use super::targeting::UiPointerGestureContinuityKind;
pub use pointer::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionLifecycleCounters,
    UiInteractionObservationDenial, UiInteractionShutdownReport, UiInteractionStateSnapshot,
    UiPointerGesturePressReceipt, UiPointerGestureTransition, UiTargetedPointerGesture,
};
pub use stop::{UiPointerGestureStop, UiPointerGestureStopReason};

pub(crate) use pointer::UiInteractionRuntimeState;
