use crate::{BoundedRecoveryReceipt, RecoveredPhysicalState, RecoveryCounterSnapshot};

use super::runtime_report::{
    require_matching_counters, require_matching_recovered_state,
    require_matching_reopened_boundary, require_verified_offline_report,
};
use super::{
    PersistedRecoveryArtifactDigest, RecoveryProfileId, ReopenedRecoveryArtifactAdmission,
    ReopenedRuntimeBoundaryEvidence, ReopenedRuntimeBoundaryTranscript,
    RuntimeRecoveryReportDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryRuntimeClassification {
    Recovered,
    RecoveryBlocked,
    PartialPublicationRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeRecoveryExecution {
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    recovered_state: RecoveredPhysicalState,
    counters: RecoveryCounterSnapshot,
    fresh_runtime_constructions: u32,
    runtime_cache_reads: u32,
}

impl FreshRuntimeRecoveryExecution {
    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn from_certification_runtime_evidence(
        artifact_digest: PersistedRecoveryArtifactDigest,
        recovery_profile: RecoveryProfileId,
        recovered_state: RecoveredPhysicalState,
        counters: RecoveryCounterSnapshot,
        fresh_runtime_constructions: u32,
        runtime_cache_reads: u32,
    ) -> Self {
        Self {
            artifact_digest,
            recovery_profile,
            recovered_state,
            counters,
            fresh_runtime_constructions,
            runtime_cache_reads,
        }
    }

    pub(crate) fn from_store_recovery_execution(
        admission: &ReopenedRecoveryArtifactAdmission,
        transcript: &ReopenedRuntimeBoundaryTranscript,
        receipt: &BoundedRecoveryReceipt,
    ) -> Result<Self, RuntimeRecoveryReportDenial> {
        let offline = admission.report();
        let boundary =
            ReopenedRuntimeBoundaryEvidence::from_reopened_runtime_transcript(transcript);
        require_verified_offline_report(offline)?;
        require_matching_reopened_boundary(admission, &boundary)?;
        require_matching_recovered_state(receipt, offline)?;
        require_matching_counters(receipt, offline)?;
        Ok(Self {
            artifact_digest: admission.artifact_digest().clone(),
            recovery_profile: admission.recovery_profile().clone(),
            recovered_state: receipt.execution().recovered_state().clone(),
            counters: receipt.counters(),
            fresh_runtime_constructions: boundary.fresh_runtime_constructions(),
            runtime_cache_reads: boundary.runtime_cache_reads(),
        })
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub const fn recovered_state(&self) -> &RecoveredPhysicalState {
        &self.recovered_state
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub const fn fresh_runtime_constructions(&self) -> u32 {
        self.fresh_runtime_constructions
    }

    pub const fn runtime_cache_reads(&self) -> u32 {
        self.runtime_cache_reads
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshRuntimeRecoveryWitness {
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    recovered_state: RecoveredPhysicalState,
    counters: RecoveryCounterSnapshot,
    fresh_runtime_constructions: u32,
    runtime_cache_reads: u32,
}

impl FreshRuntimeRecoveryWitness {
    pub(super) fn from_fresh_runtime_execution(execution: FreshRuntimeRecoveryExecution) -> Self {
        Self {
            artifact_digest: execution.artifact_digest,
            recovery_profile: execution.recovery_profile,
            recovered_state: execution.recovered_state,
            counters: execution.counters,
            fresh_runtime_constructions: execution.fresh_runtime_constructions,
            runtime_cache_reads: execution.runtime_cache_reads,
        }
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub const fn recovered_state(&self) -> &RecoveredPhysicalState {
        &self.recovered_state
    }

    pub const fn counters(&self) -> RecoveryCounterSnapshot {
        self.counters
    }

    pub const fn fresh_runtime_constructions(&self) -> u32 {
        self.fresh_runtime_constructions
    }

    pub const fn runtime_cache_reads(&self) -> u32 {
        self.runtime_cache_reads
    }
}
