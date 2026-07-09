use worth_query::facade::WorthQueryUnifiedInspectionResult;

use crate::{
    WorthServerDirectContextArtifact, WorthServerQuerySupportPosture, WorthServerResponseEnvelope,
};

#[derive(Debug)]
pub struct WorthServerDirectInspection {
    plan_proof: crate::WorthServerOperationPlanProof,
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    inspection_result: WorthQueryUnifiedInspectionResult,
    response_envelope: WorthServerResponseEnvelope,
    canonical_digest: String,
}

impl WorthServerDirectInspection {
    pub(crate) fn new(
        plan_proof: crate::WorthServerOperationPlanProof,
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        inspection_result: WorthQueryUnifiedInspectionResult,
        response_envelope: WorthServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-direct-inspection-v1:{}:{}",
            handoff_digest,
            inspection_result.receipt().result_digest()
        );
        Self {
            plan_proof,
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            inspection_result,
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

    pub fn inspection_result(&self) -> &WorthQueryUnifiedInspectionResult {
        &self.inspection_result
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &WorthServerResponseEnvelope {
        &self.response_envelope
    }
}
