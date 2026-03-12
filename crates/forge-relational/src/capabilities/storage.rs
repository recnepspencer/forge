use crate::identity::data::PartitionId;
use crate::storage::logic::state::PartitionAccess;
use crate::storage::overlay::{PartitionState, WorkingState};

pub(crate) trait StorageRead: PartitionAccess {}

impl<T: PartitionAccess + ?Sized> StorageRead for T {}

#[allow(dead_code)]
pub(crate) trait StorageWrite: StorageRead {
    fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState;
}

impl StorageWrite for WorkingState {
    fn get_partition_mut(&mut self, partition_id: PartitionId) -> &mut PartitionState {
        WorkingState::get_partition_mut(self, partition_id)
    }
}
