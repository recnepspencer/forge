use crate::domain_computation::provider_session::graph_provider::bounded_step::{
    WorthQueryGraphProviderMemorySnapshot, WorthQueryGraphProviderStepRetainedEvidence,
};

#[derive(Default)]
pub(super) struct WorthQueryManagedProviderRetentionLedger {
    provider_bytes: usize,
    projection_bytes: usize,
    artifact_bytes: usize,
    peak_bytes: usize,
}

impl WorthQueryManagedProviderRetentionLedger {
    pub(super) fn admit_step(
        &mut self,
        evidence: WorthQueryGraphProviderStepRetainedEvidence,
        prior_provider_bytes: usize,
    ) {
        self.provider_bytes =
            prior_provider_bytes.saturating_add(as_usize(evidence.provider_bytes()));
        self.projection_bytes = as_usize(evidence.projection_bytes());
        self.record_peak();
    }

    pub(super) fn observe_active_provider(
        &mut self,
        memory: WorthQueryGraphProviderMemorySnapshot,
        prior_provider_bytes: usize,
    ) {
        self.provider_bytes =
            prior_provider_bytes.saturating_add(as_usize(memory.retained_bytes()));
        self.record_peak();
    }

    pub(super) fn reconcile_provider_liabilities(&mut self, provider_bytes: usize) {
        self.provider_bytes = provider_bytes;
        self.record_peak();
    }

    pub(super) fn release_projection(&mut self, released_bytes: usize) -> bool {
        let Some(remaining) = self.projection_bytes.checked_sub(released_bytes) else {
            return false;
        };
        self.projection_bytes = remaining;
        true
    }

    pub(super) fn settle_artifacts(&mut self, retained_bytes: usize) {
        self.artifact_bytes = retained_bytes;
        self.record_peak();
    }

    pub(super) fn current_bytes(&self) -> usize {
        self.provider_bytes
            .saturating_add(self.projection_bytes)
            .saturating_add(self.artifact_bytes)
    }

    pub(super) const fn provider_bytes(&self) -> usize {
        self.provider_bytes
    }

    pub(super) const fn peak_bytes(&self) -> usize {
        self.peak_bytes
    }

    fn record_peak(&mut self) {
        self.peak_bytes = self.peak_bytes.max(self.current_bytes());
    }
}

fn as_usize(bytes: u64) -> usize {
    usize::try_from(bytes).unwrap_or(usize::MAX)
}
