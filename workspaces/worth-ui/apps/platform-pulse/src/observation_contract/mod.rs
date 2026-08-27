mod envelope;
mod focus;
mod focus_projection;
mod intent;
mod launch;
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
pub use focus::{
    PlatformPulseSemanticFocusCause, PlatformPulseSemanticFocusOutcome,
    PlatformPulseSemanticFocusParticipant, PlatformPulseSemanticFocusPhysicalOutcome,
    PlatformPulseSemanticFocusPublished,
};
pub use intent::{
    PlatformPulseIntentAdmissionTrace, PlatformPulseIntentAttemptObservationReference,
    PlatformPulseIntentCausalTraceObservation, PlatformPulseIntentEvidenceReferenceObservation,
    PlatformPulseIntentExecutorGateObservation, PlatformPulseIntentExecutorStartedObservation,
    PlatformPulseIntentInputObservation, PlatformPulseIntentInteractionFamily,
    PlatformPulseIntentOperabilityObservation, PlatformPulseIntentOperabilityTrace,
    PlatformPulseIntentOutcomeTrace, PlatformPulseIntentPayloadTrace,
    PlatformPulseIntentPostureObservation, PlatformPulseIntentPosturePublished,
    PlatformPulseIntentRouteTrace, PlatformPulseIntentSourceTrace,
    PlatformPulseIntentTraceProjectionDenial, PlatformPulseIntentWatcherShutdownEvidence,
    PlatformPulseQueryActionObservation,
};
pub use launch::{
    PlatformPulseLaunchConfigurationDenial, PlatformPulseLaunchConfigurationDenialKind,
};
pub use lifecycle::{
    PlatformPulseApplicationGenerationObservation, PlatformPulseFirstFramePublished,
    PlatformPulseLifecycleObservation, PlatformPulseMountedFrameObservation,
    PlatformPulseNativeRebindDenialStage, PlatformPulseNativeRebindPreparationDenial,
    PlatformPulsePortalDismissed, PlatformPulseProcessStarted,
    PlatformPulseReplacementDenialFamily, PlatformPulseReplacementPreserved,
    PlatformPulseReplacementPublished, PlatformPulseShutdownCompleted,
    PlatformPulseSourceSnapshotObservation, PlatformPulseTerminalFailure,
    PlatformPulseTerminalFailureFamily, PlatformPulseVisualComparison,
    PlatformPulseWatcherBackendObservation,
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
