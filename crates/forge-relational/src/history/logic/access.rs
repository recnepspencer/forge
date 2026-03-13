use std::collections::BTreeSet;

use crate::history::data::{
    BranchHead, BranchId, CommitId, CommitReference, MergeConflictRecord, MergeInspection,
    VersionGraphSnapshot,
};
use crate::logic::runtime::RelationalRuntime;
use crate::publication::data::diff::{PatchStreamPosition, RelationalPatchRecord};
use crate::replay::data::CanonicalCommitEnvelope;

pub struct HistoryAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.history.current_version_id()
    }

    pub fn history_access(&self) -> HistoryAccess<'_> {
        HistoryAccess::new(self)
    }
}

impl<'runtime> HistoryAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn latest_commit(&self) -> Option<&CommitReference> {
        self.runtime
            .publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.commit)
            .or_else(|| {
                self.runtime
                    .history
                    .commit_envelopes
                    .values()
                    .max_by_key(|envelope| envelope.commit.commit_id)
                    .map(|envelope| &envelope.commit)
            })
    }

    pub(crate) fn commit_envelope(&self, commit_id: CommitId) -> Option<&CanonicalCommitEnvelope> {
        self.runtime
            .history
            .commit_envelopes
            .get(&commit_id)
            .map(|envelope| envelope.as_ref())
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

    pub(crate) fn patches_after(
        &self,
        after_position: Option<PatchStreamPosition>,
        max_commits: usize,
    ) -> Vec<RelationalPatchRecord> {
        let start = after_position
            .map(std::ops::Bound::Excluded)
            .unwrap_or(std::ops::Bound::Unbounded);
        self.runtime
            .history
            .patch_stream_index
            .range((start, std::ops::Bound::Unbounded))
            .filter_map(|(_, commit_id)| self.commit_envelope(*commit_id))
            .map(|envelope| envelope.patch.clone())
            .take(max_commits)
            .collect()
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

    pub fn ancestor_chain(&self, commit_id: CommitId) -> Vec<CommitId> {
        let mut ordered = self.ancestor_set(commit_id).into_iter().collect::<Vec<_>>();
        ordered.sort_by_key(|id| id.0);
        ordered
    }

    pub fn latest_common_ancestor_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> Option<CommitId> {
        let left = self.branch_head(left_branch)?.commit_id;
        let right = self.branch_head(right_branch)?.commit_id;
        self.latest_common_ancestor(left, right)
    }

    pub fn can_merge_branch_into(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> bool {
        let Some(source_head) = self.branch_head(source_branch) else {
            return false;
        };
        let Some(target_head) = self.branch_head(target_branch) else {
            return false;
        };
        self.latest_common_ancestor(target_head.commit_id, source_head.commit_id)
            .is_some()
    }

    pub fn inspect_merge(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> MergeInspection {
        let source_head = self.branch_head(source_branch).cloned();
        let target_head = self.branch_head(target_branch).cloned();
        let merge_base =
            source_head
                .as_ref()
                .zip(target_head.as_ref())
                .and_then(|(source, target)| {
                    self.latest_common_ancestor(source.commit_id, target.commit_id)
                });

        let source_only_commits = source_head
            .as_ref()
            .map(|head| self.branch_unique_commits(head.commit_id, merge_base))
            .unwrap_or_default();
        let target_only_commits = target_head
            .as_ref()
            .map(|head| self.branch_unique_commits(head.commit_id, merge_base))
            .unwrap_or_default();
        let conflicting_records = self.merge_conflicts_between(
            source_only_commits.as_slice(),
            target_only_commits.as_slice(),
        );

        MergeInspection {
            source_branch: source_branch.clone(),
            target_branch: target_branch.clone(),
            source_head,
            target_head,
            merge_base,
            source_only_commits,
            target_only_commits,
            can_merge: merge_base.is_some() && conflicting_records.is_empty(),
            conflicting_records,
        }
    }

    pub(crate) fn latest_common_ancestor(
        &self,
        left: CommitId,
        right: CommitId,
    ) -> Option<CommitId> {
        let left_ancestors = self.ancestor_set(left);
        let right_ancestors = self.ancestor_set(right);
        left_ancestors
            .intersection(&right_ancestors)
            .copied()
            .max_by_key(|commit_id| commit_id.0)
    }

    fn ancestor_set(&self, start: CommitId) -> BTreeSet<CommitId> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![start];
        while let Some(commit_id) = stack.pop() {
            if !seen.insert(commit_id) {
                continue;
            }
            if let Some(node) = self.runtime.history.commit_graph.get(&commit_id) {
                stack.extend(node.commit.parents.iter().copied());
            }
        }
        seen
    }

    fn branch_unique_commits(&self, head: CommitId, merge_base: Option<CommitId>) -> Vec<CommitId> {
        let mut commits = self.ancestor_set(head).into_iter().collect::<Vec<_>>();
        if let Some(merge_base) = merge_base {
            let base_ancestors = self.ancestor_set(merge_base);
            commits.retain(|commit_id| !base_ancestors.contains(commit_id));
        }
        commits.sort_by_key(|commit_id| commit_id.0);
        commits
    }

    fn merge_conflicts_between(
        &self,
        left_commits: &[CommitId],
        right_commits: &[CommitId],
    ) -> Vec<MergeConflictRecord> {
        let left_records = self.commit_record_set(left_commits);
        let right_records = self.commit_record_set(right_commits);
        let mut conflicts = left_records
            .intersection(&right_records)
            .cloned()
            .collect::<Vec<_>>();
        conflicts.sort();
        conflicts
    }

    fn commit_record_set(&self, commits: &[CommitId]) -> BTreeSet<MergeConflictRecord> {
        commits
            .iter()
            .filter_map(|commit_id| self.runtime.history.commit_envelopes.get(commit_id))
            .flat_map(|envelope| envelope.patch.records.iter())
            .map(|record| match record.target {
                crate::transactions::data::RecordRef::Entity(entity_id) => {
                    MergeConflictRecord::Entity(entity_id)
                }
                crate::transactions::data::RecordRef::Relation(relation_id) => {
                    MergeConflictRecord::Relation(relation_id)
                }
            })
            .collect()
    }
}
