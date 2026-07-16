use crate::{RecoveredPhysicalState, RecoveryCounterSnapshot};

use super::{PersistedRecoveryArtifactDigest, RecoveryProfileId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineRecoveryVerifierConclusion {
    Verified,
    CorruptRecord,
    IncompletePhysicalRecordSet,
    AmbiguousPhysicalRecordSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineRecoveryVerificationReport {
    artifact_digest: PersistedRecoveryArtifactDigest,
    format_version: String,
    backend_profile: String,
    recovery_profile: RecoveryProfileId,
    conclusion: OfflineRecoveryVerifierConclusion,
    verified_runtime_classification: super::RecoveryRuntimeClassification,
    recovered_state: Option<RecoveredPhysicalState>,
    counters: Option<RecoveryCounterSnapshot>,
    inspected_records: usize,
    inspected_bytes: usize,
    semantic_decode_attempts: u32,
    live_runtime_constructions: u32,
    runtime_cache_reads: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct OfflineInspectionMeasurements {
    inspected_records: usize,
    inspected_bytes: usize,
    semantic_decode_attempts: u32,
}

impl OfflineInspectionMeasurements {
    pub(super) const fn new(
        inspected_records: usize,
        inspected_bytes: usize,
        semantic_decode_attempts: u32,
    ) -> Self {
        Self {
            inspected_records,
            inspected_bytes,
            semantic_decode_attempts,
        }
    }
}

impl OfflineRecoveryVerificationReport {
    pub(super) fn from_offline_inspection(
        artifact_digest: PersistedRecoveryArtifactDigest,
        recovery_profile: RecoveryProfileId,
        conclusion: OfflineRecoveryVerifierConclusion,
        verified_runtime_classification: super::RecoveryRuntimeClassification,
        recovered_state: Option<RecoveredPhysicalState>,
        counters: Option<RecoveryCounterSnapshot>,
        measurements: OfflineInspectionMeasurements,
    ) -> Self {
        Self {
            format_version: artifact_digest.format_version().to_string(),
            backend_profile: artifact_digest.backend_profile().to_string(),
            artifact_digest,
            recovery_profile,
            conclusion,
            verified_runtime_classification,
            recovered_state,
            counters,
            inspected_records: measurements.inspected_records,
            inspected_bytes: measurements.inspected_bytes,
            semantic_decode_attempts: measurements.semantic_decode_attempts,
            live_runtime_constructions: 0,
            runtime_cache_reads: 0,
        }
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub fn format_version(&self) -> &str {
        &self.format_version
    }

    pub fn backend_profile(&self) -> &str {
        &self.backend_profile
    }

    pub const fn recovery_profile(&self) -> &RecoveryProfileId {
        &self.recovery_profile
    }

    pub const fn conclusion(&self) -> OfflineRecoveryVerifierConclusion {
        self.conclusion
    }

    pub const fn verified_runtime_classification(&self) -> super::RecoveryRuntimeClassification {
        self.verified_runtime_classification
    }

    pub const fn recovered_state(&self) -> Option<&RecoveredPhysicalState> {
        self.recovered_state.as_ref()
    }

    pub const fn counters(&self) -> Option<RecoveryCounterSnapshot> {
        self.counters
    }

    pub const fn inspected_records(&self) -> usize {
        self.inspected_records
    }

    pub const fn inspected_bytes(&self) -> usize {
        self.inspected_bytes
    }

    pub const fn semantic_decode_attempts(&self) -> u32 {
        self.semantic_decode_attempts
    }

    pub const fn live_runtime_constructions(&self) -> u32 {
        self.live_runtime_constructions
    }

    pub const fn runtime_cache_reads(&self) -> u32 {
        self.runtime_cache_reads
    }
}
