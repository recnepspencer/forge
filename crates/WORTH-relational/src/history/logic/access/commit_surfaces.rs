use crate::capabilities::PublicationBundleSource;
use crate::history::data::{BranchHead, BranchId, CommitId, CommitReference, VersionGraphSnapshot};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    pub fn latest_commit(&self) -> Option<&CommitReference> {
        let latest_published = self.runtime.latest_published_commit_ref();
        let latest_history = self
            .runtime
            .history
            .commit_envelopes
            .values()
            .max_by_key(|envelope| envelope.commit.commit_id)
            .map(|envelope| &envelope.commit);
        match (latest_published, latest_history) {
            (Some(published), Some(history)) => Some(if published.commit_id >= history.commit_id {
                published
            } else {
                history
            }),
            (Some(published), None) => Some(published),
            (None, Some(history)) => Some(history),
            (None, None) => None,
        }
    }

    pub(crate) fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_envelopes
            .get(&commit_id)
            .map(|envelope| envelope.as_ref())
    }

    pub(crate) fn commit_envelope_for_version(
        &self,
        version_id: crate::identity::data::VersionId,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_envelopes
            .values()
            .map(|envelope| envelope.as_ref())
            .find(|envelope| envelope.commit.version_id == version_id)
    }

    pub(crate) fn latest_patch_stream_position(&self) -> Option<PatchStreamPosition> {
        self.runtime
            .history
            .patch_stream_index
            .last_key_value()
            .map(|(position, _)| *position)
    }

    pub(crate) fn contains_patch_stream_position(&self, position: PatchStreamPosition) -> bool {
        self.runtime
            .history
            .patch_stream_index
            .contains_key(&position)
    }

    pub(crate) fn commit_envelopes_snapshot(&self) -> Vec<CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_envelopes
            .values()
            .map(|envelope| envelope.as_ref().clone())
            .collect()
    }

    pub(crate) fn next_commit_id(&self) -> CommitId {
        self.runtime.history.preview_next_commit_id()
    }

    pub(crate) fn preview_next_version_id(&self) -> crate::identity::data::VersionId {
        self.runtime.history.preview_next_version_id()
    }

    pub(crate) fn commit_count(&self) -> usize {
        self.runtime.history.commit_graph.len()
    }

    pub fn branch_head(&self, branch_id: &BranchId) -> Option<&CommitReference> {
        self.runtime
            .history
            .branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }

    pub(crate) fn recent_commit_ids(
        &self,
        branch_id: Option<&BranchId>,
        limit: usize,
    ) -> Vec<CommitId> {
        match branch_id {
            Some(branch_id) => self
                .branch_commit_envelopes(branch_id)
                .into_iter()
                .rev()
                .take(limit)
                .map(|envelope| envelope.commit.commit_id)
                .collect(),
            None => self
                .runtime
                .history
                .commit_envelopes
                .values()
                .rev()
                .take(limit)
                .map(|envelope| envelope.commit.commit_id)
                .collect(),
        }
    }

    pub fn branches(&self) -> Vec<BranchHead> {
        self.runtime
            .history
            .branch_heads
            .iter()
            .map(|(branch_id, head)| BranchHead {
                branch_id: branch_id.clone(),
                head: head.clone(),
            })
            .collect()
    }

    pub(crate) fn branch_head_versions(&self) -> Vec<crate::identity::data::VersionId> {
        self.runtime
            .history
            .branch_heads
            .values()
            .filter_map(|head| head.as_ref().map(|head| head.version_id))
            .collect()
    }

    pub fn version_graph(&self) -> VersionGraphSnapshot {
        VersionGraphSnapshot {
            branches: self.branches(),
            commits: self
                .runtime
                .history
                .commit_graph
                .values()
                .cloned()
                .collect(),
        }
    }

    pub(super) fn branch_commit_envelopes(
        &self,
        branch_id: &BranchId,
    ) -> Vec<&CanonicalCommitEnvelope> {
        let Some(head) = self.branch_head(branch_id) else {
            return Vec::new();
        };
        let branch_commits = self.ancestor_set(head.commit_id);
        let mut envelopes = self
            .runtime
            .history
            .commit_envelopes
            .values()
            .filter(|envelope| {
                branch_commits.contains(&envelope.commit.commit_id)
                    && envelope.commit.branch_id == *branch_id
            })
            .map(|envelope| envelope.as_ref())
            .collect::<Vec<_>>();
        envelopes
            .sort_by_key(|envelope| (envelope.commit.version_id.0, envelope.commit.commit_id.0));
        envelopes
    }
}
