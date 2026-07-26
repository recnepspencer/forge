use std::sync::Arc;

use super::{
    WorthQueryGraphProviderMemorySnapshot, WorthQueryGraphProviderStepArtifactEvidence,
    WorthQueryGraphProviderStepDispositionKind, WorthQueryGraphProviderStepFailureEvidence,
};
use crate::domain_computation::WorthQueryGraphReadMaterial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryGraphProviderStepCompletion {
    Continue,
    Complete,
    Failed,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphProviderStepRetainedEvidence {
    provider_memory_arena_identity: u64,
    provider_allocation_count: u64,
    provider_bytes: u64,
    projection_bytes: u64,
    artifact_bytes: u64,
    total_bytes: u64,
}

impl WorthQueryGraphProviderStepRetainedEvidence {
    pub(super) fn new(
        provider_memory: WorthQueryGraphProviderMemorySnapshot,
        projection_bytes: usize,
        artifact_bytes: usize,
    ) -> Self {
        let projection_bytes = u64::try_from(projection_bytes).unwrap_or(u64::MAX);
        let artifact_bytes = u64::try_from(artifact_bytes).unwrap_or(u64::MAX);
        let provider_bytes = provider_memory.retained_bytes();
        Self {
            provider_memory_arena_identity: provider_memory.arena_identity(),
            provider_allocation_count: provider_memory.retained_allocation_count(),
            provider_bytes,
            projection_bytes,
            artifact_bytes,
            total_bytes: provider_bytes
                .saturating_add(projection_bytes)
                .saturating_add(artifact_bytes),
        }
    }

    pub const fn provider_memory_arena_identity(self) -> u64 {
        self.provider_memory_arena_identity
    }

    pub const fn provider_allocation_count(self) -> u64 {
        self.provider_allocation_count
    }

    pub const fn provider_bytes(self) -> u64 {
        self.provider_bytes
    }

    pub const fn projection_bytes(self) -> u64 {
        self.projection_bytes
    }

    pub const fn artifact_bytes(self) -> u64 {
        self.artifact_bytes
    }

    pub const fn total_bytes(self) -> u64 {
        self.total_bytes
    }
}

#[derive(Debug, PartialEq)]
pub struct WorthQueryGraphProviderStepReport {
    completion: WorthQueryGraphProviderStepCompletion,
    provider_receipt: Option<Arc<str>>,
    completed_work_units: u64,
    attempted_effect_count: u64,
    applied_effect_count: u64,
    peak_scratch_bytes: u64,
    retained: WorthQueryGraphProviderStepRetainedEvidence,
    projection: Option<WorthQueryGraphReadMaterial>,
    artifacts: WorthQueryGraphProviderStepArtifactEvidence,
    checkpoint_available: bool,
    failure: Option<WorthQueryGraphProviderStepFailureEvidence>,
}

impl WorthQueryGraphProviderStepReport {
    pub(super) fn from_disposition(
        disposition: super::WorthQueryGraphProviderStepDisposition,
        completed_work_units: u64,
        attempted_effect_count: u64,
        applied_effect_count: u64,
        peak_scratch_bytes: u64,
        retained: WorthQueryGraphProviderStepRetainedEvidence,
        projection: Option<WorthQueryGraphReadMaterial>,
        artifacts: WorthQueryGraphProviderStepArtifactEvidence,
        checkpoint_available: bool,
    ) -> Self {
        let completion = match disposition.kind() {
            WorthQueryGraphProviderStepDispositionKind::Continue => {
                WorthQueryGraphProviderStepCompletion::Continue
            }
            WorthQueryGraphProviderStepDispositionKind::Complete => {
                WorthQueryGraphProviderStepCompletion::Complete
            }
        };
        Self {
            completion,
            provider_receipt: disposition.into_provider_receipt(),
            completed_work_units,
            attempted_effect_count,
            applied_effect_count,
            peak_scratch_bytes,
            retained,
            projection,
            artifacts,
            checkpoint_available,
            failure: None,
        }
    }

    pub(super) fn failed(
        completed_work_units: u64,
        attempted_effect_count: u64,
        applied_effect_count: u64,
        peak_scratch_bytes: u64,
        retained: WorthQueryGraphProviderStepRetainedEvidence,
        projection: Option<WorthQueryGraphReadMaterial>,
        artifacts: WorthQueryGraphProviderStepArtifactEvidence,
        checkpoint_available: bool,
        failure: WorthQueryGraphProviderStepFailureEvidence,
    ) -> Self {
        Self {
            completion: WorthQueryGraphProviderStepCompletion::Failed,
            provider_receipt: None,
            completed_work_units,
            attempted_effect_count,
            applied_effect_count,
            peak_scratch_bytes,
            retained,
            projection,
            artifacts,
            checkpoint_available,
            failure: Some(failure),
        }
    }

    pub const fn completed_work_units(&self) -> u64 {
        self.completed_work_units
    }

    pub const fn attempted_effect_count(&self) -> u64 {
        self.attempted_effect_count
    }

    pub const fn applied_effect_count(&self) -> u64 {
        self.applied_effect_count
    }

    pub const fn peak_scratch_bytes(&self) -> u64 {
        self.peak_scratch_bytes
    }

    pub const fn retained_bytes(&self) -> u64 {
        self.retained.total_bytes()
    }

    pub const fn retained_evidence(&self) -> WorthQueryGraphProviderStepRetainedEvidence {
        self.retained
    }

    pub const fn has_projection_chunk(&self) -> bool {
        self.projection.is_some()
    }

    pub const fn artifact_evidence(&self) -> WorthQueryGraphProviderStepArtifactEvidence {
        self.artifacts
    }

    pub const fn checkpoint_available(&self) -> bool {
        self.checkpoint_available
    }

    pub const fn failure(&self) -> Option<&WorthQueryGraphProviderStepFailureEvidence> {
        self.failure.as_ref()
    }

    pub(crate) const fn completion(&self) -> WorthQueryGraphProviderStepCompletion {
        self.completion
    }

    pub(crate) fn provider_receipt(&self) -> Option<&str> {
        self.provider_receipt.as_deref()
    }

    pub(crate) fn take_projection(&mut self) -> Option<WorthQueryGraphReadMaterial> {
        self.projection.take()
    }
}
