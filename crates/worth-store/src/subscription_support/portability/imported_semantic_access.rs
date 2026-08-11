use super::super::classification_error;
use super::evidence_validation::require_non_empty;
use super::import_admission::SupportImportAdmissionWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ImportedSupportSemanticAccess {
    import_admission: SupportImportAdmissionWitness,
    imported_semantic_digest: String,
}

impl ImportedSupportSemanticAccess {
    pub(crate) fn from_import_admission(
        import_admission: SupportImportAdmissionWitness,
        imported_semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if import_admission
            .source_identity_preservation_digest()
            .is_none()
        {
            return Err(classification_error(
                "subscription-support semantic import access requires source identity-preservation evidence",
            ));
        }
        Ok(Self {
            import_admission,
            imported_semantic_digest: require_non_empty(
                "imported support semantic",
                imported_semantic_digest,
            )?,
        })
    }

    pub fn import_admission(&self) -> &SupportImportAdmissionWitness {
        &self.import_admission
    }

    pub fn imported_semantic_digest(&self) -> &str {
        &self.imported_semantic_digest
    }
}
