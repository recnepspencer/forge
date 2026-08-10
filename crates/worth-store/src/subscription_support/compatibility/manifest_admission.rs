use super::evidence_validation::require_non_empty;
use super::receipt_witness::SupportCompatibilityReceiptWitness;
use super::version_window::SupportFamilyVersionWindow;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportManifestAdmissionWitness {
    version_window: SupportFamilyVersionWindow,
    manifest_digest: String,
    compatibility_digest: String,
    compatibility_receipt: SupportCompatibilityReceiptWitness,
}

#[allow(dead_code)]
impl SupportManifestAdmissionWitness {
    pub(crate) fn new(
        version_window: SupportFamilyVersionWindow,
        manifest_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let manifest_digest = require_non_empty("manifest", manifest_digest)?;
        Ok(Self {
            compatibility_receipt: SupportCompatibilityReceiptWitness::unbound_legacy(
                &version_window,
                &manifest_digest,
            ),
            version_window,
            manifest_digest,
            compatibility_digest: require_non_empty("compatibility", compatibility_digest)?,
        })
    }

    pub(crate) fn from_compatibility_receipt(
        compatibility_receipt: SupportCompatibilityReceiptWitness,
        compatibility_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let manifest_digest = compatibility_receipt.manifest_digest().to_string();
        Ok(Self {
            version_window: compatibility_receipt.version_window().clone(),
            manifest_digest,
            compatibility_digest: require_non_empty("compatibility", compatibility_digest)?,
            compatibility_receipt,
        })
    }

    pub fn version_window(&self) -> &SupportFamilyVersionWindow {
        &self.version_window
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn compatibility_receipt(&self) -> &SupportCompatibilityReceiptWitness {
        &self.compatibility_receipt
    }
}
