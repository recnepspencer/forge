use super::capsule_manifest::CapsuleSupportManifest;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportImportAdmissionWitness {
    manifest_digest: String,
    footprint_digest: String,
    target_admission_digest: String,
    source_identity_preservation_digest: Option<String>,
}

impl SupportImportAdmissionWitness {
    pub(crate) fn new(
        manifest: &CapsuleSupportManifest,
        target_admission_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            manifest_digest: manifest.manifest_digest().to_string(),
            footprint_digest: manifest.footprint().footprint_digest().to_string(),
            target_admission_digest: require_non_empty(
                "target import admission",
                target_admission_digest,
            )?,
            source_identity_preservation_digest: None,
        })
    }

    pub(crate) fn exact(
        manifest: &CapsuleSupportManifest,
        target_admission_digest: impl Into<String>,
        source_identity_preservation_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            manifest_digest: manifest.manifest_digest().to_string(),
            footprint_digest: manifest.footprint().footprint_digest().to_string(),
            target_admission_digest: require_non_empty(
                "target import admission",
                target_admission_digest,
            )?,
            source_identity_preservation_digest: Some(require_non_empty(
                "source identity preservation",
                source_identity_preservation_digest,
            )?),
        })
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }

    pub fn target_admission_digest(&self) -> &str {
        &self.target_admission_digest
    }

    pub fn source_identity_preservation_digest(&self) -> Option<&str> {
        self.source_identity_preservation_digest.as_deref()
    }
}
