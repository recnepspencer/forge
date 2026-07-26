use super::WorthUiHostExchangeSessionState;

impl WorthUiHostExchangeSessionState {
    pub(crate) fn measurement_ingress(
        &self,
    ) -> crate::facade::measurement_exchange::WorthUiHostMeasurementIngress {
        self.measurements.ingress()
    }

    pub(crate) fn begin_measurement(
        &mut self,
        intent: crate::facade::measurement_exchange::UiHostMeasurementIntent,
        current: crate::host_exchange::measurement_admission::UiHostMeasurementCurrentTruth,
        capability_report: &worth_ui_host_contract::WorthUiHostCapabilityReport,
        now: u64,
    ) -> crate::facade::measurement_exchange::UiHostMeasurementOutcome {
        self.measurements
            .begin(intent, current, capability_report, now)
    }

    pub(crate) fn complete_measurement(
        &mut self,
        observation: worth_ui_host_contract::UiHostMeasurementObservation,
        current: crate::host_exchange::measurement_admission::UiHostMeasurementCurrentTruth,
        now: u64,
    ) -> crate::facade::measurement_exchange::UiHostMeasurementOutcome {
        self.measurements.complete(observation, current, now)
    }

    pub(crate) fn cancel_measurement(
        &mut self,
        identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    ) -> crate::facade::measurement_exchange::UiHostMeasurementOutcome {
        self.measurements.cancel(identity)
    }

    pub(crate) fn expire_measurements(
        &mut self,
        now: u64,
    ) -> Box<[crate::facade::measurement_exchange::UiHostMeasurementOutcome]> {
        self.measurements.expire(now)
    }

    pub(crate) fn pending_measurement_count(&self) -> usize {
        self.measurements.pending_count()
    }

    pub(crate) fn pending_measurement_binding(
        &self,
        identity: worth_ui_host_contract::UiMeasurementRequestIdentity,
    ) -> Option<Option<worth_ui_host_contract::UiSurfaceBindingGeneration>> {
        self.measurements.pending_binding(identity)
    }

    pub(crate) fn pending_measurement_bytes(&self) -> usize {
        self.measurements.pending_bytes()
    }

    pub(crate) fn drain_measurement_ingress(
        &self,
    ) -> Vec<crate::facade::measurement_exchange::UiHostMeasurementCompletion> {
        self.measurements.drain_ingress()
    }
}
