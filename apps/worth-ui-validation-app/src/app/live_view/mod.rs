mod composition_reload_proof;
mod control_rendering;
mod evidence_rendering;
mod host_observation;
mod interaction_rendering;
mod proof;
mod receipt_color_translation;
mod rendering;
mod source_reload;
mod viewport_adapter;

pub use composition_reload_proof::{
    ValidationLiveViewCompositionRebindDecision, ValidationLiveViewCompositionRebindRow,
    ValidationLiveViewCompositionReloadCounters, ValidationLiveViewCompositionReloadProof,
};
pub use host_observation::{
    collect_live_view_host_observations_from_input, ValidationHostFrameObservationOutcome,
    ValidationHostFrameObservationUnavailable, ValidationHostObservationInput,
    ValidationLiveViewFrameMeasurementProof,
};
pub(crate) use proof::prepare_live_view_document;
pub use proof::ValidationLiveViewProjectionProof;
pub(crate) use rendering::render_live_view_state_proof;

const LIVE_VIEW_ID: &str = "validation.live_view.primitive_proof";
