//! Application entry and builder surfaces — first lifecycle capability.

mod active_application_admission;
mod active_application_inspection;
mod active_application_session;
mod active_application_session_identity;
mod active_framework_turn;
mod active_host_output_handoff;
mod app;
mod app_builder;
mod application_replacement;
mod builder;
pub use active_application_session::{
    WorthUiActiveApplicationSession, WorthUiActiveInspectionReceipt,
};
pub use active_application_session_identity::WorthUiActiveApplicationSessionIdentity;
pub use active_framework_turn::{
    WorthUiActiveCanvasSpatialFrameCompletion, WorthUiActiveFrameworkTurnCompletion,
    WorthUiActiveFrameworkTurnExecution, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
};
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{WorthUiAppBuilder, WorthUiBuilder, WorthUiQueryViewRegistrationError};
pub use application_replacement::{
    WorthUiApplicationCutoverDenial, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationCutoverRetry, WorthUiApplicationPublicationObservation,
    WorthUiApplicationReplacementLoweringDenial, WorthUiApplicationReplacementOutcome,
    WorthUiApplicationReplacementPreparationDenial, WorthUiApplicationReplacementStagingDenial,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiCandidateInspectionReceipt,
    WorthUiLoweredApplicationReplacement, WorthUiPendingApplicationCutover,
    WorthUiPreparedApplicationReplacement, WorthUiReplacementCandidateSummary,
    WorthUiReplacementPlannedCostEnvelope,
};
pub use builder::CapabilityRegistrationBuilder;
