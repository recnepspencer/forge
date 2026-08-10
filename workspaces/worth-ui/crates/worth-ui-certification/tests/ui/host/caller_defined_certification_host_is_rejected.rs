use worth_ui::facade::app::WorthUi;
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
        unreachable!("the caller-defined host must never be admitted")
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
    let application = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("host-neutral application should prepare");
    let _ =
        worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_recorder(
            application,
            AlternateHost,
        );
}
