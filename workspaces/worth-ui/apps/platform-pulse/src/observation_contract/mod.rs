mod envelope;
mod lifecycle;
mod native_input;
mod projection;
#[cfg(test)]
mod projection_tests;
mod query;
mod query_projection;
mod schema_transition;
mod terminal_projection;
mod visual;
mod visual_projection;
#[cfg(test)]
mod visual_tests;
mod visual_value_projection;

pub use envelope::{
    PlatformPulseDecodedLifecycleObservation, PlatformPulseInheritedLifecycleOnly,
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
    PlatformPulseNativeRebindDenialStage, PlatformPulseNativeRebindPreparationDenial,
    PlatformPulseProcessStarted, PlatformPulseReplacementDenialFamily,
    PlatformPulseReplacementPreserved, PlatformPulseReplacementPublished,
    PlatformPulseShutdownCompleted, PlatformPulseSourceSnapshotObservation,
    PlatformPulseTerminalFailure, PlatformPulseTerminalFailureFamily,
    PlatformPulseVisualComparison, PlatformPulseWatcherBackendObservation,
};
pub use native_input::{PlatformPulseNativeInputIngressPosture, PlatformPulseNativeInputReached};
pub use projection::{
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
};
pub use query::{
    PlatformPulseLiveQueryResidue, PlatformPulseQueryProjectionEvidence,
    PlatformPulseQueryProjectionPosture, PlatformPulseQueryProjectionPublished,
    PlatformPulseQueryProjectionResidue, PlatformPulseQueryShutdownEvidence,
    PlatformPulseQueryWatcherShutdownEvidence,
};
pub use schema_transition::{
    PlatformPulseProjectionSchemaField, PlatformPulseProjectionSchemaTransitionKind,
    PlatformPulseProjectionSchemaTransitionObservation,
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
