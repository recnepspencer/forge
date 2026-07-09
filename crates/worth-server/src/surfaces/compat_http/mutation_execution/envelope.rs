use crate::{
    WorthServerDirectContextArtifact, WorthServerQuerySupportPosture, WorthServerResponseEnvelope,
};

use super::idempotency::WorthServerIdempotentReplayReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityMutationEnvelope {
    support_posture: WorthServerQuerySupportPosture,
    workspace_name: String,
    handoff_digest: String,
    direct_context: WorthServerDirectContextArtifact,
    response_envelope: WorthServerResponseEnvelope,
    replay_receipt: WorthServerIdempotentReplayReceipt,
    canonical_digest: String,
}

impl WorthServerCompatibilityMutationEnvelope {
    pub(crate) fn new(
        support_posture: WorthServerQuerySupportPosture,
        workspace_name: String,
        handoff_digest: String,
        direct_context: WorthServerDirectContextArtifact,
        response_envelope: WorthServerResponseEnvelope,
        replay_receipt: WorthServerIdempotentReplayReceipt,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-compat-mutation-envelope-v1|handoff:{}|response:{}|replay:{}",
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

    pub fn response_envelope(&self) -> &WorthServerResponseEnvelope {
        &self.response_envelope
    }

    pub fn replay_receipt(&self) -> &WorthServerIdempotentReplayReceipt {
        &self.replay_receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
