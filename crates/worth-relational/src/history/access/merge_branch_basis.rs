use std::sync::Arc;

use crate::branch::RelationalLegacyBranchBinding;
use crate::history::data::{
    BranchId, MergeBaseSelectionRule, RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial,
    ResolvedMergeBase,
};
use worth_foundational::FoundationalBranchTarget;

use super::HistoryAccess;

impl<'runtime> HistoryAccess<'runtime> {
    /// Transitional immutable merge-history projection for existing Query
    /// consumers. Both descriptive branch names are owner-bound before the
    /// basis is resolved; no raw id can select a transaction target.
    pub fn historical_merge_branch_basis(
        &self,
        source_branch: &BranchId,
        target_branch: &BranchId,
    ) -> Result<RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial> {
        let source_identity = self.runtime.branch_identity(source_branch).map_err(|_| {
            RelationalMergeBranchBasisDenial::MissingSourceHead {
                branch_id: source_branch.clone(),
            }
        })?;
        let target_identity = self.runtime.branch_identity(target_branch).map_err(|_| {
            RelationalMergeBranchBasisDenial::MissingTargetHead {
                branch_id: target_branch.clone(),
            }
        })?;
        let source_binding = self
            .runtime
            .legacy_branch_binding_for_identity(&source_identity)
            .map_err(|_| RelationalMergeBranchBasisDenial::MissingSourceHead {
                branch_id: source_branch.clone(),
            })?;
        let target_binding = self
            .runtime
            .legacy_branch_binding_for_identity(&target_identity)
            .map_err(|_| RelationalMergeBranchBasisDenial::MissingTargetHead {
                branch_id: target_branch.clone(),
            })?;
        self.resolve_merge_branch_basis_from_bindings(&source_binding, &target_binding)
    }

    pub(crate) fn resolve_merge_branch_basis_from_bindings(
        &self,
        source_binding: &RelationalLegacyBranchBinding,
        target_binding: &RelationalLegacyBranchBinding,
    ) -> Result<RelationalMergeBranchBasis, RelationalMergeBranchBasisDenial> {
        let source_head = self.commit_receipt_for_binding(source_binding, false)?;
        let target_head = self.commit_receipt_for_binding(target_binding, true)?;
        let source_branch = source_binding.identity().branch_id().clone();
        let target_branch = target_binding.identity().branch_id().clone();
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

    fn commit_receipt_for_binding(
        &self,
        binding: &RelationalLegacyBranchBinding,
        target: bool,
    ) -> Result<crate::history::data::RelationalCommitReceipt, RelationalMergeBranchBasisDenial>
    {
        let branch_id = binding.identity().branch_id().clone();
        let missing = |branch_id: BranchId| {
            if target {
                RelationalMergeBranchBasisDenial::MissingTargetHead { branch_id }
            } else {
                RelationalMergeBranchBasisDenial::MissingSourceHead { branch_id }
            }
        };
        let cell = self
            .runtime
            .history
            .branch_cell(&branch_id)
            .ok_or_else(|| missing(branch_id.clone()))?;
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != binding.truth_version()
        {
            return Err(missing(branch_id));
        }
        let FoundationalBranchTarget::Basis(target) = binding.observation().target() else {
            return Err(missing(branch_id));
        };
        self.runtime
            .history
            .commit_catalog
            .get(crate::history::data::CommitId(target.commit_id()))
            .map(|artifact| artifact.envelope().commit.clone())
            .ok_or_else(|| missing(branch_id))
    }

    #[cfg(test)]
    pub(crate) fn resolve_merge_branch_basis(
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
