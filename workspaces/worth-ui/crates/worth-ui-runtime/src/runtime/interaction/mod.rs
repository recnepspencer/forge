pub(crate) mod gesture;
pub(crate) mod targeting;

pub use gesture::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionLifecycleCounters,
    UiInteractionObservationDenial, UiInteractionShutdownReport, UiInteractionStateSnapshot,
    UiPointerGestureContinuityKind, UiPointerGesturePressReceipt, UiPointerGestureStop,
    UiPointerGestureStopReason, UiPointerGestureTransition, UiTargetedPointerGesture,
};
pub use targeting::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedTargetFrameRelation,
};

pub(crate) use gesture::UiInteractionRuntimeState;
