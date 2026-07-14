use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, RelationalBridgeSourceError, TruthBranchHeadSource,
    TruthBranchIdentity,
};

use super::RuntimeBridgeRelationalSource;
use crate::capabilities::CommitEnvelopeSource;
use crate::presentation::bridge::patch_envelopes::commit_envelope_to_bridge_envelope;

impl TruthBranchHeadSource for RuntimeBridgeRelationalSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let branch_id = crate::history::data::BranchId(
            branch_identity
                .relational_branch_id()
                .ok_or_else(|| {
                    RelationalBridgeSourceError::new(
                        "unsupported relational bridge branch identity",
                    )
                })?
                .to_string(),
        );
        let history = self.runtime.history();
        let head = history.branch_head(&branch_id).ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational runtime has no branch head for `{}`",
                branch_id.0
            ))
        })?;
        let envelope = self.runtime.commit_envelope(head.commit_id).ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational runtime has no authoritative commit envelope for branch head `{}` on `{}`",
                head.commit_id.0,
                branch_id.0
            ))
        })?;

        Ok(commit_envelope_to_bridge_envelope(envelope))
    }
}
