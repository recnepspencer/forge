use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiLastValidObservation,
    WorthUiReloadStormIterationOutcome,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormOrderedTruth {
    initial_active_plan_digest: u64,
    final_active_plan_digest: u64,
    final_last_valid_plan_digest: u64,
    prepared_preservation_plan_digests: Vec<u64>,
    denied_preservation_plan_digests: Vec<u64>,
}

impl WorthUiReloadStormOrderedTruth {
    pub(crate) fn from_outcomes(
        initial_active: WorthUiActiveRuntimeObservation,
        final_active: WorthUiActiveRuntimeObservation,
        final_last_valid: WorthUiLastValidObservation,
        outcomes: &[WorthUiReloadStormIterationOutcome],
    ) -> Self {
        let mut prepared_preservation_plan_digests = Vec::new();
        let mut denied_preservation_plan_digests = Vec::new();
        for outcome in outcomes {
            match outcome {
                WorthUiReloadStormIterationOutcome::PreparedPendingCutover(iteration) => {
                    prepared_preservation_plan_digests.push(iteration.active_plan_digest_after());
                }
                WorthUiReloadStormIterationOutcome::DeniedPreserved(iteration) => {
                    denied_preservation_plan_digests.push(iteration.active_plan_digest_after());
                }
            }
        }
        Self {
            initial_active_plan_digest: initial_active.active_plan_digest(),
            final_active_plan_digest: final_active.active_plan_digest(),
            final_last_valid_plan_digest: final_last_valid.active_plan_digest(),
            prepared_preservation_plan_digests,
            denied_preservation_plan_digests,
        }
    }

    pub fn initial_active_plan_digest(&self) -> u64 {
        self.initial_active_plan_digest
    }

    pub fn final_active_plan_digest(&self) -> u64 {
        self.final_active_plan_digest
    }

    pub fn final_last_valid_plan_digest(&self) -> u64 {
        self.final_last_valid_plan_digest
    }

    pub fn prepared_preservation_plan_digests(&self) -> &[u64] {
        &self.prepared_preservation_plan_digests
    }

    pub fn denied_preservation_plan_digests(&self) -> &[u64] {
        &self.denied_preservation_plan_digests
    }
}
