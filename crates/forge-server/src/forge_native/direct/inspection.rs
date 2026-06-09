use forge_query::facade::ForgeQueryUnifiedInspectionResult;

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

#[derive(Debug)]
pub struct ForgeServerDirectInspection {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    inspection_result: ForgeQueryUnifiedInspectionResult,
    response_envelope: ForgeServerResponseEnvelope,
    canonical_digest: String,
}

impl ForgeServerDirectInspection {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        inspection_result: ForgeQueryUnifiedInspectionResult,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-inspection-v1:{}:{}",
            handoff_digest,
            inspection_result.receipt().result_digest()
        );
        Self {
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            inspection_result,
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

    pub fn inspection_result(&self) -> &ForgeQueryUnifiedInspectionResult {
        &self.inspection_result
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }
}
