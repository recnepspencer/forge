use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, CommittedPatchSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest,
};

use super::RuntimeBridgeRelationalSource;
use crate::capabilities::CommitEnvelopeSource;
use crate::presentation::bridge::identities::parse_bridge_commit_identity;
use crate::presentation::bridge::patch_envelopes::commit_envelope_to_bridge_envelope;

impl CommittedPatchSource for RuntimeBridgeRelationalSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let commit_id = parse_bridge_commit_identity(request.commit_identity())?;
        let envelope = self.runtime.commit_envelope(commit_id).ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational runtime has no authoritative commit envelope for bridge commit `{}`",
                commit_id.0
            ))
        })?;

        Ok(commit_envelope_to_bridge_envelope(envelope))
    }
}
