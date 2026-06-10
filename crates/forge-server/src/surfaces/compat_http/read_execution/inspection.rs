use forge_query::facade::ForgeQueryUnifiedInspectionResult;

use crate::{
    ForgeServerCompatibilityCachePolicy, ForgeServerCompatibilityCertificationBundle,
    ForgeServerCompatibilityFileEnvelope, ForgeServerDirectContextArtifact,
    ForgeServerExternalBasisRequest, ForgeServerQuerySupportPosture, ForgeServerReadValidator,
    ForgeServerResponseEnvelope,
};

use super::super::project_metadata_inspection_envelope;

#[derive(Debug)]
pub struct ForgeServerCompatibilityInspection {
    operation_name: String,
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    declaration_digest: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    basis_request: ForgeServerExternalBasisRequest,
    inspection_result: ForgeQueryUnifiedInspectionResult,
    response_envelope: ForgeServerResponseEnvelope,
    validator: ForgeServerReadValidator,
    cache_policy: ForgeServerCompatibilityCachePolicy,
    file_envelope: ForgeServerCompatibilityFileEnvelope,
    certification_bundle: ForgeServerCompatibilityCertificationBundle,
    canonical_digest: String,
}

impl ForgeServerCompatibilityInspection {
    pub(crate) fn new(
        operation_name: impl Into<String>,
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        declaration_digest: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        basis_request: ForgeServerExternalBasisRequest,
        inspection_result: ForgeQueryUnifiedInspectionResult,
        response_envelope: ForgeServerResponseEnvelope,
        validator: ForgeServerReadValidator,
        cache_policy: ForgeServerCompatibilityCachePolicy,
        certification_bundle: ForgeServerCompatibilityCertificationBundle,
    ) -> Self {
        let operation_name = operation_name.into().trim().to_string();
        let file_envelope = project_metadata_inspection_envelope(
            &direct_context,
            &operation_name,
            inspection_result.receipt().result_digest(),
            &response_envelope,
            &support_posture,
            &cache_policy,
        );
        let canonical_digest = format!(
            "forge-server-compat-inspection-v3:{}:{}:{}:{}:{}:{}:{}:{}",
            operation_name,
            handoff_digest,
            basis_request.canonical_digest(),
            validator.canonical_digest(),
            cache_policy.canonical_digest(),
            inspection_result.receipt().result_digest(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            operation_name,
            support_posture,
            workspace_name,
            declaration_digest,
            handoff_digest,
            direct_context,
            basis_request,
            inspection_result,
            response_envelope,
            validator,
            cache_policy,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn basis_request(&self) -> &ForgeServerExternalBasisRequest {
        &self.basis_request
    }

    pub fn inspection_result(&self) -> &ForgeQueryUnifiedInspectionResult {
        &self.inspection_result
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn validator(&self) -> &ForgeServerReadValidator {
        &self.validator
    }

    pub fn cache_policy(&self) -> &ForgeServerCompatibilityCachePolicy {
        &self.cache_policy
    }

    pub fn file_envelope(&self) -> &ForgeServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &ForgeServerCompatibilityCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
