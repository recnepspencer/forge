mod continuity;
mod presented_frame;

pub use continuity::UiPointerGestureContinuityKind;
pub use presented_frame::{
    UiInteractionTargetingDenial, UiPresentedInteractionTarget, UiPresentedTargetFrameRelation,
};

pub(crate) use continuity::{issue_continuity, UiPointerGestureContinuityDenial};
pub(crate) use presented_frame::resolve_presented_target;
