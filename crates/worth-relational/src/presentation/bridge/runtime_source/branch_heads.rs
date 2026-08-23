use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, RelationalBridgeSourceError, TruthBranchHeadSource,
    TruthBranchIdentity,
};

use super::RuntimeBridgeRelationalSource;

impl TruthBranchHeadSource for RuntimeBridgeRelationalSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let (commit_id, snapshot_identity) = self.branch_head_bindings.resolve(branch_identity)?;

        match self.publish_commit_at_snapshot(commit_id, snapshot_identity) {
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
