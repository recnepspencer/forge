use worth_query::facade::runtime::WorthQueryLiveReadResult;

use crate::{
    WorthServerDirectContextArtifact, WorthServerQuerySupportPosture, WorthServerResponseEnvelope,
};

#[derive(Debug)]
pub struct WorthServerDirectRead {
    plan_proof: crate::WorthServerOperationPlanProof,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    read_result: WorthQueryLiveReadResult,
    response_envelope: WorthServerResponseEnvelope,
    canonical_digest: String,
}

impl WorthServerDirectRead {
    pub(crate) fn new(
        plan_proof: crate::WorthServerOperationPlanProof,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        read_result: WorthQueryLiveReadResult,
        response_envelope: WorthServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-read-v1:{}:{}",
            handoff_digest,
            read_result.receipt().result_digest()
        );
        Self {
            plan_proof,
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            read_result,
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

    pub fn read_result(&self) -> &WorthQueryLiveReadResult {
        &self.read_result
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &WorthServerResponseEnvelope {
        &self.response_envelope
    }
}
