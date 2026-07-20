use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, RelationalBridgeSourceError, TruthBranchHeadSource,
    TruthBranchIdentity,
};

use super::RuntimeBridgeRelationalSource;
use crate::capabilities::CommitEnvelopeSource;

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

        match self.publish_commit(envelope.commit.commit_id) {
            worth_proof::TransitionOutcome::Success(publication) => {
                Ok(publication.into_bridge_envelope())
            }
            worth_proof::TransitionOutcome::Denied(denial) => {
                Err(RelationalBridgeSourceError::new(format!(
                    "relational branch-head patch could not be admitted by Bridge: {denial}"
                )))
            }
            worth_proof::TransitionOutcome::Deferred(_) => Err(RelationalBridgeSourceError::new(
                "relational branch-head publication deferred",
            )),
            worth_proof::TransitionOutcome::Stale(_) => Err(RelationalBridgeSourceError::new(
                "relational branch-head authority is stale",
            )),
            worth_proof::TransitionOutcome::RebindRequired(_) => Err(
                RelationalBridgeSourceError::new("relational branch-head requires graph rebind"),
            ),
            worth_proof::TransitionOutcome::Failed(_) => Err(RelationalBridgeSourceError::new(
                "relational branch-head lowering failed",
            )),
        }
    }
}
