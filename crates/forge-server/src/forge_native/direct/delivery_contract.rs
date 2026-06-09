use forge_query::facade::ForgeQueryRuntimeDownstreamDeliveryContract;

use crate::{ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture};

use super::{ForgeServerDirectDeliveryRequest, ForgeServerDirectLeaseDeclaration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectDeliveryContract {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    lease_declaration: ForgeServerDirectLeaseDeclaration,
    request: ForgeServerDirectDeliveryRequest,
    downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
    response_envelope: crate::ForgeServerResponseEnvelope,
    canonical_digest: String,
}

impl ForgeServerDirectDeliveryContract {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        lease_declaration: ForgeServerDirectLeaseDeclaration,
        request: ForgeServerDirectDeliveryRequest,
        downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
        response_envelope: crate::ForgeServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-delivery-contract-v1|handoff:{handoff_digest}|lease:{}|request:{}|contract:{}",
            lease_declaration.canonical_digest(),
            request.canonical_digest(),
            downstream_delivery_contract.contract_digest(),
        );
        Self {
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            lease_declaration,
            request,
            downstream_delivery_contract,
            response_envelope,
            canonical_digest,
        }
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &ForgeServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn lease_declaration(&self) -> &ForgeServerDirectLeaseDeclaration {
        &self.lease_declaration
    }

    pub fn request(&self) -> &ForgeServerDirectDeliveryRequest {
        &self.request
    }

    pub fn downstream_delivery_contract(&self) -> &ForgeQueryRuntimeDownstreamDeliveryContract {
        &self.downstream_delivery_contract
    }

    pub fn response_envelope(&self) -> &crate::ForgeServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn delivery_contract_digest(&self) -> &str {
        self.canonical_digest()
    }

    pub fn runtime_backed_resume_supported(&self) -> bool {
        matches!(
            self.support_posture,
            ForgeServerQuerySupportPosture::RuntimeBackedResumeSupported { .. }
        )
    }

    pub fn durable_resume_supported(&self) -> bool {
        matches!(
            self.support_posture,
            ForgeServerQuerySupportPosture::DurableResumeSupported { .. }
        )
    }

    pub fn runtime_resume_support_posture(
        &self,
    ) -> forge_query::facade::ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture.runtime_resume_support_posture()
    }

    pub fn durable_resume_support_posture(
        &self,
    ) -> forge_query::facade::ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture.durable_resume_support_posture()
    }
}
