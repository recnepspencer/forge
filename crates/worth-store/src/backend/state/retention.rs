use std::collections::{BTreeMap, BTreeSet, VecDeque};

use worth_relational::facade::history::{BranchId, CommitId};

use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub(crate) fn retention_closure_from_frontiers(
        &self,
        frontier_commit_ids: impl IntoIterator<Item = CommitId>,
    ) -> Result<Vec<CommitId>, StoreError> {
        let mut visited = BTreeSet::new();
        let mut queue = frontier_commit_ids.into_iter().collect::<VecDeque<_>>();
        while let Some(commit_id) = queue.pop_front() {
            if !visited.insert(commit_id) {
                continue;
            }
            let record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::RetentionClosureBasisMissing,
                    format!("retention closure missing commit {}", commit_id.0),
                )
            })?;
            for parent_id in &record.envelope.commit.parents {
                queue.push_back(*parent_id);
            }
        }

        let mut ordered = visited
            .into_iter()
            .map(|commit_id| {
                self.commit_record(commit_id)
                    .map(|record| (record.commit_sequence, commit_id))
                    .ok_or_else(|| {
                        StoreError::backend_integrity(
                            "retention closure record disappeared during ordering",
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        ordered.sort_by_key(|(sequence, _)| *sequence);
        Ok(ordered
            .into_iter()
            .map(|(_, commit_id)| commit_id)
            .collect())
    }

    pub(crate) fn branch_commit_sequences(&self, branch_id: &BranchId) -> BTreeMap<u64, CommitId> {
        self.commit_envelopes
            .values()
            .filter(|record| &record.envelope.branch_context == branch_id)
            .map(|record| (record.commit_sequence, record.envelope.commit.commit_id))
            .collect()
    }
}
