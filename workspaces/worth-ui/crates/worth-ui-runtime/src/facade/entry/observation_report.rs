use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(crate) fn host_observation_ingress(
        &self,
    ) -> crate::facade::observation_report::WorthUiHostObservationIngress {
        self.host_exchange.observation_ingress()
    }

    pub(crate) fn validate_host_observation_batch(
        &mut self,
        batch: worth_ui_host_contract::UiHostObservationBatch,
    ) -> crate::facade::observation_report::UiHostObservationReportOutcome {
        self.host_exchange.validate_observation_batch(
            batch,
            self.host_session.identity().as_u64(),
            self.host_session.protocol(),
            self.mounted.observation_validation_basis(),
        )
    }

    pub(crate) fn retained_host_observation_report_count(&self) -> usize {
        self.host_exchange.retained_observation_report_count()
    }

    pub(crate) fn retained_host_observation_byte_count(&self) -> usize {
        self.host_exchange.retained_observation_byte_count()
    }

    pub(crate) fn quarantined_host_observation_batch_count(&self) -> usize {
        self.host_exchange.quarantined_observation_batch_count()
    }

    pub(crate) fn quarantined_host_observation_byte_count(&self) -> usize {
        self.host_exchange.quarantined_observation_byte_count()
    }

    pub(crate) fn host_observation_work_report(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationWorkReport {
        self.host_exchange.observation_work_report()
    }

    pub(crate) fn validate_enqueued_host_observation_batches(
        &mut self,
    ) -> Box<[crate::facade::observation_report::UiHostObservationReportOutcome]> {
        self.host_exchange
            .drain_observation_ingress()
            .into_iter()
            .map(|batch| self.validate_host_observation_batch(batch))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
