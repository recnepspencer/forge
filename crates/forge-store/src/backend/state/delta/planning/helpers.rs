use crate::failure::{StoreError, StoreErrorKind};
use forge_relational::facade::history::{BranchId, CommitId};

use crate::backend::records::{BranchDeltaLayerRecord, StoreState};

impl StoreState {
    pub(crate) fn trace_linear_branch_segment(
        &self,
        branch_id: &BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_commit_id: CommitId,
    ) -> Result<Vec<CommitId>, StoreError> {
        if Some(target_commit_id) == base_frontier_commit_id {
            return Ok(Vec::new());
        }

        let mut reversed = Vec::new();
        let mut current = target_commit_id;
        loop {
            let record = self.commit_record(current).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!("branch delta traversal missing commit {}", current.0),
                )
            })?;
            if record.envelope.branch_context != *branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReadTargetIllegal,
                    format!(
                        "commit {} drifted onto branch `{}` during branch delta planning",
                        current.0, record.envelope.branch_context.0
                    ),
                ));
            }
            reversed.push(current);

            match record.envelope.commit.parents.as_slice() {
                [] => {
                    if base_frontier_commit_id.is_none() {
                        break;
                    }
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaReadTargetIllegal,
                        format!(
                            "target commit {} does not descend from the branch basis",
                            target_commit_id.0
                        ),
                    ));
                }
                [parent] => {
                    if Some(*parent) == base_frontier_commit_id {
                        break;
                    }
                    current = *parent;
                }
                _ => {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaTargetRequiresMergeAwareWidening,
                        format!(
                            "target commit {} requires merge-aware target widening, which milestone 5 does not admit",
                            target_commit_id.0
                        ),
                    ));
                }
            }
        }

        reversed.reverse();
        Ok(reversed)
    }

    pub(crate) fn find_covering_branch_delta_layer(
        &self,
        branch_id: &BranchId,
        base_frontier_commit_id: Option<CommitId>,
        remaining_commit_ids: &[CommitId],
    ) -> Option<&BranchDeltaLayerRecord> {
        self.branch_delta_layer_records
            .values()
            .filter(|record| {
                record.branch_id == *branch_id
                    && record.base_frontier_commit_id == base_frontier_commit_id
                    && remaining_commit_ids.starts_with(&record.commit_ids)
            })
            .max_by_key(|record| record.commit_ids.len())
    }
}

pub(crate) fn regime_for_commit_span(commit_span: usize) -> crate::delta::BranchDeltaReadRegime {
    if commit_span <= 8 {
        crate::delta::BranchDeltaReadRegime::Sparse
    } else {
        crate::delta::BranchDeltaReadRegime::Dense
    }
}
