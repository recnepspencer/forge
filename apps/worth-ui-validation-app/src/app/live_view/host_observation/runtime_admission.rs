use worth_ui::facade::WorthUiRuntimeHost;

use super::input::ValidationHostObservationInput;
use super::measurement_proof::{
    ValidationHostFrameObservationUnavailable, ValidationLiveViewFrameMeasurementProof,
};
use super::surface_target::first_surface_node_id;
use crate::app::live_view::proof::ValidationLiveViewProjectionProof;

pub fn collect_live_view_host_observations_from_input(
    runtime: &WorthUiRuntimeHost,
    proof: Result<&ValidationLiveViewProjectionProof, &str>,
    input: ValidationHostObservationInput,
) -> ValidationLiveViewFrameMeasurementProof {
    let Ok(proof) = proof else {
        return ValidationLiveViewFrameMeasurementProof::unavailable(
            ValidationHostFrameObservationUnavailable::ProjectionUnavailable,
        );
    };
    let mounted = proof.mounted_product_view();
    let Some(surface_node_id) = first_surface_node_id(proof) else {
        return ValidationLiveViewFrameMeasurementProof::unavailable(
            ValidationHostFrameObservationUnavailable::SurfaceNodeUnavailable,
        );
    };
    let draft = input.into_draft(mounted.receipt_digest(), surface_node_id);
    match runtime.admit_host_frame_observations(mounted, draft) {
        Ok(admitted) => {
            let measurement = runtime.measure_mounted_product_view(mounted, admitted.clone());
            ValidationLiveViewFrameMeasurementProof::from_admitted(admitted, measurement)
        }
        Err(denials) => ValidationLiveViewFrameMeasurementProof::denied(denials),
    }
}
