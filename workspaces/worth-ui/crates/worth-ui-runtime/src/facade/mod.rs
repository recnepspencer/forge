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
pub mod host_observation;
mod host_session_authority;
mod inspection;
pub mod inspection_bridge;
mod inspection_observation;
mod inspection_receipt;
pub mod lifecycle;
mod measurement_inspection_evidence;
#[cfg(test)]
mod measurement_inspection_test_support;
#[cfg(test)]
mod measurement_inspection_tests;
pub mod obligations;
pub mod prepared_application_authority;
pub mod query_binding;
pub mod registry;
mod retained_obligation_registry;
pub mod runtime_handoff;
pub mod source_ingress;

pub(crate) use inspection::foreign_evidence_refs_for_obligation_record;

pub use entry::{
    CapabilityRegistrationBuilder, WorthUi, WorthUiActiveApplicationSession,
    WorthUiActiveApplicationSessionIdentity, WorthUiActiveCanvasSpatialFrameCompletion,
    WorthUiActiveFrameworkTurnCompletion, WorthUiActiveFrameworkTurnExecution,
    WorthUiActiveInspectionReceipt, WorthUiActiveOrdinaryFrameCompletion,
    WorthUiActiveRealtimeFrameCompletion, WorthUiActiveVirtualizedDataFrameCompletion,
    WorthUiAllocationCatalogActivationDenial, WorthUiApp, WorthUiAppBuilder,
    WorthUiApplicationCutoverDenial, WorthUiApplicationCutoverReceipt,
    WorthUiApplicationReplacementLoweringDenial, WorthUiApplicationReplacementOutcome,
    WorthUiApplicationReplacementPreparationDenial, WorthUiApplicationReplacementStagingDenial,
    WorthUiApplicationSemanticNoOpReceipt, WorthUiBuilder, WorthUiCandidateInspectionReceipt,
    WorthUiLoweredApplicationReplacement, WorthUiPendingApplicationCutover,
    WorthUiPreparedApplicationReplacement, WorthUiReplacementCandidateSummary,
    WorthUiReplacementPlannedCostEnvelope,
};
pub(crate) use host_session_authority::WorthUiHostPlanBinding;
pub(crate) use host_session_authority::WorthUiHostSessionAuthority;
pub use host_session_authority::{
    WorthUiHostMeasurementCapability, WorthUiHostMeasurementSessionInput,
    WorthUiHostSessionIdentity,
};
pub use lifecycle::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
pub use worth_ui_dsl::WorthUiDslPackage;
