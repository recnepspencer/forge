#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryWorkflowArtifactRegistryEvidence {
    production_generation: u64,
    produced_artifact_count: usize,
    retained_artifact_count: usize,
    disposed_artifact_count: usize,
    retained_bytes: usize,
    provider_release_complete_count: usize,
    provider_release_pending_count: usize,
    provider_release_recovery_required_count: usize,
}

impl WorthQueryWorkflowArtifactRegistryEvidence {
    pub(super) const fn new(
        production_generation: super::WorthQueryArtifactProductionGeneration,
        produced_artifact_count: usize,
        retained_artifact_count: usize,
        disposed_artifact_count: usize,
        retained_bytes: usize,
        provider_release_complete_count: usize,
        provider_release_pending_count: usize,
        provider_release_recovery_required_count: usize,
    ) -> Self {
        Self {
            production_generation: production_generation.ordinal(),
            produced_artifact_count,
            retained_artifact_count,
            disposed_artifact_count,
            retained_bytes,
            provider_release_complete_count,
            provider_release_pending_count,
            provider_release_recovery_required_count,
        }
    }

    pub const fn production_generation(self) -> u64 {
        self.production_generation
    }

    pub const fn produced_artifact_count(self) -> usize {
        self.produced_artifact_count
    }

    pub const fn retained_artifact_count(self) -> usize {
        self.retained_artifact_count
    }

    pub const fn disposed_artifact_count(self) -> usize {
        self.disposed_artifact_count
    }

    pub const fn retained_bytes(self) -> usize {
        self.retained_bytes
    }

    pub const fn provider_release_complete_count(self) -> usize {
        self.provider_release_complete_count
    }

    pub const fn provider_release_pending_count(self) -> usize {
        self.provider_release_pending_count
    }

    pub const fn provider_release_recovery_required_count(self) -> usize {
        self.provider_release_recovery_required_count
    }
}
