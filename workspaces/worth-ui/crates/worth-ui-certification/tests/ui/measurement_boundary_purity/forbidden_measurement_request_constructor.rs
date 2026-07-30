use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiHostMeasurementRequest, UiMeasurementRequestIdentity,
    UiViewportExtentRequest, WorthUiHostCapability, WorthUiHostCapabilityReport,
};

fn main() {
    let report = WorthUiHostCapabilityReport::available(vec![
        WorthUiHostCapability::ViewportObservation,
    ]);
    let _ = UiHostMeasurementRequest::final_layout_size(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &report,
    );
}
