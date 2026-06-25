mod input;
mod measurement_proof;
mod native_collector;
mod runtime_admission;
mod surface_target;

pub use input::ValidationHostObservationInput;
pub use measurement_proof::{
    ValidationHostFrameObservationOutcome, ValidationHostFrameObservationUnavailable,
    ValidationLiveViewFrameMeasurementProof,
};
pub(crate) use native_collector::collect_live_view_host_observations;
pub use runtime_admission::collect_live_view_host_observations_from_input;
