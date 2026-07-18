//! Application entry and builder surfaces — first lifecycle capability.

mod active_application_admission;
mod active_application_inspection;
mod active_application_session;
mod active_application_session_identity;
mod app;
mod app_builder;
mod application_replacement;
mod builder;
pub use active_application_session::{
    WorthUiActiveApplicationSession, WorthUiActiveFrameworkTurnCompletion,
    WorthUiActiveInspectionReceipt,
};
pub use active_application_session_identity::WorthUiActiveApplicationSessionIdentity;
pub use app::{WorthUi, WorthUiApp};
pub use app_builder::{WorthUiAppBuilder, WorthUiBuilder, WorthUiQueryViewRegistrationError};
pub use application_replacement::{
    WorthUiApplicationCutoverDenial, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationReplacementLoweringDenial, WorthUiApplicationReplacementNoOp,
    WorthUiApplicationReplacementPreparation, WorthUiApplicationReplacementPreparationDenial,
    WorthUiApplicationReplacementStagingDenial, WorthUiCandidateInspectionReceipt,
    WorthUiLoweredApplicationReplacement, WorthUiPendingApplicationCutover,
    WorthUiPreparedApplicationReplacement,
};
pub use builder::CapabilityRegistrationBuilder;
