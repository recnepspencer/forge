use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, CommittedPatchSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest,
};

use super::RuntimeBridgeRelationalSource;
use crate::presentation::bridge::identities::parse_bridge_commit_identity;

impl CommittedPatchSource for RuntimeBridgeRelationalSource {
    fn authoritative_source_profile(
        &self,
    ) -> Option<worth_runtime_bridge::facade::BridgeAuthoritativeSourceProfile> {
        Some(RuntimeBridgeRelationalSource::authoritative_source_profile(
            self,
        ))
    }

    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let commit_id = parse_bridge_commit_identity(request.commit_identity())?;
        let publication = match request.snapshot_identity() {
            Some(snapshot) => {
                let observation = self.observation_bindings.resolve(snapshot)?;
                let selected_commit = observation.selected_root().commit_id().ok_or_else(|| {
                    RelationalBridgeSourceError::new(format!(
                        "relational bridge snapshot {snapshot:?} has no committed selected root"
                    ))
                })?;
                if selected_commit != commit_id {
                    return Err(RelationalBridgeSourceError::new(format!(
                        "relational bridge snapshot {snapshot:?} selects commit `{}` rather than requested commit `{}`",
                        selected_commit.0, commit_id.0
                    )));
                }
                self.publish_commit_at_snapshot(commit_id, snapshot.clone())
            }
            None => self.publish_commit(commit_id)?,
        };
        match publication {
            worth_proof::TransitionOutcome::Success(publication) => {
                Ok(publication.into_bridge_envelope())
            }
            worth_proof::TransitionOutcome::Denied(denial) => {
                Err(RelationalBridgeSourceError::new(format!(
                    "relational committed patch could not be admitted by Bridge: {denial}"
                )))
            }
            worth_proof::TransitionOutcome::Deferred(_) => Err(RelationalBridgeSourceError::new(
                "relational committed patch publication deferred",
            )),
            worth_proof::TransitionOutcome::Stale(_) => Err(RelationalBridgeSourceError::new(
                "relational committed patch authority is stale",
            )),
            worth_proof::TransitionOutcome::RebindRequired(_) => {
                Err(RelationalBridgeSourceError::new(
                    "relational committed patch requires graph rebind",
                ))
            }
            worth_proof::TransitionOutcome::Failed(_) => Err(RelationalBridgeSourceError::new(
                "relational committed patch lowering failed",
            )),
        }
    }
}
