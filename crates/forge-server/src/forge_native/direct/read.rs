use forge_query::facade::ForgeQueryLiveReadResult;

use crate::{
    ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

#[derive(Debug)]
pub struct ForgeServerDirectRead {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    read_result: ForgeQueryLiveReadResult,
    response_envelope: ForgeServerResponseEnvelope,
    canonical_digest: String,
}

impl ForgeServerDirectRead {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        read_result: ForgeQueryLiveReadResult,
        response_envelope: ForgeServerResponseEnvelope,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-direct-read-v1:{}:{}",
            handoff_digest,
            read_result.receipt().result_digest()
        );
        Self {
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            read_result,
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

    pub fn read_result(&self) -> &ForgeQueryLiveReadResult {
        &self.read_result
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }
}
