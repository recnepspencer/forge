//! Host observation lane: freeze request → observe → normalize → admit freshness.

mod host_session;
mod result_construction;
mod transitions;
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
#[cfg(test)]
#[path = "tests/mod.rs"]
pub(crate) mod tests;

pub(crate) use host_session::request_host_measurement;
pub use transitions::UiHostMeasurementFreshnessWitness;
pub(crate) use transitions::{
    admit_fresh_host_evidence, construct_freshness_witness, normalize_host_observation,
    observe_host_measurement,
};
pub use host_session::UiHostMeasurementExecutionDenial;
pub use measurement_assumption_profile::UiHostMeasurementAssumptionProfile;
pub use measurement_evidence_boundary::{
    admit_current_host_measurement_evidence, collect_host_measurement_evidence,
};
pub(crate) use measurement_invalidation::invalidate_stale_host_measurement_evidence;
pub use measurement_normalization_context::UiHostMeasurementNormalizationContext;
pub use measurement_request_boundary::{freeze_measurement_request, UiHostMeasurementNeed};
pub(crate) use measurement_result_boundary::normalize_host_measurement_evidence;
pub use measurement_result_denial::{
    UiHostMeasurementEvidenceDenial, UiHostMeasurementInvalidationReason,
    UiHostMeasurementNormalizationDenial,
};