use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

#[derive(Default)]
struct ContractOnlyAdapter;

impl WorthUiMeasurementHostAdapter for ContractOnlyAdapter {
    fn observe_measurement(
        &self,
        _request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        unreachable!("headless capability admission denies before observation")
    }
}

impl WorthUiHostMechanicsAdapter for ContractOnlyAdapter {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::missing(Vec::new())
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            0,
        ))
    }
}

fn admit_contract_only_adapter(adapter: &impl WorthUiHostMechanicsAdapter) {
    let _ = adapter.mechanical_host_contract();
    let _ = adapter.mechanical_capability_report();
}

fn main() {
    admit_contract_only_adapter(&ContractOnlyAdapter);
}
