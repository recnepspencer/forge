//! Private runtime join between text-owned raster meaning and host-owned atlas
//! effects.
//!
//! The runtime derives exact demands and rasterizes only the misses selected
//! by the host transaction. Physical Signal progression and atlas ownership
//! remain inside the native host; this module retains only portable demand,
//! pin-candidate, and settlement meaning.

#[cfg(any(test, feature = "certification-support"))]
mod gate_d_pin_evidence;
mod mounted_coordinator;
mod preparation;
mod rasterization;
mod recovery;
mod transaction;

#[cfg(any(test, feature = "certification-support"))]
pub(crate) use gate_d_pin_evidence::run_gate_d_pin_world;
#[cfg(any(test, feature = "certification-support"))]
pub use gate_d_pin_evidence::UiGateDPinWorldEvidence;
pub(crate) use mounted_coordinator::{
    UiNativeMountedTextCoordinator, UiNativeMountedTextOutcome, UiNativeMountedTextPending,
    UiNativeMountedTextReleaseOutcome,
};
pub(crate) use preparation::{
    prepare_mounted_semantic_text, UiMountedEventTimeDpiAuthority,
    UiNativeTextPresentationPreparation, UiNativeTextPresentationPrepared,
    UiNativeTextPresentationReadiness,
};
pub(crate) use rasterization::{UiNativeTextMissRasterizer, UiNativeTextRasterWorkReport};
pub(crate) use transaction::UiNativeTextAtlasTransaction;
