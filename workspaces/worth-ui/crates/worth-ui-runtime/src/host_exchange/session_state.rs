mod measurement;
mod observation;

/// Host-originated observation and measurement authority for one session.
pub(crate) struct WorthUiHostExchangeSessionState {
    observations: super::observation_report_validation::UiHostObservationReportValidation,
    measurements: super::measurement_admission::UiHostMeasurementAdmission,
}

impl WorthUiHostExchangeSessionState {
    pub(crate) fn new(
        observation_capacity: super::observation_report_validation::UiHostObservationCapacity,
    ) -> Self {
        Self {
            observations:
                super::observation_report_validation::UiHostObservationReportValidation::new(
                    observation_capacity,
                ),
            measurements: Default::default(),
        }
    }

    pub(crate) fn shutdown(&mut self) {
        self.observations.shutdown();
        self.measurements.shutdown();
    }
}
