use super::WorthQueryManagedSafePointObservation;
use crate::domain_computation::WorthQueryGraphProviderStepRetainedEvidence;

pub(super) struct WorthQueryManagedYieldSafePoint {
    observation: WorthQueryManagedSafePointObservation,
    checkpoint_available: bool,
    retained: WorthQueryGraphProviderStepRetainedEvidence,
}

impl WorthQueryManagedYieldSafePoint {
    pub(super) const fn new(
        observation: WorthQueryManagedSafePointObservation,
        checkpoint_available: bool,
        retained: WorthQueryGraphProviderStepRetainedEvidence,
    ) -> Self {
        Self {
            observation,
            checkpoint_available,
            retained,
        }
    }

    pub(super) fn observation(&self) -> &WorthQueryManagedSafePointObservation {
        &self.observation
    }

    pub(super) const fn checkpoint_available(&self) -> bool {
        self.checkpoint_available
    }

    pub(super) const fn retained(&self) -> WorthQueryGraphProviderStepRetainedEvidence {
        self.retained
    }
}
