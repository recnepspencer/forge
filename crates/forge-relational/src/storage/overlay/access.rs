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
        let base_ids = self.base_partitions.keys().copied();
        let staged_ids = self.staged.partition_ids();
        debug_assert!(partition_ids_are_canonical(staged_ids.iter().copied()));

        let mut merged = Vec::with_capacity(self.base_partitions.len() + staged_ids.len());
        let mut base_iter = base_ids.peekable();
        let mut staged_iter = staged_ids.into_iter().peekable();

        loop {
            match (base_iter.peek().copied(), staged_iter.peek().copied()) {
                (Some(base), Some(staged)) => match base.cmp(&staged) {
                    std::cmp::Ordering::Less => {
                        merged.push(base);
                        base_iter.next();
                    }
                    std::cmp::Ordering::Greater => {
                        merged.push(staged);
                        staged_iter.next();
                    }
                    std::cmp::Ordering::Equal => {
                        merged.push(base);
                        base_iter.next();
                        staged_iter.next();
                    }
                },
                (Some(base), None) => {
                    merged.push(base);
                    base_iter.next();
                    merged.extend(base_iter);
                    break;
                }
                (None, Some(staged)) => {
                    merged.push(staged);
                    staged_iter.next();
                    merged.extend(staged_iter);
                    break;
                }
                (None, None) => break,
            }
        }

        merged
    }

    fn touched_entity_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.staged.touched_entity_slots(partition_id)
    }

    fn touched_relation_slots(&self, partition_id: PartitionId) -> Option<Vec<usize>> {
        self.staged.touched_relation_slots(partition_id)
    }
}

fn partition_ids_are_canonical(partition_ids: impl IntoIterator<Item = PartitionId>) -> bool {
    let mut partition_ids = partition_ids.into_iter();
    let Some(mut previous) = partition_ids.next() else {
        return true;
    };
    for current in partition_ids {
        if previous >= current {
            return false;
        }
        previous = current;
    }
    true
}
