use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiHostMeasurementRequest, UiMeasurementRequestIdentity,
    UiViewportExtentRequest, WorthUiHostCapability, WorthUiHostCapabilityReport,
};

fn main() {
    let capabilities = WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::ViewportObservation,
    ]);
    let request = UiHostMeasurementRequest::viewport_extent(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &capabilities,
    )
    .expect("request should admit");
    let UiHostMeasurementRequest { identity: _, .. } = request;
}
