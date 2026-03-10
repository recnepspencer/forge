use std::collections::BTreeSet;

use crate::history::data::{
    BranchCreateError, BranchHead, BranchId, CommitId, MergeConflictRecord, MergeInspection,
    VersionGraphSnapshot,
};
use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn latest_commit(&self) -> Option<&crate::history::data::CommitReference> {
        self.publication
            .latest_bundle
            .as_ref()
            .map(|bundle| &bundle.commit)
            .or_else(|| {
                self.history
                    .commit_envelopes
                    .values()
                    .max_by_key(|envelope| envelope.commit.commit_id)
                    .map(|envelope| &envelope.commit)
            })
    }

    pub fn branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Option<&crate::history::data::CommitReference> {
        self.history
            .branch_heads
            .get(branch_id)
            .and_then(|head| head.as_ref())
    }

    pub fn branches(&self) -> Vec<BranchHead> {
        self.history
            .branch_heads
            .iter()
            .map(|(branch_id, head)| BranchHead {
                branch_id: branch_id.clone(),
                head: head.clone(),
            })
            .collect()
    }

    pub fn version_graph(&self) -> VersionGraphSnapshot {
        VersionGraphSnapshot {
            branches: self.branches(),
            commits: self.history.commit_graph.values().cloned().collect(),
        }
    }

    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        crate::identity::data::VersionId(self.history.next_version_id.saturating_sub(1))
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

    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: &BranchId,
    ) -> Result<(), BranchCreateError> {
        if self.history.branch_heads.contains_key(&new_branch) {
            return Err(BranchCreateError::BranchAlreadyExists);
        }
        let Some(source_head) = self.history.branch_heads.get(from_branch).cloned() else {
            return Err(BranchCreateError::SourceBranchMissing);
        };
        self.history
            .branch_heads
            .insert(new_branch, source_head.clone());
        if let Some(source_head) = source_head {
            let state = self.build_visibility_state(
                source_head.version_id,
                crate::snapshots::data::SnapshotId(0),
                crate::snapshots::data::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
            );
            self.pin_branch_state(&state);
        }
        Ok(())
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
            if let Some(node) = self.history.commit_graph.get(&commit_id) {
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
            .filter_map(|commit_id| self.history.commit_envelopes.get(commit_id))
            .flat_map(|envelope| envelope.patch.records.iter())
            .filter_map(|record| match (record.entity_id, record.relation_id) {
                (Some(entity_id), None) => Some(MergeConflictRecord::Entity(entity_id)),
                (None, Some(relation_id)) => Some(MergeConflictRecord::Relation(relation_id)),
                _ => None,
            })
            .collect()
    }
}
impl RelationalRuntime {
    pub fn retain_version_for_replay(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        if let Some(retained) = self.snapshots.replay_retained.get_mut(&version_id) {
            retained.ref_count += 1;
            return true;
        }
        if version_id.0 == 0 || version_id.0 > self.current_version_id().0 {
            return false;
        }
        let state = self.build_visibility_state(
            version_id,
            crate::snapshots::data::SnapshotId(0),
            crate::snapshots::data::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        );
        self.pin_replay_state(&state);
        self.snapshots.replay_retained.insert(
            version_id,
            crate::logic::runtime::ReplayRetentionState { state, ref_count: 1 },
        );
        true
    }

    pub fn release_version_replay_retention(
        &mut self,
        version_id: crate::identity::data::VersionId,
    ) -> bool {
        let Some(mut retained) = self.snapshots.replay_retained.remove(&version_id) else {
            return false;
        };
        if retained.ref_count > 1 {
            retained.ref_count -= 1;
            self.snapshots.replay_retained.insert(version_id, retained);
            return true;
        }
        self.unpin_replay_state(&retained.state);
        true
    }
}
