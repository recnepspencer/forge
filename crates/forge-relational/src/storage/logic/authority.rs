use std::collections::BTreeMap;

use crate::logic::runtime::RelationalRuntime;
use crate::storage::overlay::PartitionState;

pub struct StorageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn storage_authority(&mut self) -> StorageAuthority<'_> {
        StorageAuthority::new(self)
    }
}

impl<'runtime> StorageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub(crate) fn publish_partitions(
        &mut self,
        committed_partitions: BTreeMap<crate::identity::data::PartitionId, PartitionState>,
    ) {
        for (partition_id, partition_state) in committed_partitions {
            self.runtime.partitions.insert(partition_id, partition_state);
        }
    }
}
