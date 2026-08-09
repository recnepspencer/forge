//! Public Worth UI runtime surfaces ordered by lifecycle capability and authority class.
//!
//! Lifecycle order: entry → lifecycle → registry → runtime_handoff → boundaries → evidence → host → inspection

pub mod admission;
mod app_inspection_closeout;
pub mod application;
pub mod declaration;
pub mod entry;
pub mod evidence;
pub mod execution;
pub mod graph;
pub mod host;
mod host_session_authority;
mod inspection;
pub mod inspection_bridge;
mod inspection_observation;
mod inspection_receipt;
pub mod intent;
pub mod interaction;
pub mod lifecycle;
pub mod measurement_exchange;
mod measurement_inspection_evidence;
#[cfg(test)]
mod measurement_inspection_test_support;
#[cfg(test)]
mod measurement_inspection_tests;
pub mod mounted;
pub mod obligations;
pub mod observation;
pub mod observation_report;
pub mod prepared_application_authority;
pub mod query_binding;
pub mod rebind;
pub mod registry;
mod retained_obligation_registry;
pub mod runtime_handoff;
pub mod source_ingress;

#[cfg(test)]
pub(crate) use crate::declaration::WorthUiRustAuthoredDeclarationFixture;
pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;

pub use entry::{
    CapabilityRegistrationBuilder, UiChangeProfileInstalled, UiChangeProfileMissing, WorthUi,
    WorthUiActiveApplicationGenerationIdentity, WorthUiActiveApplicationSession,
    WorthUiActiveApplicationSessionIdentity, WorthUiActiveCanvasSpatialFrameCompletion,
    WorthUiActiveFrameworkTurnCompletion, WorthUiActiveFrameworkTurnExecution,
    WorthUiActiveInspectionReceipt, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
    WorthUiAllocationCatalogActivationDenial, WorthUiApp, WorthUiApplicationBuilder,
    WorthUiApplicationCutoverDenial, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationReplacementLoweringDenial, WorthUiApplicationReplacementOutcome,
    WorthUiApplicationReplacementPreparationDenial, WorthUiApplicationReplacementStagingDenial,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiCandidateInspectionReceipt,
    WorthUiHostNeutralApp, WorthUiLoweredApplicationReplacement,
    WorthUiMountedApplicationReplacementInFlight,
    WorthUiMountedApplicationReplacementIndeterminate, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedFrameExecutionStop, WorthUiMountedFrameFrameworkTransitionStop,
    WorthUiMountedLaneProjectionDenial, WorthUiMountedPreviewAdmissionRejection,
    WorthUiMountedPreviewCompletionRejection, WorthUiMountedPreviewDisposition,
    WorthUiMountedPreviewInFlight, WorthUiMountedPreviewOutcome,
    WorthUiMountedPreviewPreparationDenial, WorthUiMountedPreviewPreparationRejection,
    WorthUiMountedPreviewRetentionRejection, WorthUiMountedReplacementAdmissionDenial,
    WorthUiMountedReplacementCompletionDenial, WorthUiMountedReplacementPreparationOutcome,
    WorthUiMountedReplacementRetentionDenial, WorthUiNativeApplicationShell,
    WorthUiNativeApplicationShellLaunchDenial, WorthUiNativeApplicationShutdownReceipt,
    WorthUiNativeIntentAttemptPrepared, WorthUiNativeIntentConfirmationRequired,
    WorthUiNativeIntentIngress, WorthUiNativeIntentPosture, WorthUiNativeIntentPostureKind,
    WorthUiNativeIntentPosturePublicationCompletion, WorthUiNativeIntentPosturePublicationOutcome,
    WorthUiNativeIntentPosturePublicationRecovery, WorthUiNativeIntentPosturePublicationStop,
    WorthUiNativeIntentStop, WorthUiNativeIntentStopped, WorthUiNativeIntentTerminalPostureOutcome,
    WorthUiNativeIntentTransition, WorthUiNativeInteractionIngressStop,
    WorthUiNativeProjectionRebindDenial, WorthUiNativeSourceRebindDenial,
    WorthUiPendingApplicationCutover, WorthUiPendingMountedPreview,
    WorthUiPreparedApplicationReplacement, WorthUiPreparedMountedApplicationReplacement,
    WorthUiPreparedMountedPreview, WorthUiReplacementCandidateSummary,
    WorthUiReplacementPlannedCostEnvelope, WorthUiResolvedMountedPreview,
};
pub(crate) use host_session_authority::WorthUiHostSessionActivationDenial;
pub(crate) use host_session_authority::WorthUiHostSessionAuthority;
pub(crate) use host_session_authority::{UiHostEffectPort, WorthUiHostPlanBinding};
pub use host_session_authority::{
    WorthUiHostMeasurementCapability, WorthUiHostMeasurementSessionInput,
    WorthUiHostSessionIdentity,
};
pub use lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
