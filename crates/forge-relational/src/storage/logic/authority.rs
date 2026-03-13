use std::collections::BTreeMap;

use crate::identity::data::{PartitionId, RecordId, VersionBound, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RecordLifecycleState;
use crate::storage::overlay::{PartitionState, WorkingState};
use crate::storage::substrate::{HistoricalMetadata, PinClass, RecordKind};

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
            self.runtime
                .partitions
                .insert(partition_id, partition_state);
        }
    }

    pub(crate) fn working_state_for_touched_partitions(
        &self,
        touched_partitions: impl IntoIterator<Item = crate::identity::data::PartitionId>,
    ) -> WorkingState {
        WorkingState::from_touched_partitions(
            &self.runtime.partitions,
            touched_partitions,
            self.runtime.config.storage.adjacency_policy.clone(),
        )
    }

    pub(crate) fn clear_named_pins(&mut self, class: PinClass) {
        for partition in self.runtime.partitions.values_mut() {
            partition.entity_arena.clear_named_pins(class);
            partition.relation_arena.clear_named_pins(class);
        }
    }

    pub(crate) fn pin_snapshot_record<K: RecordKind>(&mut self, record_id: RecordId<K::Domain>) {
        let slot = slot_of::<K>(&record_id);
        let Some(partition) = self
            .runtime
            .partitions
            .get_mut(&partition_of::<K>(&record_id))
        else {
            return;
        };
        let arena = K::arena_mut(partition);
        if arena.snapshot_pin_count(slot).is_none() {
            return;
        }
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.snapshot_pin_adjustments += 1);
        arena.increment_snapshot_pin(slot);
        if arena.retired_at_for_slot(slot).is_some() {
            arena.set_lifecycle_for_slot(slot, RecordLifecycleState::PinnedBySnapshot);
        }
    }

    pub(crate) fn unpin_snapshot_record<K: RecordKind>(
        &mut self,
        record_id: RecordId<K::Domain>,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        let Some(partition) = self.runtime.partitions.get_mut(&partition_id) else {
            return;
        };
        let arena = K::arena_mut(partition);
        if arena.snapshot_pin_count(slot).unwrap_or(0) == 0 {
            return;
        }
        self.runtime
            .services
            .instrumentation
            .count(|counters| counters.snapshot_pin_adjustments += 1);
        arena.decrement_snapshot_pin(slot);
        let retired_at = arena.retired_at_for_slot(slot);
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn adjust_named_pin<K: RecordKind>(
        &mut self,
        record_id: RecordId<K::Domain>,
        class: PinClass,
        delta: i32,
        retention_fence: VersionId,
    ) {
        let slot = slot_of::<K>(&record_id);
        let partition_id = partition_of::<K>(&record_id);
        let Some(partition_len) = self
            .runtime
            .partitions
            .get(&partition_id)
            .map(|partition| K::arena(partition).slot_count())
        else {
            return;
        };
        if slot >= partition_len {
            return;
        }
        {
            let partition = self
                .runtime
                .partitions
                .get_mut(&partition_id)
                .expect("pin partition present");
            let arena = K::arena_mut(partition);
            if arena.snapshot_pin_count(slot).is_none() {
                return;
            }
            if let Some(pin_count) = arena.adjust_named_pin(slot, class) {
                *pin_count = pin_count.saturating_add_signed(delta);
            }
        }
        let retired_at = self
            .runtime
            .partitions
            .get(&partition_id)
            .and_then(|partition| K::arena(partition).retired_at_for_slot(slot));
        self.refresh_retention_state::<K>(partition_id, slot, retired_at, retention_fence);
    }

    pub(crate) fn refresh_retention_state<K: RecordKind>(
        &mut self,
        partition_id: PartitionId,
        slot: usize,
        retired_at: Option<VersionId>,
        retention_fence: VersionId,
    ) {
        let Some(_retired_at) = retired_at else {
            return;
        };
        let partition = self
            .runtime
            .partitions
            .get_mut(&partition_id)
            .expect("retention partition present");
        let arena = K::arena_mut(partition);
        let lifecycle = match self.runtime.config.storage.retention.backend {
            crate::config::data::RetentionBackend::PinTrackedRetention => {
                if arena.snapshot_pin_count(slot).unwrap_or(0) > 0 {
                    RecordLifecycleState::PinnedBySnapshot
                } else if arena.branch_pin_count(slot).unwrap_or(0) > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if arena.replay_pin_count(slot).unwrap_or(0) > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else {
                    RecordLifecycleState::Reclaimable
                }
            }
            crate::config::data::RetentionBackend::EpochChunkRetention => {
                if arena.branch_pin_count(slot).unwrap_or(0) > 0 {
                    RecordLifecycleState::PinnedByBranch
                } else if arena.replay_pin_count(slot).unwrap_or(0) > 0 {
                    RecordLifecycleState::PinnedByReplayRetention
                } else if retired_at.is_some_and(|retired| {
                    !VersionBound::new(retention_fence).retains_retired(retired)
                }) {
                    RecordLifecycleState::Reclaimable
                } else {
                    RecordLifecycleState::PinnedBySnapshot
                }
            }
        };
        arena.set_lifecycle_for_slot(slot, lifecycle);
    }

    pub(crate) fn reclaim_record_if_reclaimable<K: RecordKind>(
        &mut self,
        partition_id: PartitionId,
        slot: usize,
    ) -> bool {
        let Some(partition) = self.runtime.partitions.get_mut(&partition_id) else {
            return false;
        };
        let arena = K::arena_mut(partition);
        let Some(slot_view) = arena.get_slot(slot) else {
            return false;
        };
        if slot_view.lifecycle() != RecordLifecycleState::Reclaimable {
            return false;
        }
        arena.set_lifecycle_for_slot(slot, RecordLifecycleState::Reusable);
        arena.reset_slot(slot);
        true
    }

    pub(crate) fn trim_live_history<K: RecordKind>(
        &mut self,
        slots_by_partition: std::collections::BTreeMap<
            PartitionId,
            std::collections::BTreeSet<usize>,
        >,
        oldest_pinned_version: VersionId,
    ) -> usize
    where
        K::Meta: crate::storage::substrate::HistoricalMetadata,
    {
        let mut total_trimmed = 0usize;
        for (partition_id, slots) in slots_by_partition {
            let Some(partition) = self.runtime.partitions.get_mut(&partition_id) else {
                continue;
            };
            let arena = K::arena_mut(partition);
            for slot in slots {
                if arena
                    .get_slot(slot)
                    .is_none_or(|slot_view| slot_view.lifecycle() != RecordLifecycleState::Live)
                {
                    continue;
                }
                if arena
                    .metadata_history_at(slot)
                    .is_some_and(|metadata_history| metadata_history.len() > 1)
                {
                    continue;
                }
                let bound = VersionBound::new(oldest_pinned_version);
                let original_len = match arena.payload_history_at(slot) {
                    Some(history) => history.len(),
                    None => continue,
                };
                let trimmed_len = {
                    let Some(history) = arena.payload_history_at_mut(slot) else {
                        continue;
                    };
                    history.retain(|entry| {
                        entry
                            .retired_at
                            .is_none_or(|retired| bound.retains_retired(retired))
                    });
                    history.len()
                };
                if let Some(metadata_history) = arena.metadata_history_at_mut(slot) {
                    metadata_history.retain(|entry| {
                        entry
                            .retired_at()
                            .is_none_or(|retired| bound.retains_retired(retired))
                    });
                }
                total_trimmed += original_len.saturating_sub(trimmed_len);
            }
        }
        total_trimmed
    }
}

fn partition_of<K: RecordKind>(id: &RecordId<K::Domain>) -> PartitionId {
    id.partition_id
}

fn slot_of<K: RecordKind>(id: &RecordId<K::Domain>) -> usize {
    id.local_slot.0 as usize
}
