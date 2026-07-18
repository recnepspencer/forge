//! Public Worth UI runtime surfaces ordered by lifecycle capability and authority class.
//!
//! Lifecycle order: entry → lifecycle → registry → runtime_handoff → boundaries → evidence → host → inspection

pub mod admission;
mod app_inspection_closeout;
pub mod declaration;
pub mod entry;
pub mod evidence;
pub mod graph;
pub mod host_observation;
mod host_session_authority;
mod inspection;
pub mod inspection_bridge;
mod inspection_observation;
mod inspection_receipt;
pub mod lifecycle;
mod measurement_inspection_evidence;
#[cfg(test)]
mod measurement_inspection_query_denial_tests;
#[cfg(test)]
mod measurement_inspection_test_support;
#[cfg(test)]
mod measurement_inspection_tests;
pub mod obligations;
pub mod prepared_application_authority;
pub mod query_binding;
pub mod registry;
mod retained_obligation_registry;
#[doc(hidden)]
pub mod runtime_exports;
pub mod runtime_handoff;

pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;

pub use crate::runtime::exports::*;
pub use entry::{
    CapabilityRegistrationBuilder, WorthUi, WorthUiActiveApplicationSession,
    WorthUiActiveApplicationSessionIdentity, WorthUiActiveFrameworkTurnCompletion,
    WorthUiActiveInspectionReceipt, WorthUiApp, WorthUiAppBuilder, WorthUiApplicationCutoverDenial,
    WorthUiApplicationCutoverReceipt, WorthUiApplicationReplacementLoweringDenial,
    WorthUiApplicationReplacementNoOp, WorthUiApplicationReplacementPreparation,
    WorthUiApplicationReplacementPreparationDenial, WorthUiApplicationReplacementStagingDenial,
    WorthUiBuilder, WorthUiCandidateInspectionReceipt, WorthUiLoweredApplicationReplacement,
    WorthUiPendingApplicationCutover, WorthUiPreparedApplicationReplacement,
};
pub(crate) use host_session_authority::WorthUiHostSessionAuthority;
pub use host_session_authority::{
    WorthUiHostMeasurementCapability, WorthUiHostMeasurementSessionInput,
    WorthUiHostSessionIdentity,
};
pub use lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
pub use worth_ui_dsl::WorthUiDslPackage;
