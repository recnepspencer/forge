#[cfg(target_os = "windows")]
mod first_frame_progression;
mod launch;
mod native_desktop_lease;
#[cfg(target_os = "windows")]
mod normal_close_progression;
#[cfg(target_os = "windows")]
mod preservation_progression;
#[cfg(target_os = "windows")]
mod progression;
#[cfg(target_os = "windows")]
mod replacement_progression;
mod shutdown;
#[cfg(target_os = "windows")]
mod source_action_progression;
#[cfg(target_os = "windows")]
mod visual_snapshot_progression;
#[cfg(target_os = "windows")]
mod watched_native_observation;
mod watched_observation;

pub(crate) use launch::{
    CargoBuiltPlatformPulse, EmergencyPlatformPulseExit, EmergencyPlatformPulseExitFailure,
    LivePlatformPulseProcess, PlatformPulseProcessLaunchFailure,
};
#[cfg(target_os = "windows")]
pub(crate) use progression::{
    AwaitingFirstFrame, AwaitingPreservation, AwaitingRecovery, AwaitingReplacement, Closed,
    GreenSuccessor, IdentityTraced, InitialBlue, Installed, NativeBoundExecutableWorld,
    OverlayCleared, OverlayPublished, PreservedPredecessor, PreservedPredecessorEvidence,
    Published, PulseExecutableWorld, RecoveredBlue, SnapshotCaptured,
};
pub(crate) use shutdown::{PlatformPulseProcessExitFailure, SuccessfulPlatformPulseExit};
pub(crate) use watched_observation::{
    await_watched_observation, WatchedPulseObservationFailure, WatchedPulseTransition,
};
