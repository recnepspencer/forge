use std::sync::Arc;

use crate::branch::RelationalBranchReferenceCell;
use crate::branch::{RelationalBranchRoot, RelationalBranchRootIdentityIssuer};
use crate::history::data::{BranchId, CanonicalCommitEnvelope, CommitId, RelationalCommitReceipt};
use crate::history::RelationalCommitArtifact;
use crate::publication::patch::data::PatchStreamPosition;

use super::HistorySubsystem;

pub(crate) struct PreparedVersionedArtifactPublication {
    pub(crate) commit_id: CommitId,
    pub(crate) commit_reference: RelationalCommitReceipt,
    pub(crate) branch_id: BranchId,
    pub(crate) next_cell: RelationalBranchReferenceCell,
    pub(crate) patch_position: PatchStreamPosition,
    pub(crate) envelope: Arc<CanonicalCommitEnvelope>,
    pub(crate) root: Arc<RelationalBranchRoot>,
    pub(crate) artifact: RelationalCommitArtifact,
    pub(crate) new_authoritative_bytes: u64,
    pub(crate) next_root_issuer: RelationalBranchRootIdentityIssuer,
    pub(crate) recovery_readmission: bool,
}

impl HistorySubsystem {
    pub(crate) fn install_prepared_versioned_artifact(
        &mut self,
        prepared: PreparedVersionedArtifactPublication,
    ) {
        let PreparedVersionedArtifactPublication {
            commit_id,
            commit_reference,
            branch_id,
            mut next_cell,
            patch_position,
            envelope,
            root,
            artifact,
            new_authoritative_bytes,
            next_root_issuer,
            recovery_readmission,
        } = prepared;
        self.commit_branch_root_capture(next_root_issuer);
        if recovery_readmission {
            self.commit_catalog.install_prepared_recovery(artifact);
        } else {
            self.commit_catalog.install_prepared(artifact);
        }
        self.next_commit_id = self
            .next_commit_id
            .checked_add(1)
            .expect("prepared truth publication reserved commit sequence capacity");
        self.next_version_id = self
            .next_version_id
            .checked_add(1)
            .expect("prepared truth publication reserved version sequence capacity");
        self.record_root_publication(&branch_id, &root, new_authoritative_bytes);
        next_cell.install_root(root);
        self.commit_graph.insert(
            commit_id,
            crate::history::data::VersionNode {
                commit: commit_reference,
            },
        );
        self.commit_envelopes.insert(commit_id, envelope);
        self.patch_stream_index.insert(patch_position, commit_id);
        // The reference cell is the visibility linearization point. Every
        // canonical consumer above is installed before this infallible move.
        self.insert_branch_cell(next_cell);
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
            None,
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
        root: Option<Arc<crate::branch::RelationalBranchRoot>>,
        advances_truth: bool,
    ) -> Result<(), String> {
        self.publish_artifact_inner(
            commit_id,
            commit_reference,
            branch_id,
            next_cell,
            patch_position,
            envelope,
            root,
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
        root: Option<Arc<crate::branch::RelationalBranchRoot>>,
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
            match root.as_ref() {
                Some(root) => self
                    .commit_catalog
                    .append_envelope_with_root(Arc::clone(&envelope), Arc::clone(root)),
                None => self.commit_catalog.append_envelope(Arc::clone(&envelope)),
            }
            .map_err(|denial| format!("published catalog admission denied: {denial:?}"))?;
        }
        if advances_truth {
            self.advance_commit_sequence().map_err(str::to_owned)?;
        } else {
            self.advance_metadata_commit_sequence()
                .map_err(str::to_owned)?;
        }
        if let Some(root) = root.as_ref() {
            self.record_root_publication(
                &branch_id,
                root,
                root.publication_cost().new_authoritative_bytes,
            );
        }
        let mut next_cell = next_cell;
        if let Some(root) = root {
            next_cell.install_root(root);
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
