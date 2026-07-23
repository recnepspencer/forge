use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiMeasurementHostAdapter, WorthUiOperationalHostAdapter,
};

struct AlternateHost;

impl WorthUiMeasurementHostAdapter for AlternateHost {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        unreachable!("headless capability report denies measurement construction")
    }
}

impl WorthUiOperationalHostAdapter for AlternateHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::headless())
    }

    fn consume_output(
        &self,
        _output: &worth_ui_host_contract::WorthUiHostOutputEnvelope,
    ) -> worth_ui_host_contract::WorthUiHostOutputDisposition {
        worth_ui_host_contract::WorthUiHostOutputDisposition::Consumed
    }
}

fn main() {
    let _ = AlternateHost.operational_host_contract();
    let _ = AlternateHost.operational_capability_report();
}
