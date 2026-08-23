//! Portable recovery posture for native text-atlas effects.
//!
//! Native correlation and GPU ownership remain private to the native host.
//! The runtime treats effects-indeterminate as terminal presentation
//! uncertainty and never forges or interprets a native recovery token.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeTextPresentationRecoveryPosture {
    HostOwned,
}
