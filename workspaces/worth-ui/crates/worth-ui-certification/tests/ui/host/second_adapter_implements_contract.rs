use worth_ui::facade::app::WorthUi;
use worth_ui::facade::dsl::WorthUiDslPackage;
use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiMeasurementHostAdapter, WorthUiOperationalHostAdapter,
};

struct AlternateHost;

impl WorthUiMeasurementHostAdapter for AlternateHost {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        unreachable!("headless configuration denies measurement construction")
    }
}

impl WorthUiOperationalHostAdapter for AlternateHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::headless())
    }
}

fn main() {
    let _ = WorthUi::app()
        .with_dsl_package(WorthUiDslPackage::named("certification.host"))
        .with_host(AlternateHost)
        .freeze().expect("application preparation should succeed");
}
