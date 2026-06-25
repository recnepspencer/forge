use worth_ui::facade::WorthUiRuntimeHost;

use super::input::ValidationHostObservationInput;
use super::measurement_proof::ValidationLiveViewFrameMeasurementProof;
use super::runtime_admission::collect_live_view_host_observations_from_input;
use crate::app::live_view::proof::ValidationLiveViewProjectionProof;

pub(crate) struct ValidationHostFrameObservation {
    area: egui::Rect,
    proof: ValidationLiveViewFrameMeasurementProof,
}

pub(crate) fn collect_live_view_host_observations(
    ui: &egui::Ui,
    runtime: &WorthUiRuntimeHost,
    proof: Result<&ValidationLiveViewProjectionProof, &str>,
) -> ValidationHostFrameObservation {
    let area = ui.available_rect_before_wrap();
    let input = ValidationHostObservationInput::new(
        area.width(),
        area.height(),
        ui.ctx().cumulative_pass_nr(),
    )
    .with_dpi_scale(ui.ctx().pixels_per_point());
    let proof = collect_live_view_host_observations_from_input(runtime, proof, input);
    ValidationHostFrameObservation { area, proof }
}

impl ValidationHostFrameObservation {
    pub(crate) fn area(&self) -> egui::Rect {
        self.area
    }

    pub(crate) fn measurement_proof(&self) -> &ValidationLiveViewFrameMeasurementProof {
        &self.proof
    }
}
