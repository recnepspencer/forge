mod continuity;
mod execution_affinity;
mod presented_frame;

pub use continuity::UiPointerGestureContinuityKind;
#[cfg(test)]
pub(crate) use presented_frame::interaction_target_view_for_test;
pub(crate) use presented_frame::{
    admit_current_target, admit_current_target_incarnation, require_current_target,
};
pub use presented_frame::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedInteractionTargetView,
    UiPresentedTargetFrameRelation,
};

pub(crate) use continuity::{issue_continuity, UiPointerGestureContinuityDenial};
pub(crate) use execution_affinity::{
    admit_continued_intent_execution_affinity, admit_presented_intent_execution_affinity,
    UiIntentExecutionTargetAffinity,
};
pub(crate) use presented_frame::resolve_presented_target;
