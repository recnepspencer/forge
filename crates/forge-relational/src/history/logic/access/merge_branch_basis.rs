use std::sync::Arc;

use crate::history::data::{
    BranchId, MergeBaseSelectionRule, RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial,
    ResolvedMergeBase,
};

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    pub fn resolve_merge_branch_basis(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> Result<RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial> {
        let target_head = self.branch_head(target_branch).cloned().ok_or_else(|| {
            RelationalMergeBranchBasisDenial::MissingTargetHead {
                branch_id: target_branch.clone(),
            }
        })?;
        let source_head = self.branch_head(source_branch).cloned().ok_or_else(|| {
            RelationalMergeBranchBasisDenial::MissingSourceHead {
                branch_id: source_branch.clone(),
            }
        })?;

        let left_ancestors = self.ancestor_closure_by_commit_id_order(target_head.commit_id);
        let right_ancestors = self.ancestor_closure_by_commit_id_order(source_head.commit_id);
        let merge_base_commit_id = self
            .max_commit_id_common_ancestor(target_head.commit_id, source_head.commit_id)
            .ok_or_else(|| RelationalMergeBranchBasisDenial::MissingMergeBase {
                source_branch: source_branch.clone(),
                target_branch: target_branch.clone(),
            })?;
        let merge_base = self
            .commit_envelope(merge_base_commit_id)
            .map(|envelope| envelope.commit.clone())
            .ok_or(RelationalMergeBranchBasisDenial::MissingMergeBaseEnvelope {
                commit_id: merge_base_commit_id,
            })?;

        Ok(RelationalMergeBranchBasis {
            source_head,
            target_head,
            merge_base: ResolvedMergeBase {
                rule: MergeBaseSelectionRule::MaxCommitIdCommonAncestor,
                commit: merge_base,
                supporting_left_ancestors: Arc::from(left_ancestors),
                supporting_right_ancestors: Arc::from(right_ancestors),
            },
        })
    }
}
