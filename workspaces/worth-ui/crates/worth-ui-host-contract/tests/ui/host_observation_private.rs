use worth_ui_host_contract::{
    UiHostObservation, UiHostObservationValue, UiMeasurementEvidenceFamily,
    UiMeasurementRequest, UiMeasurementRequestIdentity, UiViewportExtentObservation,
    UiViewportExtentRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
};

fn main() {
    let capabilities = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui());
    let request = UiMeasurementRequest::viewport_extent(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &capabilities,
    )
    .expect("request should admit");
    let observation = UiHostObservation::from_request(
        &request,
        UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
            width: 1.0,
            height: 1.0,
        }),
    )
    .expect("observation should match request");
    let UiHostObservation { request: _, .. } = observation;
}
