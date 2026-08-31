mod current_target;
#[allow(
    dead_code,
    reason = "Gate 0 freezes primary-pointer admission before host consumption"
)]
mod inspection;
mod owner;
#[allow(
    dead_code,
    reason = "Gate 0 exposes owner-issued transitions before Gate 1 live resolver threading"
)]
mod transition;

pub(crate) use current_target::{
    UiPointerPresenceAppearanceOwnerSnapshot, UiPointerPresenceAppearancePosture,
    UiPointerPresenceClass,
};
pub(crate) use inspection::UiPrimaryPointerKind;
pub(crate) use owner::UiPointerPresenceOwner;
pub use transition::UiPointerPresenceTargetTransition;
