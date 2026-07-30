//! Application entry and builder surfaces — first lifecycle capability.

mod active_application_admission;
mod active_application_inspection;
mod active_application_session;
mod active_framework_turn;
mod app;
mod app_builder;
mod application_replacement;
mod builder;
mod interaction;
mod measurement_exchange;
mod mounted_allocation_denial;
mod mounted_allocation_establishment;
mod mounted_allocation_inspection;
mod mounted_content_rebind;
mod mounted_frame_execution;
mod mounted_identity;
mod mounted_inspection;
mod mounted_preview;
mod mounted_publication;
#[cfg(test)]
mod native_application_identity_trace_test_support;
#[cfg(test)]
mod native_application_identity_trace_tests;
mod native_application_shell;
#[cfg(test)]
mod native_identity_trace_audit;
#[cfg(test)]
mod native_identity_trace_host;
mod native_projection_rebind;
#[cfg(test)]
mod native_projection_rebind_tests;
mod native_replacement_allocation;
mod native_source_rebind;
mod observation;
mod observation_report;
mod rebind_execution;
mod rebind_recovery;
mod visual_overlay;
mod visual_snapshot;
pub use crate::lifecycle::WorthUiActiveApplicationSessionIdentity;
pub use crate::runtime::exports::WorthUiAllocationCatalogActivationDenial;
pub use active_application_session::{
    WorthUiActiveApplicationSession, WorthUiActiveInspectionReceipt,
};
pub use active_framework_turn::{
    WorthUiActiveCanvasSpatialFrameCompletion, WorthUiActiveFrameworkTurnCompletion,
    WorthUiActiveFrameworkTurnExecution, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
    WorthUiMountedLaneProjectionDenial,
};
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{
    UiChangeProfileInstalled, UiChangeProfileMissing, WorthUiApplicationBuilder,
    WorthUiProjectionRegistrationError, WorthUiQueryViewRegistrationError,
};
pub use application_replacement::{
    WorthUiApplicationCutoverDenial, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationCutoverRetry, WorthUiApplicationPublicationObservation,
    WorthUiApplicationReplacementLoweringDenial, WorthUiApplicationReplacementOutcome,
    WorthUiApplicationReplacementPreparationDenial, WorthUiApplicationReplacementStagingDenial,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiCandidateInspectionReceipt,
    WorthUiLoweredApplicationReplacement, WorthUiMountedApplicationReplacementInFlight,
    WorthUiMountedApplicationReplacementIndeterminate, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedReplacementAdmissionDenial, WorthUiMountedReplacementCompletionDenial,
    WorthUiMountedReplacementPreparationOutcome, WorthUiMountedReplacementRetentionDenial,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationReplacement,
    WorthUiPreparedMountedApplicationReplacement, WorthUiReplacementCandidateSummary,
    WorthUiReplacementPlannedCostEnvelope,
};
pub use builder::CapabilityRegistrationBuilder;
pub use mounted_allocation_denial::{
    WorthUiMountedAllocationEstablishmentDenial, WorthUiMountedAllocationRuntimeStage,
};
pub use mounted_allocation_establishment::WorthUiMountedAllocationCertificationExt;
pub use mounted_allocation_establishment::{
    UiMountedAllocationMeasurementRequest, WorthUiMountedAllocationEstablishmentReceipt,
};
pub use mounted_allocation_inspection::{
    WorthUiMountedAllocationInspectionCertificationExt,
    WorthUiMountedAllocationProjectionInspectionDenial,
};
pub(crate) use mounted_content_rebind::{
    WorthUiMountedContentPublicationReceipt, WorthUiMountedContentRebindInFlight,
    WorthUiMountedContentRebindIndeterminate, WorthUiMountedContentRebindOutcome,
    WorthUiPreparedMountedContentRebind,
};
pub use mounted_frame_execution::{
    WorthUiMountedFrameExecutionStop, WorthUiMountedFrameFrameworkTransitionStop,
};
pub use mounted_identity::WorthUiMountedIdentityCertificationExt;
pub use mounted_preview::{
    WorthUiMountedPreviewAdmissionRejection, WorthUiMountedPreviewCompletionRejection,
    WorthUiMountedPreviewDisposition, WorthUiMountedPreviewInFlight, WorthUiMountedPreviewOutcome,
    WorthUiMountedPreviewPreparationDenial, WorthUiMountedPreviewPreparationRejection,
    WorthUiMountedPreviewRetentionRejection, WorthUiPendingMountedPreview,
    WorthUiPreparedMountedPreview, WorthUiResolvedMountedPreview,
};
pub use native_application_shell::{
    WorthUiNativeApplicationShell, WorthUiNativeApplicationShellLaunchDenial,
    WorthUiNativeApplicationShutdownReceipt,
};
pub use native_projection_rebind::WorthUiNativeProjectionRebindDenial;
pub use native_source_rebind::WorthUiNativeSourceRebindDenial;
pub(crate) use rebind_execution::WorthUiPreparedEvidenceOnlyApplicationRebind;
pub(crate) use rebind_recovery::WorthUiRebindRecoveryAuthority;
