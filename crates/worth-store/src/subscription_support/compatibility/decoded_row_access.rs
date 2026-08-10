use super::super::stable_digest;
use super::evidence_validation::require_non_empty;
use super::manifest_admission::SupportManifestAdmissionWitness;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDecodedRowSemanticAccess {
    admission_witness: SupportManifestAdmissionWitness,
    semantic_digest: String,
}

impl SupportDecodedRowSemanticAccess {
    pub(crate) fn from_manifest_admission(
        admission_witness: SupportManifestAdmissionWitness,
        semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(Self {
            admission_witness,
            semantic_digest: require_non_empty("decoded semantic row", semantic_digest)?,
        })
    }

    pub fn admission_witness(&self) -> &SupportManifestAdmissionWitness {
        &self.admission_witness
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }
}

#[allow(dead_code)]
fn _digest_for_semantic_access(
    access: &SupportDecodedRowSemanticAccess,
) -> Result<String, StoreError> {
    stable_digest(access)
}
