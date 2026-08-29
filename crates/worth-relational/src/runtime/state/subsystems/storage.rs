use std::collections::BTreeMap;
use std::sync::{Arc, RwLockWriteGuard};

use crate::identity::data::PartitionId;
use crate::runtime::state::subsystems::{RuntimeOwnedState, RuntimeSubsystem};
use crate::storage::overlay::PartitionState;

/// The authoritative record substrate keyed by partition, structurally shared
/// so that a reader projects truth without blocking the executor that is
/// installing the next published partition.
pub(crate) type PartitionMap = BTreeMap<PartitionId, Arc<PartitionState>>;

/// The runtime's record substrate, owned behind its own lock.
///
/// Readers never retain the lock: `read` hands back a structurally shared
/// snapshot and releases immediately, so no read path can deadlock against a
/// settling executor and no lock is ever held across derived work. A writer
/// copies exactly the partitions it publishes, and only while a reader still
/// holds the previous edition of one.
#[derive(Debug, Default)]
pub(crate) struct StorageSubsystem {
    partitions: RuntimeOwnedState<PartitionMap>,
}

impl StorageSubsystem {
    pub(crate) fn read(&self) -> PartitionMap {
        self.partitions.read().clone()
    }

    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, PartitionMap> {
        self.partitions.write()
    }

    pub(crate) fn share(&self) -> Self {
        Self {
            partitions: self.partitions.share(),
        }
    }

    pub(crate) fn replace(&self, partitions: PartitionMap) {
        *self.write() = partitions;
    }

    pub(crate) fn partition_ids(&self) -> Vec<PartitionId> {
        self.partitions.read().keys().copied().collect()
    }

    pub(crate) fn partition(&self, partition_id: PartitionId) -> Option<Arc<PartitionState>> {
        self.partitions.read().get(&partition_id).map(Arc::clone)
    }

    pub(crate) fn contains(&self, partition_id: PartitionId) -> bool {
        self.partitions.read().contains_key(&partition_id)
    }

    /// Owned copies of every partition, for checkpoint capture and recovery.
    pub(crate) fn owned_partitions(&self) -> Vec<PartitionState> {
        self.partitions
            .read()
            .values()
            .map(|partition| partition.as_ref().clone())
            .collect()
    }

    pub(crate) fn install_owned(
        &self,
        partitions: impl IntoIterator<Item = (PartitionId, PartitionState)>,
    ) {
        self.replace(
            partitions
                .into_iter()
                .map(|(partition_id, partition)| (partition_id, Arc::new(partition)))
                .collect(),
        );
    }
}

/// Exclusive access to one partition's authoritative state, copying it out of
/// structural sharing only when a reader still holds the previous edition.
pub(crate) fn partition_entry_mut<'map>(
    partitions: &'map mut PartitionMap,
    partition_id: PartitionId,
) -> Option<&'map mut PartitionState> {
    partitions
        .get_mut(&partition_id)
        .map(|partition| Arc::make_mut(partition))
}

impl RuntimeSubsystem for StorageSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        let mut partitions = self.read();
        for partition in partitions.values_mut() {
            Arc::make_mut(partition).clear_runtime_pin_counters();
        }
        Self {
            partitions: RuntimeOwnedState::new(partitions),
        }
    }
}
