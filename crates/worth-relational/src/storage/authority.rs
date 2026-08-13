mod pinning;
mod publication;
mod retention;

use crate::identity::data::{PartitionId, RecordId};
use crate::runtime::RelationalRuntime;
use crate::storage::substrate::RecordKind;

pub struct StorageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn storage_authority(&mut self) -> StorageAuthority<'_> {
        StorageAuthority::new(self)
    }
}

impl<'runtime> StorageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}

pub(super) fn partition_of<K: RecordKind>(id: &RecordId<K::Domain>) -> PartitionId {
    id.partition_id
}

pub(super) fn slot_of<K: RecordKind>(id: &RecordId<K::Domain>) -> usize {
    id.slot_index()
}
