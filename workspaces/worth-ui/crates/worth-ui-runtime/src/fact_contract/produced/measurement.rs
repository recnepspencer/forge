#[derive(Debug)]
pub struct UiMeasurementChangedFact {
    result: crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult,
}

impl UiMeasurementChangedFact {
    pub(crate) fn new(
        result: crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult,
    ) -> Self {
        Self { result }
    }

    pub fn result(
        &self,
    ) -> &crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult {
        &self.result
    }
}
