use forge_query::facade::ForgeQueryRuntimeStateSnapshot;

use crate::{
    ForgeServerCompatibilityCachePolicy, ForgeServerDirectContextArtifact,
    ForgeServerExternalBasisRequest, ForgeServerQuerySupportPosture, ForgeServerReadValidator,
    ForgeServerResponseEnvelope,
};

#[derive(Debug)]
pub struct ForgeServerCompatibilityState {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    declaration_digest: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    basis_request: ForgeServerExternalBasisRequest,
    runtime_state: ForgeQueryRuntimeStateSnapshot,
    response_envelope: ForgeServerResponseEnvelope,
    validator: ForgeServerReadValidator,
    cache_policy: ForgeServerCompatibilityCachePolicy,
    canonical_digest: String,
}

impl ForgeServerCompatibilityState {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        declaration_digest: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        basis_request: ForgeServerExternalBasisRequest,
        runtime_state: ForgeQueryRuntimeStateSnapshot,
        response_envelope: ForgeServerResponseEnvelope,
        validator: ForgeServerReadValidator,
        cache_policy: ForgeServerCompatibilityCachePolicy,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-state-v1:{}:{}:{}:{}:{}",
            handoff_digest,
            basis_request.canonical_digest(),
            validator.canonical_digest(),
            cache_policy.canonical_digest(),
            runtime_state.state_digest().terminal_projection_for_reporting(),
        );
        Self {
            support_posture,
            workspace_name,
            declaration_digest,
            handoff_digest,
            direct_context,
            basis_request,
            runtime_state,
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

    pub fn runtime_state(&self) -> &ForgeQueryRuntimeStateSnapshot {
        &self.runtime_state
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
