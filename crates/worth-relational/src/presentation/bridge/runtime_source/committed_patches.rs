use worth_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, CommittedPatchSource, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest,
};

use super::RuntimeBridgeRelationalSource;
use crate::presentation::bridge::identities::parse_bridge_commit_identity;

impl RuntimeBridgeRelationalSource {
    fn publish_commit(
        &self,
        commit_id: crate::history::data::CommitId,
    ) -> Result<
        super::super::RelationalBridgePublicationOutcome,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        let selected_commit = self.selected_commit_for_retained_observation(commit_id)?;
        Ok(self.publish_commit_for_selected_observation(selected_commit))
    }

    fn publish_commit_on_branch(
        &self,
        commit_id: crate::history::data::CommitId,
        branch_identity: &worth_runtime_bridge::facade::TruthBranchIdentity,
    ) -> Result<
        super::super::RelationalBridgePublicationOutcome,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        let (head_commit_id, snapshot_identity) =
            self.branch_head_bindings.resolve(branch_identity)?;
        let observation = self.observation_bindings.resolve(&snapshot_identity)?;
        let selected_commit = if head_commit_id == commit_id {
            self.select_exact_commit_for_observation(commit_id, observation)?
        } else {
            self.select_commit_for_observation(commit_id, observation)?
        };
        Ok(self.publish_commit_for_selected_observation(selected_commit))
    }

    fn selected_commit_for_retained_observation(
        &self,
        commit_id: crate::history::data::CommitId,
    ) -> Result<
        super::RelationalBridgeSelectedCommitObservation,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        let snapshot_identity = match self
            .branch_head_bindings
            .unique_snapshot_for_commit(commit_id)?
        {
            Some(snapshot) => snapshot,
            None => self
                .observation_bindings
                .snapshot_identity_for_commit(commit_id)?,
        };
        let observation = self.observation_bindings.resolve(&snapshot_identity)?;
        self.select_commit_for_observation(commit_id, observation)
    }

    /// Publish one commit through an explicitly retained observation while
    /// consuming runtime-affine widening admission. A raw commit identity or
    /// copied snapshot identity cannot open this door.
    pub fn publish_commit_with_widening_at_snapshot(
        &self,
        commit_id: crate::history::data::CommitId,
        snapshot_identity: &worth_runtime_bridge::facade::TruthSnapshotIdentity,
        admission: &super::super::RelationalOpaqueAspectWideningAdmission,
    ) -> Result<
        super::super::RelationalBridgePublicationOutcome,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        let observation = self.observation_bindings.resolve(snapshot_identity)?;
        let selected_commit = self.select_commit_for_observation(commit_id, observation)?;
        Ok(self.runtime.with_runtime(|runtime| {
            runtime.publish_commit_for_bridge_with_widening_at_observation(
                selected_commit,
                self.graph_role.clone(),
                admission,
            )
        }))
    }

    pub(super) fn publish_commit_for_selected_observation(
        &self,
        selected_commit: super::RelationalBridgeSelectedCommitObservation,
    ) -> super::super::RelationalBridgePublicationOutcome {
        self.runtime.with_runtime(|runtime| match &self.partition {
            Some(partition) => runtime.publish_commit_for_bridge_graph_partition_at_observation(
                selected_commit,
                self.graph_role.clone(),
                partition.relational,
                partition.truth.clone(),
            ),
            None => runtime.publish_commit_for_bridge_graph_role_at_observation(
                selected_commit,
                self.graph_role.clone(),
            ),
        })
    }

    fn select_commit_for_observation(
        &self,
        commit_id: crate::history::data::CommitId,
        observation: super::RelationalBridgeSelectedObservation,
    ) -> Result<
        super::RelationalBridgeSelectedCommitObservation,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        self.runtime
            .with_runtime(|runtime| observation.select_reachable_commit(runtime, commit_id))
    }

    pub(super) fn select_exact_commit_for_observation(
        &self,
        commit_id: crate::history::data::CommitId,
        observation: super::RelationalBridgeSelectedObservation,
    ) -> Result<
        super::RelationalBridgeSelectedCommitObservation,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        self.runtime
            .with_runtime(|runtime| observation.select_exact_selected_commit(runtime, commit_id))
    }
}

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
                let selected_commit = self.select_commit_for_observation(commit_id, observation)?;
                self.publish_commit_for_selected_observation(selected_commit)
            }
            None => match request.branch_identity() {
                Some(branch) => self.publish_commit_on_branch(commit_id, branch)?,
                None => self.publish_commit(commit_id)?,
            },
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
