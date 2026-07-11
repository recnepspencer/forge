use super::{
    OfflineRecoveryVerificationReport, OfflineRecoveryVerifierConclusion, RuntimeRecoveryReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRecoveryComparisonClassification {
    Equivalent,
    ArtifactDigestMismatch,
    FormatVersionMismatch,
    BackendProfileMismatch,
    RecoveryProfileMismatch,
    RuntimeClassificationMismatch,
    RecoveredStateMismatch,
    CounterMismatch,
    VerifierConclusionMismatch,
    OfflineIndependenceViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRecoveryComparisonReport {
    classification: RuntimeRecoveryComparisonClassification,
    runtime_digest: String,
    verifier_digest: String,
    runtime_classification: super::RecoveryRuntimeClassification,
    verifier_conclusion: OfflineRecoveryVerifierConclusion,
}

impl RuntimeRecoveryComparisonReport {
    pub fn compare(
        runtime: &RuntimeRecoveryReport,
        offline: &OfflineRecoveryVerificationReport,
    ) -> Self {
        let classification = classify_runtime_offline_comparison(runtime, offline);
        Self {
            classification,
            runtime_digest: runtime.artifact_digest().value().to_string(),
            verifier_digest: offline.artifact_digest().value().to_string(),
            runtime_classification: runtime.classification(),
            verifier_conclusion: offline.conclusion(),
        }
    }

    pub const fn classification(&self) -> RuntimeRecoveryComparisonClassification {
        self.classification
    }

    pub const fn is_equivalent(&self) -> bool {
        matches!(
            self.classification,
            RuntimeRecoveryComparisonClassification::Equivalent
        )
    }

    pub fn runtime_digest(&self) -> &str {
        &self.runtime_digest
    }

    pub fn verifier_digest(&self) -> &str {
        &self.verifier_digest
    }

    pub const fn runtime_classification(&self) -> super::RecoveryRuntimeClassification {
        self.runtime_classification
    }

    pub const fn verifier_conclusion(&self) -> OfflineRecoveryVerifierConclusion {
        self.verifier_conclusion
    }
}

fn classify_runtime_offline_comparison(
    runtime: &RuntimeRecoveryReport,
    offline: &OfflineRecoveryVerificationReport,
) -> RuntimeRecoveryComparisonClassification {
    if runtime.artifact_digest() != offline.artifact_digest() {
        return RuntimeRecoveryComparisonClassification::ArtifactDigestMismatch;
    }
    if runtime.format_version() != offline.format_version() {
        return RuntimeRecoveryComparisonClassification::FormatVersionMismatch;
    }
    if runtime.backend_profile() != offline.backend_profile() {
        return RuntimeRecoveryComparisonClassification::BackendProfileMismatch;
    }
    if runtime.recovery_profile() != offline.recovery_profile() {
        return RuntimeRecoveryComparisonClassification::RecoveryProfileMismatch;
    }
    if runtime.classification() != offline.verified_runtime_classification() {
        return RuntimeRecoveryComparisonClassification::RuntimeClassificationMismatch;
    }
    if offline.live_runtime_constructions() != 0 || offline.runtime_cache_reads() != 0 {
        return RuntimeRecoveryComparisonClassification::OfflineIndependenceViolation;
    }
    if offline.conclusion() != OfflineRecoveryVerifierConclusion::Verified {
        return RuntimeRecoveryComparisonClassification::VerifierConclusionMismatch;
    }
    if !offline.recovered_state().is_some_and(|offline_state| {
        runtime
            .recovered_state()
            .has_same_recovered_contents(offline_state)
    }) {
        return RuntimeRecoveryComparisonClassification::RecoveredStateMismatch;
    }
    if Some(runtime.counters()) != offline.counters() {
        return RuntimeRecoveryComparisonClassification::CounterMismatch;
    }
    RuntimeRecoveryComparisonClassification::Equivalent
}
