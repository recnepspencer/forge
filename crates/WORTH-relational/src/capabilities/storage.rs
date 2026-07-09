use crate::storage::logic::state::PartitionAccess;

pub(crate) trait StorageRead: PartitionAccess {}

impl<T: PartitionAccess + ?Sized> StorageRead for T {}
