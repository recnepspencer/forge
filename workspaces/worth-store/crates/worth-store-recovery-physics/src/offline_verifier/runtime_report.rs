use crate::{BoundedRecoveryReceipt, RecoveryCounterSnapshot};

use super::{
    FreshRuntimeRecoveryWitness, OfflineRecoveryVerificationReport,
    OfflineRecoveryVerifierConclusion, PersistedRecoveryArtifactDigest,
    RecoveryNondeterministicMetadata, RecoveryProfileId, RecoveryRuntimeClassification,
    ReopenedRecoveryArtifactAdmission, ReopenedRuntimeBoundaryEvidence,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRecoveryReport {
    artifact_digest: PersistedRecoveryArtifactDigest,
    format_version: String,
    backend_profile: String,
    recovery_profile: RecoveryProfileId,
    classification: RecoveryRuntimeClassification,
    recovered_state: crate::RecoveredPhysicalState,
    counters: RecoveryCounterSnapshot,
    fresh_runtime_constructions: u32,
    runtime_cache_reads: u32,
    allowed_nondeterministic_metadata: Vec<RecoveryNondeterministicMetadata>,
}

impl RuntimeRecoveryReport {
    pub fn from_verified_bounded_recovery(
        witness: FreshRuntimeRecoveryWitness,
        offline: &OfflineRecoveryVerificationReport,
        classification: RecoveryRuntimeClassification,
        receipt: &BoundedRecoveryReceipt,
        allowed_nondeterministic_metadata: Vec<RecoveryNondeterministicMetadata>,
    ) -> Result<Self, RuntimeRecoveryReportDenial> {
        require_verified_offline_report(offline)?;
        require_matching_fresh_runtime_witness(&witness, offline, receipt)?;
        require_matching_runtime_classification(classification, offline)?;
        require_matching_recovered_state(receipt, offline)?;
        require_matching_counters(receipt, offline)?;
        Ok(Self {
            format_version: offline.artifact_digest().format_version().to_string(),
            backend_profile: offline.artifact_digest().backend_profile().to_string(),
            artifact_digest: offline.artifact_digest().clone(),
            recovery_profile: offline.recovery_profile().clone(),
            classification,
            recovered_state: receipt.execution().recovered_state().clone(),
            counters: receipt.counters(),
            fresh_runtime_constructions: witness.fresh_runtime_constructions(),
            runtime_cache_reads: witness.runtime_cache_reads(),
            allowed_nondeterministic_metadata: canonical_nondeterministic_metadata(
                allowed_nondeterministic_metadata,
            ),
        })
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

    pub const fn classification(&self) -> RecoveryRuntimeClassification {
        self.classification
    }

    pub const fn recovered_state(&self) -> &crate::RecoveredPhysicalState {
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

    pub fn allowed_nondeterministic_metadata(&self) -> &[RecoveryNondeterministicMetadata] {
        &self.allowed_nondeterministic_metadata
    }
}

fn require_matching_fresh_runtime_witness(
    witness: &FreshRuntimeRecoveryWitness,
    offline: &OfflineRecoveryVerificationReport,
    receipt: &BoundedRecoveryReceipt,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if witness.artifact_digest() == offline.artifact_digest()
        && witness.recovery_profile() == offline.recovery_profile()
        && witness.recovered_state() == receipt.execution().recovered_state()
        && witness.counters() == receipt.counters()
        && witness.fresh_runtime_constructions() > 0
        && witness.runtime_cache_reads() == 0
    {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::FreshRuntimeWitnessMismatch)
}

pub(super) fn require_matching_reopened_boundary(
    admission: &ReopenedRecoveryArtifactAdmission,
    boundary: &ReopenedRuntimeBoundaryEvidence,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if boundary.artifact_digest() == admission.artifact_digest()
        && boundary.recovery_profile() == admission.recovery_profile()
        && boundary.boundary_epoch() > 0
        && !boundary.storage_boundary_id().is_empty()
        && boundary.fresh_runtime_constructions() > 0
        && boundary.runtime_cache_reads() == 0
    {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::FreshRuntimeWitnessMismatch)
}

pub(super) fn require_verified_offline_report(
    offline: &OfflineRecoveryVerificationReport,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if offline.conclusion() == OfflineRecoveryVerifierConclusion::Verified {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::VerifierConclusionMismatch)
}

fn require_matching_runtime_classification(
    classification: RecoveryRuntimeClassification,
    offline: &OfflineRecoveryVerificationReport,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if classification == offline.verified_runtime_classification() {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::RuntimeClassificationMismatch)
}

pub(super) fn require_matching_recovered_state(
    receipt: &BoundedRecoveryReceipt,
    offline: &OfflineRecoveryVerificationReport,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if offline.recovered_state().is_some_and(|offline_state| {
        receipt
            .execution()
            .recovered_state()
            .has_same_recovered_contents(offline_state)
    }) {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::RecoveredStateMismatch)
}

pub(super) fn require_matching_counters(
    receipt: &BoundedRecoveryReceipt,
    offline: &OfflineRecoveryVerificationReport,
) -> Result<(), RuntimeRecoveryReportDenial> {
    if Some(receipt.counters()) == offline.counters() {
        return Ok(());
    }
    Err(RuntimeRecoveryReportDenial::CounterMismatch)
}

fn canonical_nondeterministic_metadata(
    mut metadata: Vec<RecoveryNondeterministicMetadata>,
) -> Vec<RecoveryNondeterministicMetadata> {
    metadata.sort();
    metadata.dedup();
    metadata
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRecoveryReportDenial {
    SameProcessLiveStateReuse,
    FreshRuntimeWitnessMismatch,
    MissingReopenedRuntimeBoundary,
    RuntimeClassificationMismatch,
    RecoveredStateMismatch,
    CounterMismatch,
    VerifierConclusionMismatch,
}
