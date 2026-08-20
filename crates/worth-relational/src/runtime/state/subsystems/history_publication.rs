use std::sync::Arc;

use crate::branch::RelationalBranchReferenceCell;
use crate::history::data::{BranchId, CanonicalCommitEnvelope, CommitId, RelationalCommitReceipt};
use crate::publication::patch::data::PatchStreamPosition;

use super::HistorySubsystem;

impl HistorySubsystem {
    pub(crate) fn publish_versioned_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        next_cell: RelationalBranchReferenceCell,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.publish_artifact(
            commit_id,
            commit_reference,
            branch_id,
            next_cell,
            patch_position,
            envelope,
            true,
        )
    }

    pub(crate) fn publish_metadata_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        next_cell: RelationalBranchReferenceCell,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
    ) -> Result<(), String> {
        self.publish_artifact(
            commit_id,
            commit_reference,
            branch_id,
            next_cell,
            patch_position,
            envelope,
            false,
        )
    }

    fn publish_artifact(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        next_cell: RelationalBranchReferenceCell,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
        advances_truth: bool,
    ) -> Result<(), String> {
        self.publish_artifact_inner(
            commit_id,
            commit_reference,
            branch_id,
            next_cell,
            patch_position,
            envelope,
            advances_truth,
        )
    }

    fn publish_artifact_inner(
        &mut self,
        commit_id: CommitId,
        commit_reference: RelationalCommitReceipt,
        branch_id: BranchId,
        next_cell: RelationalBranchReferenceCell,
        patch_position: PatchStreamPosition,
        envelope: Arc<CanonicalCommitEnvelope>,
        advances_truth: bool,
    ) -> Result<(), String> {
        if !self.has_branch(&branch_id) {
            return Err(format!("published branch `{}` is missing", branch_id.0));
        }
        if let Some(existing) = self.commit_catalog.get(commit_id) {
            if existing.envelope().as_ref() != envelope.as_ref() {
                return Err(format!(
                    "commit id {} cannot name two immutable catalog artifacts",
                    commit_id.0
                ));
            }
        } else {
            self.commit_catalog
                .append_envelope(Arc::clone(&envelope))
                .map_err(|denial| format!("published catalog admission denied: {denial:?}"))?;
        }
        if advances_truth {
            self.advance_commit_sequence().map_err(str::to_owned)?;
        } else {
            self.advance_metadata_commit_sequence()
                .map_err(str::to_owned)?;
        }
        self.insert_branch_cell(next_cell);
        self.commit_graph.insert(
            commit_id,
            crate::history::data::VersionNode {
                commit: commit_reference,
            },
        );
        self.commit_envelopes.insert(commit_id, envelope);
        self.patch_stream_index.insert(patch_position, commit_id);
        Ok(())
    }
}
