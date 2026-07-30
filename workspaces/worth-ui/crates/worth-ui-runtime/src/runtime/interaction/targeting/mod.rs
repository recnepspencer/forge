mod continuity;
mod presented_frame;

pub use continuity::UiPointerGestureContinuityKind;
#[cfg(test)]
pub(crate) use presented_frame::interaction_target_view_for_test;
pub(crate) use presented_frame::require_current_target;
pub use presented_frame::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedInteractionTargetView,
    UiPresentedTargetFrameRelation,
};

pub(crate) use continuity::{issue_continuity, UiPointerGestureContinuityDenial};
pub(crate) use presented_frame::resolve_presented_target;
