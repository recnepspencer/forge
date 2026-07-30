mod batch;
pub(crate) mod draft;
pub(crate) mod gesture;
mod semantic;
mod settlement;
mod snapshot;
mod state;
pub(crate) mod targeting;
mod transition;

pub use batch::{
    UiHostInteractionIngressOutcome, UiInteractionBatchReceipt, UiInteractionObservationDenial,
    UiInteractionShutdownReport, UiQuarantinedHostInteractionBatch,
};
pub use draft::{
    UiDraftByteBudget, UiDraftByteBudgetDenial, UiDraftFieldIdentity, UiDraftMutationKind,
    UiDraftMutationReceipt, UiDraftSessionIdentity, UiLocalInputRecipientAdmission,
    UiLocalInputRecipientBindingReceipt, UiLocalInputRecipientBindingStop,
    UiLocalInputRecipientBindingStopReason, UiLocalInputRecipientContract,
    UiLocalInputRecipientFamily, UiLocalInputStop, UiLocalInputStopReason, UI_DRAFT_SESSION_LIMIT,
    UI_DRAFT_UTF8_BYTE_LIMIT,
};
pub use gesture::{
    UiPointerGestureContinuityKind, UiPointerGesturePressReceipt, UiPointerGestureStop,
    UiPointerGestureStopReason, UiTargetedPointerGesture, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
pub use semantic::{
    UiActivateInteraction, UiActivateInteractionSource, UiEditCommitInteraction,
    UiKeyboardActivationEvidence, UiSemanticInteraction, UiSubmitInteraction,
};
pub(crate) use semantic::{UiEditCommitInput, UiKeyboardSemanticInput};
pub use settlement::UiInteractionLifecycleSettlementReceipt;
pub use snapshot::{UiInteractionLifecycleCounters, UiInteractionStateSnapshot};
pub(crate) use state::{UiInteractionLifecycleStopReason, UiInteractionRuntimeState};
pub use targeting::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedInteractionTargetView,
    UiPresentedTargetFrameRelation,
};
pub use transition::{UiInteractionStop, UiInteractionTransition};
