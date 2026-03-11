use std::collections::BTreeMap;

use crate::identity::data::PartitionId;

use super::PartitionState;

pub(crate) trait PartitionAccess {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState>;
    fn partition_ids(&self) -> Vec<PartitionId>;

    fn touched_entity_slots(&self, _partition_id: PartitionId) -> Option<Vec<usize>> {
        None
    }

    fn touched_relation_slots(&self, _partition_id: PartitionId) -> Option<Vec<usize>> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BorrowedWorkingState<'a> {
    partitions: &'a BTreeMap<PartitionId, PartitionState>,
}

impl<'a> BorrowedWorkingState<'a> {
    pub(crate) fn new(partitions: &'a BTreeMap<PartitionId, PartitionState>) -> Self {
        Self { partitions }
    }
}

impl PartitionAccess for BorrowedWorkingState<'_> {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState> {
        self.partitions.get(&partition_id)
    }

    fn partition_ids(&self) -> Vec<PartitionId> {
        self.partitions.keys().copied().collect()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OverlayStateView<'a, S> {
    base_partitions: &'a BTreeMap<PartitionId, PartitionState>,
    staged: &'a S,
}

impl<'a, S> OverlayStateView<'a, S> {
    pub(crate) fn new(
        base_partitions: &'a BTreeMap<PartitionId, PartitionState>,
        staged: &'a S,
    ) -> Self {
        Self {
            base_partitions,
            staged,
        }
    }
}

impl<S: PartitionAccess> PartitionAccess for OverlayStateView<'_, S> {
    fn get_partition(&self, partition_id: PartitionId) -> Option<&PartitionState> {
        self.staged
            .get_partition(partition_id)
            .or_else(|| self.base_partitions.get(&partition_id))
    }

    fn partition_ids(&self) -> Vec<PartitionId> {
        let mut partition_ids = self.base_partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in self.staged.partition_ids() {
            if !partition_ids.contains(&partition_id) {
                partition_ids.push(partition_id);
            }
        }
        partition_ids.sort();
        partition_ids
    }

    fn touched_entity_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.staged.touched_entity_slots(partition_id)
    }

    fn touched_relation_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.staged.touched_relation_slots(partition_id)
    }
}
