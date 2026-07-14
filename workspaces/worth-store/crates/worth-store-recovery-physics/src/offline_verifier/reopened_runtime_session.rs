use super::{
    PersistedRecoveryArtifactDigest, RecoveryProfileId, ReopenedRecoveryArtifactAdmission,
    RuntimeRecoveryReportDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReopenedRuntimeRecoverySession {
    admission: ReopenedRecoveryArtifactAdmission,
    artifact_digest: PersistedRecoveryArtifactDigest,
    recovery_profile: RecoveryProfileId,
    boundary_epoch: u64,
    storage_boundary_id: String,
}

impl ReopenedRuntimeRecoverySession {
    pub(super) fn from_fresh_runtime_driver(
        admission: &ReopenedRecoveryArtifactAdmission,
    ) -> Result<Self, RuntimeRecoveryReportDenial> {
        Ok(Self {
            admission: admission.clone(),
            artifact_digest: admission.artifact_digest().clone(),
            recovery_profile: admission.recovery_profile().clone(),
            boundary_epoch: 1,
            storage_boundary_id: "store-recovery-physics-reopened-runtime".to_string(),
        })
    }

    pub const fn admission(&self) -> &ReopenedRecoveryArtifactAdmission {
        &self.admission
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
}
