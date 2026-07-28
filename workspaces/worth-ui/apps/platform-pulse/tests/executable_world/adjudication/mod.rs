#[cfg(target_os = "windows")]
mod identity_trace;
#[cfg(target_os = "windows")]
mod lifecycle_cleanup;
#[cfg(target_os = "windows")]
mod native_color;
#[cfg(target_os = "windows")]
mod predecessor_preservation;
#[cfg(target_os = "windows")]
mod publication_identity;
#[cfg(target_os = "windows")]
mod replacement_to_pixel;
#[cfg(target_os = "windows")]
mod source_to_pixel;
#[cfg(target_os = "windows")]
mod visual_overlay_pixels;

#[cfg(target_os = "windows")]
pub(crate) use identity_trace::{
    adjudicate_visual_retirement, adjudicate_visual_snapshot, adjudicate_visual_trace,
    ExecutableVisualIdentityFailure, ExecutableVisualRetirementEvidence,
    ExecutableVisualSnapshotEvidence, ExecutableVisualTraceEvidence,
};
#[cfg(target_os = "windows")]
pub(crate) use lifecycle_cleanup::{
    adjudicate_lifecycle_cleanup, CausalLifecycleCleanupObservationSet,
    ExecutableLifecycleCleanupEvidence, ExecutableLifecycleCleanupFailure,
};
#[cfg(target_os = "windows")]
pub(crate) use native_color::{
    adjudicate_native_color, ExpectedNativeColor, NativeColorFailure, NativeColorVerdict,
};
#[cfg(target_os = "windows")]
pub(crate) use predecessor_preservation::{
    adjudicate_predecessor_preservation, CausalPredecessorPreservationObservationSet,
    ExecutablePredecessorPreservationEvidence, ExecutablePredecessorPreservationFailure,
};
#[cfg(target_os = "windows")]
pub(crate) use publication_identity::ExecutablePublishedIdentity;
#[cfg(target_os = "windows")]
pub(crate) use replacement_to_pixel::{
    adjudicate_replacement, CausalReplacementObservationSet, ExecutableReplacementEvidence,
    ExecutableReplacementFailure, ReplacementExpectation,
};
#[cfg(target_os = "windows")]
pub(crate) use source_to_pixel::{
    adjudicate_first_frame, CausalFirstFrameObservationSet, ExecutableFirstFrameEvidence,
    ExecutableFirstFrameFailure,
};
#[cfg(target_os = "windows")]
pub(crate) use visual_overlay_pixels::{
    adjudicate_overlay_pixels, adjudicate_restored_pixels, ExecutableVisualClearEvidence,
    ExecutableVisualOverlayEvidence,
};
