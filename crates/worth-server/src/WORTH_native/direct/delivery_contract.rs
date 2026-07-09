use worth_query::facade::WorthQueryRuntimeDownstreamDeliveryContract;

use crate::{WorthServerDirectContextArtifact, WorthServerQuerySupportPosture};

use super::{WorthServerDirectDeliveryRequest, WorthServerDirectLeaseDeclaration};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectDeliveryContract {
    plan_proof: crate::WorthServerOperationPlanProof,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    lease_declaration: WorthServerDirectLeaseDeclaration,
    request: WorthServerDirectDeliveryRequest,
    downstream_delivery_contract: WorthQueryRuntimeDownstreamDeliveryContract,
    response_envelope: crate::WorthServerResponseEnvelope,
    canonical_digest: String,
}

impl WorthServerDirectDeliveryContract {
    pub(crate) fn new(
        plan_proof: crate::WorthServerOperationPlanProof,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        lease_declaration: WorthServerDirectLeaseDeclaration,
        request: WorthServerDirectDeliveryRequest,
        downstream_delivery_contract: WorthQueryRuntimeDownstreamDeliveryContract,
        response_envelope: crate::WorthServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-delivery-contract-v1|handoff:{handoff_digest}|lease:{}|request:{}|contract:{}",
            lease_declaration.canonical_digest(),
            request.canonical_digest(),
            downstream_delivery_contract.contract_for_reporting(),
        );
        Self {
            plan_proof,
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

    pub fn plan_proof(&self) -> &crate::WorthServerOperationPlanProof {
        &self.plan_proof
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }

    pub fn direct_context(&self) -> &WorthServerDirectContextArtifact {
        &self.direct_context
    }

    pub fn lease_declaration(&self) -> &WorthServerDirectLeaseDeclaration {
        &self.lease_declaration
    }

    pub fn request(&self) -> &WorthServerDirectDeliveryRequest {
        &self.request
    }

    pub fn downstream_delivery_contract(&self) -> &WorthQueryRuntimeDownstreamDeliveryContract {
        &self.downstream_delivery_contract
    }

    pub fn response_envelope(&self) -> &crate::WorthServerResponseEnvelope {
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
            WorthServerQuerySupportPosture::RuntimeBackedResumeSupported { .. }
        )
    }

    pub fn durable_resume_supported(&self) -> bool {
        matches!(
            self.support_posture,
            WorthServerQuerySupportPosture::DurableResumeSupported { .. }
        )
    }

    pub fn runtime_resume_support_posture(
        &self,
    ) -> worth_query::facade::WorthQueryLowerRuntimeSupportPosture {
        self.support_posture.runtime_resume_support_posture()
    }

    pub fn durable_resume_support_posture(
        &self,
    ) -> worth_query::facade::WorthQueryLowerRuntimeSupportPosture {
        self.support_posture.durable_resume_support_posture()
    }
}
