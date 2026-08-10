use super::super::SubscriptionSupportArtifactId;
use super::import_admission::SupportImportAdmissionWitness;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportedSupportNotResumableReport {
    import_admission: SupportImportAdmissionWitness,
    denial_reason: String,
    missing_basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl ImportedSupportNotResumableReport {
    pub(super) fn new(
        import_admission: SupportImportAdmissionWitness,
        denial_reason: String,
        missing_basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    ) -> Self {
        Self {
            import_admission,
            denial_reason,
            missing_basis_artifact_ids,
        }
    }

    pub fn import_admission(&self) -> &SupportImportAdmissionWitness {
        &self.import_admission
    }

    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    pub fn missing_basis_count(&self) -> u64 {
        self.missing_basis_artifact_ids.len() as u64
    }
}
