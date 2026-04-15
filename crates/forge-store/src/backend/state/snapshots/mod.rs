mod basis;
mod image;
mod read;
mod restore;

use std::collections::{BTreeSet, VecDeque};

use forge_relational::facade::history::CommitId;

use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    fn snapshot_history_range(
        &self,
        frontier_commit_id: CommitId,
    ) -> Result<Vec<CommitId>, StoreError> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([frontier_commit_id]);
        while let Some(commit_id) = queue.pop_front() {
            if !visited.insert(commit_id) {
                continue;
            }
            let record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot frontier closure missing commit {}", commit_id.0),
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
                        StoreError::backend_integrity("snapshot history range record missing")
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        ordered.sort_by_key(|(sequence, _)| *sequence);
        Ok(ordered
            .into_iter()
            .map(|(_, commit_id)| commit_id)
            .collect())
    }

    fn is_descendant_of(&self, target: CommitId, ancestor: CommitId) -> Result<bool, StoreError> {
        if target == ancestor {
            return Ok(true);
        }
        let mut queue = VecDeque::from([target]);
        let mut visited = BTreeSet::new();
        while let Some(commit_id) = queue.pop_front() {
            if !visited.insert(commit_id) {
                continue;
            }
            let record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotTailRangeGap,
                    format!("tail traversal missing commit {}", commit_id.0),
                )
            })?;
            for parent_id in &record.envelope.commit.parents {
                if *parent_id == ancestor {
                    return Ok(true);
                }
                queue.push_back(*parent_id);
            }
        }
        Ok(false)
    }
}
