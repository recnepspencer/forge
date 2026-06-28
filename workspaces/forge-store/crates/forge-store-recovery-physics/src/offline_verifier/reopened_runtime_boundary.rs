use crate::BoundedRecoveryReceipt;

use super::runtime_report::{require_matching_counters, require_matching_recovered_state};
use super::{
    PersistedRecoveryArtifactDigest, RecoveryProfileId, ReopenedRuntimeRecoverySession,
    RuntimeRecoveryReportDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReopenedRuntimeBoundaryEvidence {
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    boundary_epoch: u64,
    storage_boundary_id: String,
    fresh_runtime_constructions: u32,
    runtime_cache_reads: u32,
}

impl ReopenedRuntimeBoundaryEvidence {
    pub(crate) fn from_reopened_runtime_transcript(
        transcript: &ReopenedRuntimeBoundaryTranscript,
    ) -> Self {
        Self {
            artifact_digest: transcript.artifact_digest.clone(),
            recovery_profile: transcript.recovery_profile.clone(),
            boundary_epoch: transcript.boundary_epoch,
            storage_boundary_id: transcript.storage_boundary_id.clone(),
            fresh_runtime_constructions: transcript.fresh_runtime_constructions,
            runtime_cache_reads: transcript.runtime_cache_reads,
        }
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub const fn boundary_epoch(&self) -> u64 {
        self.boundary_epoch
    }

    pub fn storage_boundary_id(&self) -> &str {
        &self.storage_boundary_id
    }

    pub const fn fresh_runtime_constructions(&self) -> u32 {
        self.fresh_runtime_constructions
    }

    pub const fn runtime_cache_reads(&self) -> u32 {
        self.runtime_cache_reads
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReopenedRuntimeBoundaryTranscript {
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    boundary_epoch: u64,
    storage_boundary_id: String,
    fresh_runtime_constructions: u32,
    runtime_cache_reads: u32,
}

impl ReopenedRuntimeBoundaryTranscript {
    pub(crate) fn from_reopened_runtime_execution(
        session: &ReopenedRuntimeRecoverySession,
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, RuntimeRecoveryReportDenial> {
        require_matching_recovered_state(receipt, session.admission().report())?;
        require_matching_counters(receipt, session.admission().report())?;
        let runtime_cache_reads = receipt.counters().forbidden_full_store_scans() as u32;
        let fresh_runtime_constructions = u32::from(runtime_cache_reads == 0);
        Ok(Self {
            artifact_digest: session.artifact_digest().clone(),
            recovery_profile: session.recovery_profile().clone(),
            boundary_epoch: session.boundary_epoch(),
            storage_boundary_id: session.storage_boundary_id().to_string(),
            fresh_runtime_constructions,
            runtime_cache_reads,
        })
    }
}
