use super::decoded_s4_recovery_record_set::DecodedS4RecoveryRecords;
use super::s4_recovered_state_projection::project_s4_recovered_physical_state;
use super::s4_recovery_counter_projection::project_s4_recovery_counters;
use super::s4_verifier_conclusion::classify_s4_recovery_record_set;
use super::{
    FreshRuntimeReopenHarnessEvidence, OfflineRecoveryVerificationReport,
    PersistedRecoveryArtifactDigest, PersistedRecoveryArtifacts, RecoveryProfileId,
    RecoveryRuntimeClassification, ReopenedRecoveryArtifactAdmissionDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryOfflineVerifier {
    format_version: String,
    backend_profile: String,
    recovery_profile: RecoveryProfileId,
}

impl RecoveryOfflineVerifier {
    pub fn for_profile(
        format_version: impl Into<String>,
        backend_profile: impl Into<String>,
        recovery_profile: RecoveryProfileId,
    ) -> Self {
        Self {
            format_version: format_version.into(),
            backend_profile: backend_profile.into(),
            recovery_profile,
        }
    }

    pub fn verify_persisted_artifacts(
        &self,
        artifacts: &PersistedRecoveryArtifacts,
    ) -> Result<OfflineRecoveryVerificationReport, RecoveryOfflineVerifierDenial> {
        self.require_profile_match(artifacts)?;
        let digest = PersistedRecoveryArtifactDigest::from_artifacts(artifacts);
        let decoded = DecodedS4RecoveryRecords::from_artifacts(artifacts);
        let conclusion = classify_s4_recovery_record_set(&decoded);
        let recovery_state = project_s4_recovered_physical_state(&decoded);
        let counters = project_s4_recovery_counters(&decoded);
        Ok(OfflineRecoveryVerificationReport::from_offline_inspection(
            digest,
            artifacts.recovery_profile().clone(),
            conclusion,
            RecoveryRuntimeClassification::Recovered,
            recovery_state,
            counters,
            artifacts.records().len(),
            artifacts.total_bytes(),
            decoded.semantic_decode_attempts(),
        ))
    }

    pub fn verify_fresh_runtime_reopen(
        &self,
        artifacts: &PersistedRecoveryArtifacts,
    ) -> Result<FreshRuntimeReopenHarnessEvidence, FreshRuntimeReopenHarnessDenial> {
        let report = self
            .verify_persisted_artifacts(artifacts)
            .map_err(FreshRuntimeReopenHarnessDenial::Verifier)?;
        FreshRuntimeReopenHarnessEvidence::from_persisted_artifact_reopen(report, artifacts)
            .map_err(FreshRuntimeReopenHarnessDenial::Admission)
    }

    fn require_profile_match(
        &self,
        artifacts: &PersistedRecoveryArtifacts,
    ) -> Result<(), RecoveryOfflineVerifierDenial> {
        if self.format_version != artifacts.format_version() {
            return Err(RecoveryOfflineVerifierDenial::FormatVersionMismatch);
        }
        if self.backend_profile != artifacts.backend_profile() {
            return Err(RecoveryOfflineVerifierDenial::BackendProfileMismatch);
        }
        if &self.recovery_profile != artifacts.recovery_profile() {
            return Err(RecoveryOfflineVerifierDenial::RecoveryProfileMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryOfflineVerifierDenial {
    FormatVersionMismatch,
    BackendProfileMismatch,
    RecoveryProfileMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreshRuntimeReopenHarnessDenial {
    Verifier(RecoveryOfflineVerifierDenial),
    Admission(ReopenedRecoveryArtifactAdmissionDenial),
}
