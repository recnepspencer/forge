mod current_target;
#[allow(
    dead_code,
    reason = "Gate 0 freezes primary-pointer admission before host consumption"
)]
mod inspection;
mod owner;
#[allow(
    dead_code,
    reason = "Gate 0 carries pointer transitions without publishing appearance"
)]
mod transition;

pub(crate) use current_target::{
    UiPointerPresenceAppearanceOwnerSnapshot, UiPointerPresenceAppearancePosture,
    UiPointerPresenceClass,
};
pub(crate) use inspection::UiPrimaryPointerKind;
pub(crate) use owner::UiPointerPresenceOwner;
pub(crate) use transition::UiPointerPresenceTargetTransition;
