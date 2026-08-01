#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiIntentExecutionCapacityCertificationProfile {
    application_attempts: usize,
    destination_attempts: usize,
    provider_attempts: usize,
    intent_attempts: usize,
    retained_payload_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentExecutionReservationCertificationMetrics {
    active_attempts: usize,
    prepared_attempts: usize,
    active_occupancy: usize,
    retained_payload_bytes: usize,
    recovering_attempts: usize,
    consequence_pending_attempts: usize,
}

pub trait WorthUiIntentExecutionReservationCertificationExt {
    fn install_intent_execution_capacity_for_certification(
        &mut self,
        profile: UiIntentExecutionCapacityCertificationProfile,
    ) -> bool;

    fn intent_execution_reservation_metrics_for_certification(
        &self,
    ) -> UiIntentExecutionReservationCertificationMetrics;

    fn exhaust_intent_execution_reservation_identities_for_certification(&mut self) -> usize;
}

impl UiIntentExecutionCapacityCertificationProfile {
    pub const fn bounded(
        application_attempts: usize,
        destination_attempts: usize,
        provider_attempts: usize,
        intent_attempts: usize,
        retained_payload_bytes: usize,
    ) -> Option<Self> {
        if crate::runtime::intent_execution::UiIntentExecutionCapacity::bounded_for_certification(
            application_attempts,
            destination_attempts,
            provider_attempts,
            intent_attempts,
            retained_payload_bytes,
        )
        .is_none()
        {
            return None;
        }
        Some(Self {
            application_attempts,
            destination_attempts,
            provider_attempts,
            intent_attempts,
            retained_payload_bytes,
        })
    }

    fn into_capacity(self) -> crate::runtime::intent_execution::UiIntentExecutionCapacity {
        crate::runtime::intent_execution::UiIntentExecutionCapacity::bounded_for_certification(
            self.application_attempts,
            self.destination_attempts,
            self.provider_attempts,
            self.intent_attempts,
            self.retained_payload_bytes,
        )
        .expect("certification profile was bounded at construction")
    }
}

impl UiIntentExecutionReservationCertificationMetrics {
    pub const fn active_attempts(self) -> usize {
        self.active_attempts
    }

    pub const fn prepared_attempts(self) -> usize {
        self.prepared_attempts
    }

    pub const fn active_occupancy(self) -> usize {
        self.active_occupancy
    }

    pub const fn retained_payload_bytes(self) -> usize {
        self.retained_payload_bytes
    }

    pub const fn recovering_attempts(self) -> usize {
        self.recovering_attempts
    }

    pub const fn consequence_pending_attempts(self) -> usize {
        self.consequence_pending_attempts
    }
}

impl WorthUiIntentExecutionReservationCertificationExt
    for crate::facade::WorthUiActiveApplicationSession
{
    fn install_intent_execution_capacity_for_certification(
        &mut self,
        profile: UiIntentExecutionCapacityCertificationProfile,
    ) -> bool {
        self.install_intent_execution_capacity_for_certification(profile.into_capacity())
    }

    fn intent_execution_reservation_metrics_for_certification(
        &self,
    ) -> UiIntentExecutionReservationCertificationMetrics {
        let census = self.intent_execution_census_for_certification();
        UiIntentExecutionReservationCertificationMetrics {
            active_attempts: census.active_attempts,
            prepared_attempts: census.prepared_attempts,
            active_occupancy: census.active_occupancy,
            retained_payload_bytes: census.retained_payload_bytes,
            recovering_attempts: census.recovering_attempts,
            consequence_pending_attempts: census.consequence_pending_attempts,
        }
    }

    fn exhaust_intent_execution_reservation_identities_for_certification(&mut self) -> usize {
        self.exhaust_intent_execution_reservation_identities_for_certification()
    }
}
