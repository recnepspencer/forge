use crate::history::data::CanonicalCommitEnvelope;
use crate::history::data::CommitId;
use crate::publication::patch::data::PatchStreamPosition;
use crate::runtime::RelationalRuntime;

use super::CommitEnvelopeSource;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PatchStreamCommitRef {
    pub position: PatchStreamPosition,
    pub commit_id: CommitId,
}

pub(crate) trait PatchStreamSource: CommitEnvelopeSource {
    fn latest_patch_stream_position(&self) -> Option<PatchStreamPosition>;
    fn commit_id_at_patch_stream_position(&self, position: PatchStreamPosition)
        -> Option<CommitId>;
    fn patch_stream_commits_after(
        &self,
        after_position: Option<PatchStreamPosition>,
        max_commits: usize,
    ) -> Vec<PatchStreamCommitRef>;

    fn contains_patch_stream_position(&self, position: PatchStreamPosition) -> bool {
        self.commit_id_at_patch_stream_position(position).is_some()
    }

    fn commit_envelope_at_patch_stream_position(
        &self,
        position: PatchStreamPosition,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.commit_id_at_patch_stream_position(position)
            .and_then(|commit_id| self.commit_envelope(commit_id))
    }
}

impl PatchStreamSource for RelationalRuntime {
    fn latest_patch_stream_position(&self) -> Option<PatchStreamPosition> {
        self.history
            .patch_stream_index
            .last_key_value()
            .map(|(position, _)| *position)
    }

    fn commit_id_at_patch_stream_position(
        &self,
        position: PatchStreamPosition,
    ) -> Option<CommitId> {
        self.history.patch_stream_index.get(&position).copied()
    }

    fn patch_stream_commits_after(
        &self,
        after_position: Option<PatchStreamPosition>,
        max_commits: usize,
    ) -> Vec<PatchStreamCommitRef> {
        let start = after_position
            .map(std::ops::Bound::Excluded)
            .unwrap_or(std::ops::Bound::Unbounded);
        self.history
            .patch_stream_index
            .range((start, std::ops::Bound::Unbounded))
            .map(|(position, commit_id)| PatchStreamCommitRef {
                position: *position,
                commit_id: *commit_id,
            })
            .take(max_commits)
            .collect()
    }
}
