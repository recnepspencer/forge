mod command_routing;
mod focus;
mod motion;
mod normalized_plan;
mod policy_defaults;
mod portal;
mod scroll;
mod selection;

pub use command_routing::UiCommandRoutingPolicy;
pub(super) use command_routing::UiDeclaredCommandRoutingContract;
pub(super) use focus::UiDeclaredFocusOwnershipContract;
pub use focus::{UiFocusPolicy, UiFocusScopePolicy};
pub(super) use motion::UiDeclaredMotionPolicyContract;
pub use motion::{UiMotionPolicy, UiReducedMotionBehavior};
pub use normalized_plan::UiNormalizedServicePolicyPlan;
pub(crate) use policy_defaults::UiServicePolicyDefaults;
pub(crate) use portal::UiDeclaredPortalPlacementGeometry;
pub(super) use portal::UiDeclaredPortalSurfaceContract;
pub use portal::{UiPortalPolicy, UiPortalPolicyKind};
pub(super) use scroll::UiDeclaredScrollOwnershipContract;
pub use scroll::{UiScrollAnchorBehavior, UiScrollPolicy, UiScrollRevealAlignment};
pub(super) use selection::UiDeclaredSelectionIdentityContract;
pub use selection::{UiSelectionMode, UiSelectionPolicy};

#[cfg(test)]
mod tests;
