use worth_query::facade::WorthQueryLiveReadResult;

use crate::{
    WorthServerCompatibilityCachePolicy, WorthServerCompatibilityCertificationBundle,
    WorthServerCompatibilityFileEnvelope, WorthServerConditionalRead,
    WorthServerDirectContextArtifact, WorthServerExternalBasisRequest,
    WorthServerQuerySupportPosture, WorthServerResponseEnvelope,
};

use super::super::project_metadata_read_envelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerReadValidator {
    entity_tag: String,
    canonical_digest: String,
}

impl WorthServerReadValidator {
    pub(crate) fn new(
        result_digest: &str,
        basis_digest: Option<&str>,
        branch_digest: &str,
    ) -> Self {
        let canonical_digest = format!(
            "compat-http-read-validator-v1|result:{result_digest}|basis:{}|branch:{branch_digest}",
            basis_digest.unwrap_or("none"),
        );
        let entity_tag = format!("\"{canonical_digest}\"");
        Self {
            entity_tag,
            canonical_digest,
        }
    }

    pub fn entity_tag(&self) -> &str {
        &self.entity_tag
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Debug)]
pub struct WorthServerCompatibilityRead {
    operation_request: crate::WorthServerOperationRequest,
    plan_proof: crate::WorthServerOperationPlanProof,
    operation_name: String,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    declaration_digest: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    basis_request: WorthServerExternalBasisRequest,
    conditional_read: WorthServerConditionalRead,
    read_result: WorthQueryLiveReadResult,
    response_envelope: WorthServerResponseEnvelope,
    validator: WorthServerReadValidator,
    cache_policy: WorthServerCompatibilityCachePolicy,
    file_envelope: WorthServerCompatibilityFileEnvelope,
    certification_bundle: WorthServerCompatibilityCertificationBundle,
    canonical_digest: String,
}

impl WorthServerCompatibilityRead {
    pub(crate) fn new(
        operation_request: crate::WorthServerOperationRequest,
        plan_proof: crate::WorthServerOperationPlanProof,
        operation_name: impl Into<String>,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        declaration_digest: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        basis_request: WorthServerExternalBasisRequest,
        conditional_read: WorthServerConditionalRead,
        read_result: WorthQueryLiveReadResult,
        response_envelope: WorthServerResponseEnvelope,
        validator: WorthServerReadValidator,
        cache_policy: WorthServerCompatibilityCachePolicy,
        certification_bundle: WorthServerCompatibilityCertificationBundle,
    ) -> Self {
        let operation_name = operation_name.into().trim().to_string();
        let file_envelope = project_metadata_read_envelope(
            &direct_context,
            &operation_name,
            read_result.receipt().result_digest(),
            &response_envelope,
            &support_posture,
            &cache_policy,
        );
        let canonical_digest = format!(
            "worth-server-compat-read-v3:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            operation_request.canonical_digest(),
            operation_name,
            handoff_digest,
            basis_request.canonical_digest(),
            conditional_read.canonical_digest(),
            validator.canonical_digest(),
            cache_policy.canonical_digest(),
            read_result.receipt().result_digest(),
            file_envelope.canonical_digest(),
            certification_bundle.canonical_digest(),
        );
        Self {
            operation_request,
            plan_proof,
            operation_name,
            support_posture,
            workspace_name,
            declaration_digest,
            handoff_digest,
            direct_context,
            basis_request,
            conditional_read,
            read_result,
            response_envelope,
            validator,
            cache_policy,
            file_envelope,
            certification_bundle,
            canonical_digest,
        }
    }

    pub fn operation_request(&self) -> &crate::WorthServerOperationRequest {
        &self.operation_request
    }

    pub fn plan_proof(&self) -> &crate::WorthServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
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

    pub fn direct_context(&self) -> &WorthServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn basis_request(&self) -> &WorthServerExternalBasisRequest {
        &self.basis_request
    }

    pub fn conditional_read(&self) -> &WorthServerConditionalRead {
        &self.conditional_read
    }

    pub fn read_result(&self) -> &WorthQueryLiveReadResult {
        &self.read_result
    }

    pub fn response_envelope(&self) -> &WorthServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn validator(&self) -> &WorthServerReadValidator {
        &self.validator
    }

    pub fn cache_policy(&self) -> &WorthServerCompatibilityCachePolicy {
        &self.cache_policy
    }

    pub fn file_envelope(&self) -> &WorthServerCompatibilityFileEnvelope {
        &self.file_envelope
    }

    pub fn certification_bundle(&self) -> &WorthServerCompatibilityCertificationBundle {
        &self.certification_bundle
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
