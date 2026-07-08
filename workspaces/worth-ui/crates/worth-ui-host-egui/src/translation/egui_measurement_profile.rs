use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_runtime::facade::host_observation::UiHostMeasurementAssumptionProfile;

pub fn egui_measurement_adapter_profile_digest() -> u64 {
    stable_text_digest("worth-ui-host-egui:measurement-profile:v1")
}

pub fn egui_measurement_assumption_profile(
    capability_report: &WorthUiHostCapabilityReport,
    viewport_assumption_digest: u64,
    dpi_assumption_digest: u64,
    font_assumption_digest: u64,
) -> UiHostMeasurementAssumptionProfile {
    UiHostMeasurementAssumptionProfile::from_capability_report(
        capability_report,
        viewport_assumption_digest,
        dpi_assumption_digest,
        font_assumption_digest,
        egui_measurement_adapter_profile_digest(),
    )
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
