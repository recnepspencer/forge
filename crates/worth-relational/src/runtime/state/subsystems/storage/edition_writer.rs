use std::sync::{Arc, RwLockWriteGuard};

use crate::identity::data::PartitionId;
use crate::storage::overlay::PartitionState;

use super::edition_copy_lane::PartitionEditionCopyLane;
use super::partition_edition::{PartitionEdition, PartitionMap};
use crate::runtime::state::subsystems::RuntimeInstrumentation;

/// Exclusive authority to install the next edition of the record substrate.
///
/// The guard deliberately exposes no `DerefMut`. Every mutable entry point can
/// copy the map spine, and a copy that reaches Theta(partitions) must not hide
/// behind something that reads like a field access.
///
/// The spine is copied at most once per guard, and only when a reader edition
/// of the same map is still outstanding. When nothing observes the substrate,
/// mutation happens in place and no copy occurs at all.
pub(crate) struct PartitionEditionWriter<'subsystem> {
    edition: RwLockWriteGuard<'subsystem, PartitionEdition>,
    instrumentation: &'subsystem RuntimeInstrumentation,
    lane: PartitionEditionCopyLane,
    charged: bool,
}

impl<'subsystem> PartitionEditionWriter<'subsystem> {
    pub(super) fn new(
        edition: RwLockWriteGuard<'subsystem, PartitionEdition>,
        instrumentation: &'subsystem RuntimeInstrumentation,
        lane: PartitionEditionCopyLane,
    ) -> Self {
        Self {
            edition,
            instrumentation,
            lane,
            charged: false,
        }
    }

    /// Exclusive access to the whole map, copying the spine once if observed.
    pub(crate) fn map_mut(&mut self) -> &mut PartitionMap {
        let (map, copied) = self.edition.map_mut();
        if copied && !self.charged {
            self.charged = true;
            self.lane.charge_spine_copy(self.instrumentation);
        }
        map
    }

    /// Exclusive access to one partition's authoritative state, copying that
    /// partition out of structural sharing only when a reader still holds the
    /// previous edition of it.
    pub(crate) fn partition_mut(
        &mut self,
        partition_id: PartitionId,
    ) -> Option<&mut PartitionState> {
        self.map_mut().get_mut(&partition_id).map(Arc::make_mut)
    }

    pub(crate) fn partitions_mut(&mut self) -> impl Iterator<Item = &mut PartitionState> {
        self.map_mut().values_mut().map(Arc::make_mut)
    }

    pub(crate) fn insert(&mut self, partition_id: PartitionId, partition: PartitionState) {
        self.map_mut().insert(partition_id, Arc::new(partition));
    }

    pub(crate) fn remove(&mut self, partition_id: PartitionId) -> Option<Arc<PartitionState>> {
        self.map_mut().remove(&partition_id)
    }

    pub(crate) fn install(&mut self, partitions: PartitionMap) {
        *self.edition = PartitionEdition::new(partitions);
    }
}
