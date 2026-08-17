//! Private runtime join between text-owned raster meaning and host-owned atlas
//! effects.
//!
//! The runtime derives exact demands and rasterizes only the misses selected
//! by the host transaction. Physical Signal progression and atlas ownership
//! remain inside the native host; this module retains only portable demand,
//! pin-candidate, and settlement meaning.

mod mounted_coordinator;
mod preparation;
mod rasterization;
mod recovery;
mod transaction;

pub(crate) use mounted_coordinator::{
    UiNativeMountedSurfaceTextObservation, UiNativeMountedTextCoordinator,
};
pub(crate) use preparation::{
    prepare_mounted_semantic_text, UiMountedEventTimeDpiAuthority,
    UiNativeTextPresentationPreparation, UiNativeTextPresentationPrepared,
    UiNativeTextPresentationReadiness,
};
pub(crate) use rasterization::{UiNativeTextMissRasterizer, UiNativeTextRasterWorkReport};
pub(crate) use transaction::UiNativeTextAtlasTransaction;
