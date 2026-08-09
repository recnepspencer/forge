//! Contract-only Phase 1 owner for qualified native mechanics profiles.
//!
//! Event-loop, window, graphics, shaping, raster, and readback effects are
//! deliberately absent until their owning vertical phases activate them.

mod native_profile;
mod prepared_host;
mod text_profile;

pub use native_profile::{
    UiNativeMechanicsCapacities, UiNativePlatformProfileIdentity, WORTH_UI_NATIVE_PROFILE_MANIFEST,
};
pub use prepared_host::WorthUiPreparedNativeHost;
pub use text_profile::{
    UiBodyDefaultAtlasCapacities, UiBodyDefaultTextProfileIdentity,
    UiUnsupportedBodyDefaultCodePoint, WORTH_UI_BODY_DEFAULT_FONT, WORTH_UI_BODY_DEFAULT_LICENSE,
    WORTH_UI_TEXT_PROFILE_MANIFEST,
};

#[cfg(test)]
mod qualification_tests;
