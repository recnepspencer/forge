use worth_ui_host_contract::{
    UiHostMeasurementObservation, UiHostMeasurementObservationContractDenial,
    UiMeasurementEvidenceFamily, UiMeasurementRequestDenial, UiMeasurementRequestIdentity,
    WorthUiHostCapabilityReport, WorthUiMeasurementHostAdapter,
};

use super::{freeze_measurement_request, UiHostMeasurementNeed};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementExecutionDenial {
    Request(UiMeasurementRequestDenial),
    Observation(UiHostMeasurementObservationContractDenial),
}

pub(crate) fn request_host_measurement<A: WorthUiMeasurementHostAdapter + ?Sized>(
    adapter: &A,
    identity: UiMeasurementRequestIdentity,
    evidence_family: UiMeasurementEvidenceFamily,
    need: UiHostMeasurementNeed,
    capability_report: &WorthUiHostCapabilityReport,
) -> Result<UiHostMeasurementObservation, UiHostMeasurementExecutionDenial> {
    let request = freeze_measurement_request(identity, evidence_family, need, capability_report)
        .map_err(UiHostMeasurementExecutionDenial::Request)?;
    let observation_value = adapter.observe_measurement(&request);
    UiHostMeasurementObservation::from_request(&request, observation_value)
        .map_err(UiHostMeasurementExecutionDenial::Observation)
}
