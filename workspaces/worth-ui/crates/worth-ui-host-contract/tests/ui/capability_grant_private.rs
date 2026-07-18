use worth_ui_host_contract::{UiMeasurementCapabilityGrant, WorthUiHostCapability};

fn main() {
    let _grant = UiMeasurementCapabilityGrant {
        required_capabilities: Box::<[WorthUiHostCapability]>::default(),
    };
}
