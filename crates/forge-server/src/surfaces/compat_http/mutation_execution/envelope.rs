use crate::{
    ForgeServerDirectContextArtifact, ForgeServerQuerySupportPosture, ForgeServerResponseEnvelope,
};

use super::idempotency::ForgeServerIdempotentReplayReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerCompatibilityMutationEnvelope {
    support_posture: ForgeServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: ForgeServerDirectContextArtifact,
    response_envelope: ForgeServerResponseEnvelope,
    replay_receipt: ForgeServerIdempotentReplayReceipt,
    canonical_digest: String,
}

impl ForgeServerCompatibilityMutationEnvelope {
    pub(crate) fn new(
        support_posture: ForgeServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: ForgeServerDirectContextArtifact,
        response_envelope: ForgeServerResponseEnvelope,
        replay_receipt: ForgeServerIdempotentReplayReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-compat-mutation-envelope-v1|handoff:{}|response:{}|replay:{}",
            handoff_digest,
            response_envelope.canonical_digest(),
            replay_receipt.canonical_digest(),
        );
        Self {
            support_posture,
            workspace_name,
            handoff_digest,
            direct_context,
            response_envelope,
            replay_receipt,
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

    pub fn response_envelope(&self) -> &ForgeServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn replay_receipt(&self) -> &ForgeServerIdempotentReplayReceipt {
        &self.replay_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
