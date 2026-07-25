use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiHostMeasurementRequest, UiMeasurementRequestIdentity,
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
    let UiHostMeasurementRequest { identity: _, .. } = request;
}
