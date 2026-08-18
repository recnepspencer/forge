use std::collections::BTreeSet;

use crate::branch::RelationalLegacyBranchBinding;
#[cfg(test)]
use crate::history::data::BranchId;
use crate::history::data::{CommitId, MergeConflictRecord, MergeInspection};

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    /// Returns the ancestor closure for `commit_id`, ordered by ascending
    /// `CommitId`.
    ///
    /// This is not a linear parent chain on DAG-shaped history.
    pub fn ancestor_closure_by_commit_id_order(&self, commit_id: CommitId) -> Vec<CommitId> {
        self.ancestor_set(commit_id).into_iter().collect()
    }

    /// Convenience wrapper over the runtime's current common-ancestor
    /// selection rule for two branch heads.
    ///
    /// The underlying selection rule is `max_commit_id_common_ancestor`, which
    /// intersects both ancestor sets and chooses the maximum `CommitId`.
    #[cfg(test)]
    pub(crate) fn latest_common_ancestor_between_branches(
        &self,
        left_branch: &BranchId,
        right_branch: &BranchId,
    ) -> Option<CommitId> {
        let left = self.branch_head(left_branch)?.commit_id;
        let right = self.branch_head(right_branch)?.commit_id;
        self.max_commit_id_common_ancestor(left, right)
    }

    /// Selects the common ancestor from two exact owner observations.  Raw
    /// branch names are intentionally not a production currentness input.
    pub(crate) fn latest_common_ancestor_between_bindings(
        &self,
        left_binding: &RelationalLegacyBranchBinding,
        right_binding: &RelationalLegacyBranchBinding,
    ) -> Option<CommitId> {
        let left = self.bound_head_commit_id(left_binding)?;
        let right = self.bound_head_commit_id(right_binding)?;
        self.max_commit_id_common_ancestor(left, right)
    }

    /// Inspect merge overlap from owner-issued branch bindings. Raw branch
    /// names remain a diagnostic/test vocabulary and cannot select current
    /// heads for production planning.
    pub(crate) fn inspect_merge_from_bindings(
        &self,
        source_binding: &RelationalLegacyBranchBinding,
        target_binding: &RelationalLegacyBranchBinding,
    ) -> Option<MergeInspection> {
        let source_branch = source_binding.identity().branch_id().clone();
        let target_branch = target_binding.identity().branch_id().clone();
        let source_head_id = self.bound_head_commit_id(source_binding)?;
        let target_head_id = self.bound_head_commit_id(target_binding)?;
        let source_head = self
            .runtime
            .history
            .commit_catalog
            .get(source_head_id)
            .map(|artifact| artifact.envelope().commit.clone())?;
        let target_head = self
            .runtime
            .history
            .commit_catalog
            .get(target_head_id)
            .map(|artifact| artifact.envelope().commit.clone())?;
        let merge_base =
            self.max_commit_id_common_ancestor(source_head.commit_id, target_head.commit_id);
        let source_only_commits =
            self.branch_unique_commit_closure_by_commit_id_order(source_head.commit_id, merge_base);
        let target_only_commits =
            self.branch_unique_commit_closure_by_commit_id_order(target_head.commit_id, merge_base);
        let conflicting_records = self.merge_conflicts_between(
            source_only_commits.as_slice(),
            target_only_commits.as_slice(),
        );
        Some(MergeInspection {
            source_branch,
            target_branch,
            source_head: Some(source_head),
            target_head: Some(target_head),
            merge_base,
            source_only_commits,
            target_only_commits,
            can_merge: merge_base.is_some() && conflicting_records.is_empty(),
            conflicting_records,
        })
    }

    fn bound_head_commit_id(&self, binding: &RelationalLegacyBranchBinding) -> Option<CommitId> {
        self.runtime
            .legacy_branch_binding_commit(binding)
            .map(|identity| identity.commit_id())
    }

    #[cfg(test)]
    pub(crate) fn can_merge_branch_into(
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
        self.max_commit_id_common_ancestor(target_head.commit_id, source_head.commit_id)
            .is_some()
    }

    #[cfg(test)]
    pub(crate) fn inspect_merge(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> MergeInspection {
        // This remains a history substrate helper. New merge-semantic planning
        // should land in `crate::merge`, not grow richer semantics here.
        let source_head = self.branch_head(source_branch).cloned();
        let target_head = self.branch_head(target_branch).cloned();
        let merge_base =
            source_head
                .as_ref()
                .zip(target_head.as_ref())
                .and_then(|(source, target)| {
                    self.max_commit_id_common_ancestor(source.commit_id, target.commit_id)
                });

        let source_only_commits = source_head
            .as_ref()
            .map(|head| {
                self.branch_unique_commit_closure_by_commit_id_order(head.commit_id, merge_base)
            })
            .unwrap_or_default();
        let target_only_commits = target_head
            .as_ref()
            .map(|head| {
                self.branch_unique_commit_closure_by_commit_id_order(head.commit_id, merge_base)
            })
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

    /// Returns the common ancestor selected by the runtime's current history
    /// rule: intersect both ancestor sets, then choose the maximum `CommitId`.
    ///
    /// This is the actual 7A-certified behavior. Callers should not read richer
    /// merge semantics into this helper than the implementation proves.
    pub(crate) fn max_commit_id_common_ancestor(
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

    pub(super) fn ancestor_set(&self, start: CommitId) -> BTreeSet<CommitId> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![start];
        while let Some(commit_id) = stack.pop() {
            if !seen.insert(commit_id) {
                continue;
            }
            if let Some(artifact) = self.runtime.history.commit_catalog.get(commit_id) {
                stack.extend(artifact.envelope().commit.parents.iter().copied());
            }
        }
        self.runtime
            .performance_access()
            .count_merge_history_ancestry_traversal(seen.len());
        seen
    }

    /// Returns the branch-local ancestor closure for `head` after removing the
    /// merge-base ancestor closure, ordered by ascending `CommitId`.
    fn branch_unique_commit_closure_by_commit_id_order(
        &self,
        head: CommitId,
        merge_base: Option<CommitId>,
    ) -> Vec<CommitId> {
        let mut commits = self.ancestor_set(head).into_iter().collect::<Vec<_>>();
        if let Some(merge_base) = merge_base {
            let base_ancestors = self.ancestor_set(merge_base);
            commits.retain(|commit_id| !base_ancestors.contains(commit_id));
        }
        debug_assert!(commits.windows(2).all(|window| window[0] <= window[1]));
        commits
    }

    fn merge_conflicts_between(
        &self,
        left_commits: &[CommitId],
        right_commits: &[CommitId],
    ) -> Vec<MergeConflictRecord> {
        let left_records = self.commit_record_set(left_commits);
        let right_records = self.commit_record_set(right_commits);
        left_records.intersection(&right_records).cloned().collect()
    }

    fn commit_record_set(&self, commits: &[CommitId]) -> BTreeSet<MergeConflictRecord> {
        commits
            .iter()
            .filter_map(|commit_id| {
                self.runtime
                    .history
                    .commit_catalog
                    .get(*commit_id)
                    .map(|artifact| artifact.envelope())
            })
            .flat_map(|envelope| envelope.touched_record_refs().into_iter())
            .map(|record_ref| match record_ref {
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
