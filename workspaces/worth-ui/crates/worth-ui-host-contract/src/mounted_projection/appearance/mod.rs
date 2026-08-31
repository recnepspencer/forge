mod backdrop;
mod color;
mod compositing;
mod geometry;
mod native_profile;
mod outline;
mod overlay_order;
mod pointer_affordance;
mod portal_surface;
mod surface;
mod text_foreground;

pub use backdrop::{
    UiMountedBackdropCompletionDenial, UiMountedBackdropCompletionInput, UiMountedBackdropIdentity,
    UiMountedBackdropMechanic, UiOverlayPlacementReceipt,
};
pub use color::{UiMountedAppearanceColor, UiMountedAppearanceOpacity};
pub use compositing::{
    compose_source_over, SRGB_GAMMA_DENOMINATOR, SRGB_GAMMA_NUMERATOR,
    SRGB_LINEAR_SCALE_DENOMINATOR, SRGB_LINEAR_SCALE_NUMERATOR, SRGB_LINEAR_THRESHOLD_DENOMINATOR,
    SRGB_LINEAR_THRESHOLD_NUMERATOR, SRGB_OFFSET_DENOMINATOR, SRGB_OFFSET_NUMERATOR,
    SRGB_SCALE_DENOMINATOR, SRGB_SCALE_NUMERATOR,
};
pub use geometry::{
    UiAppearanceClip, UiAppearanceDamageAttribution, UiAppearanceDamageRegion,
    UiAppearanceEmptyRegion, UiAppearancePhysicalRadii,
};
pub use native_profile::{
    UiHostAppearanceMechanicFamily, UiHostAppearanceProfileContract, UiHostAppearanceProfileDenial,
};
pub use outline::{
    UiMountedOutlineAppearanceCompletionDenial, UiMountedOutlineAppearanceCompletionInput,
    UiMountedOutlineAppearanceMechanic,
};
pub use overlay_order::{
    UiOverlayParticipantIdentity, UiOverlayStackSnapshot, UiOverlayStackSnapshotDenial,
};
pub use pointer_affordance::{
    UiHostPrimaryPointerKind, UiMountedPointerAffordanceMechanic, UiPointerAffordanceFamily,
};
pub use portal_surface::{
    UiMountedPortalSurfaceAppearanceCompletionDenial, UiMountedPortalSurfaceAppearanceMechanic,
};
pub use surface::{
    UiAppearanceProjectionAttribution, UiMountedSurfaceAppearanceCompletionDenial,
    UiMountedSurfaceAppearanceCompletionInput, UiMountedSurfaceAppearanceMechanic,
    UiMountedSurfacePaint,
};
pub use text_foreground::{
    UiMountedTextForegroundAppearanceCompletionDenial,
    UiMountedTextForegroundAppearanceCompletionInput, UiMountedTextForegroundAppearanceMechanic,
};
