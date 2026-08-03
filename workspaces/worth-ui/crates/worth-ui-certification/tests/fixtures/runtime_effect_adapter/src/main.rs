use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
use worth_ui_runtime::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};

#[derive(Default)]
struct AlternateHost;

impl WorthUiMeasurementHostAdapter for AlternateHost {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("headless capability report denies measurement construction")
    }
}

impl WorthUiOperationalHostAdapter for AlternateHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::missing(Vec::new())
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            0,
        ))
    }
}

fn main() {
    let host = AlternateHost::default();
    let _ = host.operational_host_contract();
    let _ = host.operational_capability_report();
}
