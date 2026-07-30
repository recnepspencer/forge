pub(crate) mod gesture;
pub(crate) mod targeting;

pub use gesture::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionLifecycleCounters,
    UiInteractionLifecycleSettlementReceipt, UiInteractionObservationDenial,
    UiInteractionShutdownReport, UiInteractionStateSnapshot, UiPointerGestureContinuityKind,
    UiPointerGesturePressReceipt, UiPointerGestureStop, UiPointerGestureStopReason,
    UiPointerGestureTransition, UiQuarantinedHostInteractionBatch, UiTargetedPointerGesture,
    UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
pub use targeting::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedTargetFrameRelation,
};

pub(crate) use gesture::UiInteractionRuntimeState;
