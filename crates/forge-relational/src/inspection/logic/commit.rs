use crate::history::data::{BranchId, CommitId};
use crate::inspection::data::{
    CommitInspection, InspectionAccessPath, InspectionOrigin, RecentCommitInspectionRequest,
    RecentCommitInspectionWindow,
};
use crate::publication::patch::data::ordered_aspect_keys;

use super::access::InspectionAccess;

impl<'runtime> InspectionAccess<'runtime> {
    pub fn inspect_commit(&self, commit_id: CommitId) -> Option<CommitInspection> {
        self.count_commit_read();
        let envelope = self.commit_envelope(commit_id)?;
        Some(CommitInspection {
            commit: envelope.commit.clone(),
            changed_records: envelope
                .patch
                .authoritative_record_patches
                .iter()
                .map(|record| record.target.clone())
                .collect(),
            lineage_event_ids: envelope
                .lineage_digest_basis()
                .canonical_event_ids()
                .to_vec(),
            lineage_events: envelope.lineage_events().to_vec(),
            lineage_digest_basis: envelope.lineage_digest_basis().clone(),
            lineage_artifact_counters: envelope.lineage_artifact_counters(),
            derived_index_artifacts: envelope.derived_index_artifacts().clone(),
            changed_aspects: ordered_aspect_keys(
                envelope
                    .patch
                    .authoritative_record_patches
                    .iter()
                    .flat_map(|record| record.authoritative_changed_aspect_keys().cloned()),
            ),
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        })
    }

    pub fn inspect_recent_commits(
        &self,
        request: &RecentCommitInspectionRequest,
    ) -> RecentCommitInspectionWindow {
        let limit = usize::try_from(request.limit).unwrap_or(usize::MAX);
        let commits = self
            .recent_commit_ids(request.branch_id.as_ref(), limit)
            .into_iter()
            .filter_map(|commit_id| self.inspect_commit(commit_id))
            .collect();
        let branch_head = request
            .branch_id
            .as_ref()
            .and_then(|branch_id| self.branch_head_ref(branch_id));
        RecentCommitInspectionWindow {
            branch_head,
            commits,
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        }
    }

    pub fn inspect_branch_head(&self, branch_id: &BranchId) -> Option<CommitInspection> {
        let head = self.branch_head_ref(branch_id)?;
        self.inspect_commit(head.commit_id)
    }
}
