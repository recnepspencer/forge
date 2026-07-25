use worth_ui_host_contract::{
    UiHostMeasurementObservation, UiHostMeasurementObservationValue, UiMeasurementEvidenceFamily,
    UiHostMeasurementRequest, UiMeasurementRequestIdentity, UiViewportExtentObservation,
    UiViewportExtentRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
};

fn main() {
    let capabilities = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui());
    let request = UiHostMeasurementRequest::viewport_extent(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &capabilities,
    )
    .expect("request should admit");
    let observation = UiHostMeasurementObservation::from_request(
        &request,
        UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
            width: 1.0,
            height: 1.0,
        }),
    )
    .expect("observation should match request");
    let UiHostMeasurementObservation { request: _, .. } = observation;
}
