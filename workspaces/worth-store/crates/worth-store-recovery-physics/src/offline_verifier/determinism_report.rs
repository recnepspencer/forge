use super::{
    OfflineRecoveryVerificationReport, PersistedRecoveryArtifactDigest,
    RuntimeRecoveryComparisonReport, RuntimeRecoveryReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecoveryNondeterministicMetadata {
    WallClockTimestamp,
    ThreadScheduling,
    EnvironmentFingerprint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryDeterminismClassification {
    Deterministic,
    RuntimeVerifierDisagreement,
    ArtifactDigestMismatch,
    RuntimeClassificationMismatch,
    RecoveredStateMismatch,
    CounterMismatch,
    VerifierConclusionMismatch,
    NondeterministicMetadataMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryDeterminismReport {
    classification: RecoveryDeterminismClassification,
    artifact_digest: PersistedRecoveryArtifactDigest,
    first_runtime_verifier_comparison: RuntimeRecoveryComparisonReport,
    second_runtime_verifier_comparison: RuntimeRecoveryComparisonReport,
    allowed_nondeterministic_metadata: Vec<RecoveryNondeterministicMetadata>,
}

impl RecoveryDeterminismReport {
    pub fn compare_repeated_fresh_recovery(
        first_runtime: &RuntimeRecoveryReport,
        second_runtime: &RuntimeRecoveryReport,
        first_offline: &OfflineRecoveryVerificationReport,
        second_offline: &OfflineRecoveryVerificationReport,
    ) -> Self {
        let first_comparison =
            RuntimeRecoveryComparisonReport::compare(first_runtime, first_offline);
        let second_comparison =
            RuntimeRecoveryComparisonReport::compare(second_runtime, second_offline);
        let classification = classify_repeated_recovery(
            first_runtime,
            second_runtime,
            first_offline,
            second_offline,
            &first_comparison,
            &second_comparison,
        );
        Self {
            classification,
            artifact_digest: first_runtime.artifact_digest().clone(),
            first_runtime_verifier_comparison: first_comparison,
            second_runtime_verifier_comparison: second_comparison,
            allowed_nondeterministic_metadata: first_runtime
                .allowed_nondeterministic_metadata()
                .to_vec(),
        }
    }

    pub const fn classification(&self) -> RecoveryDeterminismClassification {
        self.classification
    }

    pub const fn is_deterministic(&self) -> bool {
        matches!(
            self.classification,
            RecoveryDeterminismClassification::Deterministic
        )
    }

    pub const fn artifact_digest(&self) -> &PersistedRecoveryArtifactDigest {
        &self.artifact_digest
    }

    pub const fn first_runtime_verifier_comparison(&self) -> &RuntimeRecoveryComparisonReport {
        &self.first_runtime_verifier_comparison
    }

    pub const fn second_runtime_verifier_comparison(&self) -> &RuntimeRecoveryComparisonReport {
        &self.second_runtime_verifier_comparison
    }

    pub fn allowed_nondeterministic_metadata(&self) -> &[RecoveryNondeterministicMetadata] {
        &self.allowed_nondeterministic_metadata
    }
}

fn classify_repeated_recovery(
    first_runtime: &RuntimeRecoveryReport,
    second_runtime: &RuntimeRecoveryReport,
    first_offline: &OfflineRecoveryVerificationReport,
    second_offline: &OfflineRecoveryVerificationReport,
    first_comparison: &RuntimeRecoveryComparisonReport,
    second_comparison: &RuntimeRecoveryComparisonReport,
) -> RecoveryDeterminismClassification {
    if !first_comparison.is_equivalent() || !second_comparison.is_equivalent() {
        return RecoveryDeterminismClassification::RuntimeVerifierDisagreement;
    }
    if first_runtime.artifact_digest() != second_runtime.artifact_digest()
        || first_offline.artifact_digest() != second_offline.artifact_digest()
    {
        return RecoveryDeterminismClassification::ArtifactDigestMismatch;
    }
    if first_runtime.classification() != second_runtime.classification() {
        return RecoveryDeterminismClassification::RuntimeClassificationMismatch;
    }
    if first_runtime.recovered_state() != second_runtime.recovered_state() {
        return RecoveryDeterminismClassification::RecoveredStateMismatch;
    }
    if first_runtime.counters() != second_runtime.counters() {
        return RecoveryDeterminismClassification::CounterMismatch;
    }
    if first_offline.conclusion() != second_offline.conclusion() {
        return RecoveryDeterminismClassification::VerifierConclusionMismatch;
    }
    if first_runtime.allowed_nondeterministic_metadata()
        != second_runtime.allowed_nondeterministic_metadata()
    {
        return RecoveryDeterminismClassification::NondeterministicMetadataMismatch;
    }
    RecoveryDeterminismClassification::Deterministic
}
