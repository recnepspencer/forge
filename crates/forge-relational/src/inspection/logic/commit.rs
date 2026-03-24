use crate::history::data::{BranchId, CommitId};
use crate::inspection::data::{
    CommitInspection, InspectionAccessPath, InspectionOrigin, RecentCommitInspectionRequest,
    RecentCommitInspectionWindow,
};
use crate::publication::patch::data::CanonicalAspectSet;

use super::access::InspectionAccess;

impl<'runtime> InspectionAccess<'runtime> {
    pub fn inspect_commit(&self, commit_id: CommitId) -> Option<CommitInspection> {
        self.runtime.services.instrumentation.count(|counters| {
            counters.inspection_commit_reads += 1;
        });
        let history_access = self.runtime.history_access();
        let envelope = history_access.commit_envelope(commit_id)?;
        Some(CommitInspection {
            commit: envelope.commit.clone(),
            changed_records: envelope
                .patch
                .records
                .iter()
                .map(|record| record.target.clone())
                .collect(),
            lineage_event_ids: envelope.lineage_digest_basis().canonical_event_ids().to_vec(),
            lineage_events: envelope.lineage_events().to_vec(),
            lineage_digest_basis: envelope.lineage_digest_basis().clone(),
            lineage_artifact_counters: envelope.lineage_artifact_counters(),
            index_generation_ids: envelope.index_generation_ids.clone(),
            index_generations: envelope.index_generations.clone(),
            changed_aspects: CanonicalAspectSet::new(
                envelope
                    .patch
                    .records
                    .iter()
                    .flat_map(|record| record.aspects.iter().cloned()),
            ),
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        })
    }

    pub fn inspect_recent_commits(
        &self,
        request: &RecentCommitInspectionRequest,
    ) -> RecentCommitInspectionWindow {
        let history_access = self.runtime.history_access();
        let limit = usize::try_from(request.limit).unwrap_or(usize::MAX);
        let commits = self
            .runtime
            .history_access()
            .recent_commit_ids(request.branch_id.as_ref(), limit)
            .into_iter()
            .filter_map(|commit_id| self.inspect_commit(commit_id))
            .collect();
        let branch_head = request
            .branch_id
            .as_ref()
            .and_then(|branch_id| history_access.branch_head(branch_id).cloned());
        RecentCommitInspectionWindow {
            branch_head,
            commits,
            origin: InspectionOrigin::CanonicalCommitStorage,
            access_path: InspectionAccessPath::CommitIndexRead,
        }
    }

    pub fn inspect_branch_head(&self, branch_id: &BranchId) -> Option<CommitInspection> {
        let history = self.runtime.history_access();
        let head = history.branch_head(branch_id)?;
        self.inspect_commit(head.commit_id)
    }
}
