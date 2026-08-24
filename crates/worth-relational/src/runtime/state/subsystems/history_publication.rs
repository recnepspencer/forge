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
}
