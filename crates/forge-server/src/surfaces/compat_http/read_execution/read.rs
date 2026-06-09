use forge_query::facade::ForgeQueryLiveReadResult;

use crate::{
    ForgeServerCompatibilityCachePolicy, ForgeServerConditionalRead,
    ForgeServerDirectContextArtifact, ForgeServerExternalBasisRequest,
    ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerReadValidator {
    entity_tag: String,
    canonical_digest: String,
}

impl ForgeServerReadValidator {
    pub(crate) fn new(result_digest: &str, basis_digest: Option<&str>) -> Self {
        let canonical_digest = format!(
            "compat-http-read-validator-v1|result:{result_digest}|basis:{}",
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
pub struct ForgeServerCompatibilityRead {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    declaration_digest: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    basis_request: ForgeServerExternalBasisRequest,
    conditional_read: ForgeServerConditionalRead,
    read_result: ForgeQueryLiveReadResult,
    response_envelope: ForgeServerResponseEnvelope,
    validator: ForgeServerReadValidator,
    cache_policy: ForgeServerCompatibilityCachePolicy,
    canonical_digest: String,
}

impl ForgeServerCompatibilityRead {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        declaration_digest: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        basis_request: ForgeServerExternalBasisRequest,
        conditional_read: ForgeServerConditionalRead,
        read_result: ForgeQueryLiveReadResult,
        response_envelope: ForgeServerResponseEnvelope,
        validator: ForgeServerReadValidator,
        cache_policy: ForgeServerCompatibilityCachePolicy,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-read-v1:{}:{}:{}:{}:{}:{}",
            handoff_digest,
            basis_request.canonical_digest(),
            conditional_read.canonical_digest(),
            validator.canonical_digest(),
            cache_policy.canonical_digest(),
            read_result.receipt().result_digest(),
        );
        Self {
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
            canonical_digest,
        }
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

    pub fn conditional_read(&self) -> &ForgeServerConditionalRead {
        &self.conditional_read
    }

    pub fn read_result(&self) -> &ForgeQueryLiveReadResult {
        &self.read_result
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

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
