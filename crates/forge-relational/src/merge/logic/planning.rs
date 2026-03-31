use std::collections::BTreeMap;
use std::sync::Arc;

use crate::history::data::{BranchId, CommitId};
use crate::merge::data::{
    BranchCommitDelta, BranchDeltaSummary, BranchTouchedRecordDelta, HistoryScopedMergePlan,
    MergeBaseSelectionRule, MergePlanningError, MergePlanningRequest, ResolvedMergeBase,
};
use crate::merge::logic::MergeAccess;
use crate::transactions::data::RecordRef;

impl<'runtime> MergeAccess<'runtime> {
    pub(crate) fn plan_history_scope(
        &self,
        request: MergePlanningRequest,
    ) -> Result<HistoryScopedMergePlan, MergePlanningError> {
        let history = self.runtime.history_access();
        let target_head = history
            .branch_head(&request.target_branch)
            .cloned()
            .ok_or_else(|| MergePlanningError::MissingTargetHead {
                branch_id: request.target_branch.clone(),
            })?;
        let source_head = history
            .branch_head(&request.source_branch)
            .cloned()
            .ok_or_else(|| MergePlanningError::MissingSourceHead {
                branch_id: request.source_branch.clone(),
            })?;

        let left_ancestors = history.ancestor_closure_by_commit_id_order(target_head.commit_id);
        let right_ancestors = history.ancestor_closure_by_commit_id_order(source_head.commit_id);
        let merge_base = history
            .max_commit_id_common_ancestor(target_head.commit_id, source_head.commit_id)
            .ok_or_else(|| MergePlanningError::MissingMergeBase {
                source_branch: request.source_branch.clone(),
                target_branch: request.target_branch.clone(),
            })?;
        let merge_base_ancestors = history.ancestor_closure_by_commit_id_order(merge_base);

        let target_delta_commits = self.branch_local_unique_commit_closure_by_commit_id_order(
            request.target_branch.clone(),
            left_ancestors.as_slice(),
            merge_base_ancestors.as_slice(),
        );
        let source_delta_commits = self.branch_local_unique_commit_closure_by_commit_id_order(
            request.source_branch.clone(),
            right_ancestors.as_slice(),
            merge_base_ancestors.as_slice(),
        );

        Ok(HistoryScopedMergePlan {
            request: request.clone(),
            target_head,
            source_head,
            merge_base: ResolvedMergeBase {
                rule: MergeBaseSelectionRule::MaxCommitIdCommonAncestor,
                commit_id: merge_base,
                supporting_left_ancestors: Arc::from(left_ancestors),
                supporting_right_ancestors: Arc::from(right_ancestors),
            },
            target_delta: self
                .branch_commit_delta(request.target_branch, target_delta_commits.as_slice()),
            source_delta: self
                .branch_commit_delta(request.source_branch, source_delta_commits.as_slice()),
        })
    }

    fn branch_commit_delta(
        &self,
        branch_id: crate::history::data::BranchId,
        commits: &[CommitId],
    ) -> BranchCommitDelta {
        let history = self.runtime.history_access();
        let mut touched_records = BTreeMap::new();
        for commit_id in commits {
            if let Some(envelope) = history.commit_envelope(*commit_id) {
                for record in &envelope.patch.records {
                    touched_records
                        .entry(record.target.clone())
                        .or_insert_with(Vec::new)
                        .push(*commit_id);
                }
                for intent in &envelope.merged_plan.merged_intents {
                    let Some(target) = intent.existing_record_target().map(|target| match target {
                        crate::transactions::data::ExistingRecordTarget::Entity(entity_id) => {
                            RecordRef::Entity(entity_id)
                        }
                        crate::transactions::data::ExistingRecordTarget::Relation(relation_id) => {
                            RecordRef::Relation(relation_id)
                        }
                    }) else {
                        continue;
                    };
                    let commit_ids = touched_records.entry(target).or_insert_with(Vec::new);
                    if !commit_ids.contains(commit_id) {
                        commit_ids.push(*commit_id);
                    }
                }
            }
        }

        let touched_records = touched_records
            .into_iter()
            .map(|(target, commit_ids)| BranchTouchedRecordDelta {
                target,
                commit_ids: Arc::from(commit_ids),
            })
            .collect::<Vec<_>>();

        BranchCommitDelta {
            branch_id,
            commits: Arc::from(commits.to_vec()),
            touched_records: Arc::from(touched_records),
        }
    }

    fn branch_local_unique_commit_closure_by_commit_id_order(
        &self,
        branch_id: BranchId,
        commits: &[CommitId],
        merge_base_ancestors: &[CommitId],
    ) -> Vec<CommitId> {
        let history = self.runtime.history_access();
        let merge_base_ancestors = merge_base_ancestors
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        commits
            .iter()
            .copied()
            .filter(|commit_id| !merge_base_ancestors.contains(commit_id))
            .filter(|commit_id| {
                history
                    .commit_envelope(*commit_id)
                    .map(|envelope| envelope.commit.branch_id == branch_id)
                    .unwrap_or(false)
            })
            .collect()
    }
}

pub(super) fn branch_delta_summary(
    head: &crate::history::data::CommitReference,
    delta: &BranchCommitDelta,
) -> BranchDeltaSummary {
    let touched_entity_count = delta
        .touched_records
        .iter()
        .filter(|record| matches!(record.target, RecordRef::Entity(_)))
        .count();
    let touched_relation_count = delta
        .touched_records
        .iter()
        .filter(|record| matches!(record.target, RecordRef::Relation(_)))
        .count();
    BranchDeltaSummary {
        branch_id: delta.branch_id.clone(),
        head_commit_id: head.commit_id,
        head_version_id: head.version_id,
        unique_commit_count: delta.commits.len(),
        touched_record_count: delta.touched_records.len(),
        touched_entity_count,
        touched_relation_count,
    }
}
