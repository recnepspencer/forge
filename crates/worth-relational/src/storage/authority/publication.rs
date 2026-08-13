use std::collections::BTreeMap;

use crate::storage::overlay::{PartitionCloneMode, PartitionMutationJournal, PartitionState};

use super::StorageAuthority;

mod execution;
mod plan;

impl<'runtime> StorageAuthority<'runtime> {
    pub(crate) fn publish_partition_commits(
        &mut self,
        clone_mode: PartitionCloneMode,
        committed_partitions: BTreeMap<
            crate::identity::data::PartitionId,
            (PartitionState, PartitionMutationJournal),
        >,
    ) {
        let existing_partition_ids = self.runtime.partitions.keys().copied().collect();
        let plan = plan::plan_partition_publication(
            clone_mode,
            &existing_partition_ids,
            committed_partitions,
        );
        execution::execute_partition_publication(self, plan);
    }
}
