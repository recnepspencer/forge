use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub fn host_observation_ingress(
        &self,
    ) -> crate::facade::observation_report::WorthUiHostObservationIngress {
        self.host_observations.ingress()
    }

    pub fn validate_host_observation_batch(
        &mut self,
        batch: worth_ui_host_contract::UiHostObservationBatch,
    ) -> crate::facade::observation_report::UiHostObservationReportOutcome {
        self.host_observations.validate(
            batch,
            crate::host_exchange::observation_report_validation::UiHostObservationValidationContext {
                host_session: self.host_session.identity().as_u64(),
                protocol: self.host_session.protocol(),
                retention: &self.mounted_retention,
                presentation: &self.mounted_presentation,
            },
        )
    }

    pub fn retained_host_observation_report_count(&self) -> usize {
        self.host_observations.retained_report_count()
    }

    pub fn retained_host_observation_byte_count(&self) -> usize {
        self.host_observations.retained_byte_count()
    }

    pub fn quarantined_host_observation_batch_count(&self) -> usize {
        self.host_observations.quarantined_batch_count()
    }

    pub fn quarantined_host_observation_byte_count(&self) -> usize {
        self.host_observations.quarantined_byte_count()
    }

    pub fn host_observation_work_report(
        &self,
    ) -> crate::facade::observation_report::UiHostObservationWorkReport {
        self.host_observations.work_report()
    }

    pub fn validate_enqueued_host_observation_batches(
        &mut self,
    ) -> Box<[crate::facade::observation_report::UiHostObservationReportOutcome]> {
        self.host_observations
            .drain_ingress()
            .into_iter()
            .map(|batch| self.validate_host_observation_batch(batch))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
