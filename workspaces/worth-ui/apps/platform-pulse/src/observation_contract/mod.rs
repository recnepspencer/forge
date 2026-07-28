mod envelope;
mod lifecycle;
mod projection;
mod terminal_projection;
mod visual;
mod visual_projection;
#[cfg(test)]
mod visual_tests;
mod visual_value_projection;

pub use envelope::{
    PlatformPulseLifecycleObservationCodecDenial, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProtocol, PlatformPulseObservationRunIdentity,
    PlatformPulseObservationSequence, PLATFORM_PULSE_LIFECYCLE_OBSERVATION_IDENTITY,
    PLATFORM_PULSE_LIFECYCLE_OBSERVATION_SCHEMA_VERSION,
    PLATFORM_PULSE_LIFECYCLE_OBSERVATION_STDOUT_PREFIX,
};
pub use lifecycle::{
    PlatformPulseApplicationGenerationObservation, PlatformPulseFirstFramePublished,
    PlatformPulseLaunchConfigurationDenial, PlatformPulseLaunchConfigurationDenialKind,
    PlatformPulseLifecycleObservation, PlatformPulseMountedFrameObservation,
    PlatformPulseProcessStarted, PlatformPulseReplacementDenialFamily,
    PlatformPulseReplacementPreserved, PlatformPulseReplacementPublished,
    PlatformPulseShutdownCompleted, PlatformPulseSourceSnapshotObservation,
    PlatformPulseTerminalFailure, PlatformPulseTerminalFailureFamily,
    PlatformPulseWatcherBackendObservation,
};
pub use projection::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
};
pub use visual::{
    PlatformPulseVisualCoordinateObservation, PlatformPulseVisualCoordinateOrientationObservation,
    PlatformPulseVisualCoordinateRoundingObservation, PlatformPulseVisualEvidenceFamilyObservation,
    PlatformPulseVisualEvidenceObservation, PlatformPulseVisualIdentityTraceObservation,
    PlatformPulseVisualMountedNodeObservation, PlatformPulseVisualOverlayCleared,
    PlatformPulseVisualOverlayPublished, PlatformPulseVisualPixelColorSpaceObservation,
    PlatformPulseVisualPixelObservation, PlatformPulseVisualPointResolutionObservation,
    PlatformPulseVisualPointTrace, PlatformPulseVisualSnapshotAffinityObservation,
    PlatformPulseVisualSnapshotCaptured, PlatformPulseVisualSnapshotRelationObservation,
    PlatformPulseVisualSnapshotRetired,
};
pub use visual_projection::{
    PlatformPulseVisualPointObservation, PlatformPulseVisualPointTraceInput,
};
