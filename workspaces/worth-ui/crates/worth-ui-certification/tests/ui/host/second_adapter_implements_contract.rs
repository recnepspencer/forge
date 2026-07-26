use worth_ui::facade::app::WorthUi;
use worth_ui_runtime::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};
use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiMeasurementHostAdapter,
};

#[derive(Default)]
struct AlternateHost;

impl WorthUiMeasurementHostAdapter for AlternateHost {
    fn observe_measurement(&self, _request: &UiHostMeasurementRequest) -> UiHostMeasurementObservationValue {
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
    let app = WorthUi::app()
        .with_host(AlternateHost::default())
        .freeze()
        .expect("application preparation should succeed");
    let session = app.launch().expect("active session claims the adapter lease");
    let _ = session.shutdown();
}
