use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiHostSessionReleaseOutcome,
    UiHostSessionReleaseReceipt, UiMeasurementRequestFamily, UiPortalAnchorRectObservation,
    WorthUiHostCapability, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostMechanicsAdapter, WorthUiMeasurementHostAdapter,
};

/// Fixed record-only host with one authored portal-anchor observation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorthUiHeadlessPortalAnchorHost {
    observation: UiPortalAnchorRectObservation,
}

impl WorthUiHeadlessPortalAnchorHost {
    pub const fn new(observation: UiPortalAnchorRectObservation) -> Self {
        Self { observation }
    }
}

impl WorthUiMeasurementHostAdapter for WorthUiHeadlessPortalAnchorHost {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::PortalAnchorRect => {
                UiHostMeasurementObservationValue::PortalAnchorRect(self.observation)
            }
            family => unreachable!("portal-anchor host does not admit {family:?}"),
        }
    }
}

impl WorthUiHostMechanicsAdapter for WorthUiHeadlessPortalAnchorHost {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::PortalAnchorObservation])
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
