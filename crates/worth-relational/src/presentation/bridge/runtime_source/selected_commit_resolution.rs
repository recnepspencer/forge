use worth_runtime_bridge::facade::RelationalBridgeSourceError;

use crate::history::data::CommitId;
use crate::history::CommitAncestryPosture;

use super::{RelationalBridgeSelectedCommitObservation, RelationalBridgeSelectedObservation};

impl RelationalBridgeSelectedObservation {
    pub(super) fn select_reachable_commit(
        self,
        runtime: &crate::runtime::RelationalRuntime,
        commit_id: CommitId,
    ) -> Result<RelationalBridgeSelectedCommitObservation, RelationalBridgeSourceError> {
        let selected_commit = self.observation.commit_id().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot {:?} has no committed selected root",
                self.snapshot_identity
            ))
        })?;
        let ancestry = runtime.history().inspect_commit_ancestry(selected_commit);
        let classification = runtime
            .history()
            .classify_commit_in_ancestry(&ancestry, commit_id);
        runtime
            .performance_access()
            .count_bridge_observation_commit_selection(classification.traversal_work());
        match classification.posture() {
            CommitAncestryPosture::SelectedCommitUnavailable => {
                Err(RelationalBridgeSourceError::new(format!(
                    "relational bridge snapshot {:?} selects unavailable commit `{}`",
                    self.snapshot_identity, selected_commit.0
                )))
            }
            CommitAncestryPosture::RequestedCommitUnavailable => {
                Err(RelationalBridgeSourceError::new(format!(
                    "relational bridge snapshot {:?} cannot see unavailable requested commit `{}`",
                    self.snapshot_identity, commit_id.0
                )))
            }
            CommitAncestryPosture::Unreachable => Err(RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot {:?} at commit `{}` cannot see requested commit `{}` without an exact retained historical observation",
                self.snapshot_identity, selected_commit.0, commit_id.0,
            ))),
            CommitAncestryPosture::Reachable => Ok(RelationalBridgeSelectedCommitObservation {
                commit_id,
                observation: self,
            }),
        }
    }

    pub(super) fn select_exact_selected_commit(
        self,
        runtime: &crate::runtime::RelationalRuntime,
        commit_id: CommitId,
    ) -> Result<RelationalBridgeSelectedCommitObservation, RelationalBridgeSourceError> {
        let selected_commit = self.observation.commit_id().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot {:?} has no selected commit",
                self.snapshot_identity
            ))
        })?;
        runtime
            .performance_access()
            .count_bridge_observation_commit_selection(0);
        if selected_commit != commit_id {
            return Err(RelationalBridgeSourceError::new(format!(
                "relational bridge snapshot {:?} selects commit `{}` rather than exact requested commit `{}`",
                self.snapshot_identity, selected_commit.0, commit_id.0
            )));
        }
        Ok(RelationalBridgeSelectedCommitObservation {
            commit_id,
            observation: self,
        })
    }
}
