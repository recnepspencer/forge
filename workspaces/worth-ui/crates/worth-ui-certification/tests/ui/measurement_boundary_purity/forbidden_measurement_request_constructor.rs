use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiHostMeasurementRequest, UiMeasurementRequestIdentity,
    UiViewportExtentRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
};

fn main() {
    let report = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui());
    let _ = UiHostMeasurementRequest::final_layout_size(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiViewportExtentRequest,
        &report,
    );
}
