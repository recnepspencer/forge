//! Host observation lane: freeze request → observe → normalize → admit freshness.

mod admitted_measurement;
mod host_session;
mod measurement_assumption_profile;
mod measurement_evidence_boundary;
mod measurement_invalidation;
#[cfg(test)]
mod measurement_invalidation_tests;
mod measurement_normalization_context;
mod measurement_request_boundary;
#[cfg(test)]
mod measurement_request_boundary_tests;
mod measurement_result_boundary;
#[cfg(test)]
mod measurement_result_boundary_tests;
mod measurement_result_denial;
mod preview_paint;
mod result_construction;
#[cfg(test)]
#[path = "tests/mod.rs"]
pub(crate) mod tests;
mod transitions;

pub use admitted_measurement::UiAdmittedHostMeasurement;
pub(crate) use host_session::request_host_measurement;
pub use host_session::UiHostMeasurementExecutionDenial;
pub use measurement_assumption_profile::UiHostMeasurementAssumptionProfile;
pub(crate) use measurement_evidence_boundary::UiHostMeasurementSourceAuthority;
pub use measurement_evidence_boundary::{
    admit_current_host_measurement_evidence, WorthUiHostMeasurementCollector,
};
pub(crate) use measurement_invalidation::invalidate_stale_host_measurement_evidence;
pub use measurement_normalization_context::{
    UiHostMeasurementNormalizationContext, UiPortalAnchorCoordinateSpacePosture,
};
pub use measurement_request_boundary::{freeze_measurement_request, UiHostMeasurementNeed};
pub(crate) use measurement_result_boundary::normalize_host_measurement_evidence;
pub use measurement_result_denial::{
    UiHostMeasurementEvidenceDenial, UiHostMeasurementInvalidationReason,
    UiHostMeasurementNormalizationDenial,
};
pub(crate) use preview_paint::seal_preview_paint_input;
pub use preview_paint::{
    UiHostPreviewDiscardReason, UiHostPreviewPaintContext, UiHostPreviewPaintDenial,
    UiHostPreviewPaintDenialReport, UiHostPreviewPaintDiscardReport, UiHostPreviewPaintDisposition,
    UiHostPreviewPaintGeometry, UiHostPreviewPaintInput, UiHostPreviewPaintReceipt,
    WorthUiPreviewPaintHost,
};
pub(crate) use transitions::{
    admit_fresh_host_evidence, construct_freshness_witness, normalize_host_observation,
    observe_host_measurement, UiHostMeasurementFreshnessWitness,
};
