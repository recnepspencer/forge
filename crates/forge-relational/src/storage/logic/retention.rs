use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::{RecordLifecycleState, RetentionPassOutcome, RetentionPlan};

impl RelationalRuntime {
    pub fn inspect_retention_plan(&mut self) -> RetentionPlan {
        let retention_fence = self.retention_fence_version(self.current_version_id());
        let mut branch_pinned_entities = 0;
        let mut replay_pinned_entities = 0;
        let mut snapshot_pinned_entities = 0;
        let mut branch_pinned_relations = 0;
        let mut replay_pinned_relations = 0;
        let mut snapshot_pinned_relations = 0;
        let mut reclaimable_entities = 0;
        let mut reclaimable_relations = 0;

        let partition_ids = self.partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in partition_ids {
            let entity_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.entity_arena.lifecycle.len())
                .unwrap_or(0);
            for slot in 0..entity_len {
                let retired_at = self
                    .partitions
                    .get(&partition_id)
                    .and_then(|partition| partition.entity_arena.retired_at[slot]);
                if retired_at.is_some() {
                    self.refresh_entity_retention_state(
                        partition_id,
                        slot,
                        retired_at,
                        retention_fence,
                    );
                }
                let lifecycle = self.partitions[&partition_id].entity_arena.lifecycle[slot];
                match lifecycle {
                    RecordLifecycleState::PinnedByBranch => branch_pinned_entities += 1,
                    RecordLifecycleState::PinnedByReplayRetention => replay_pinned_entities += 1,
                    RecordLifecycleState::PinnedBySnapshot => snapshot_pinned_entities += 1,
                    RecordLifecycleState::Reclaimable => reclaimable_entities += 1,
                    _ => {}
                }
            }

            let relation_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.relation_arena.lifecycle.len())
                .unwrap_or(0);
            for slot in 0..relation_len {
                let retired_at = self
                    .partitions
                    .get(&partition_id)
                    .and_then(|partition| partition.relation_arena.retired_at[slot]);
                if retired_at.is_some() {
                    self.refresh_relation_retention_state(
                        partition_id,
                        slot,
                        retired_at,
                        retention_fence,
                    );
                }
                let lifecycle = self.partitions[&partition_id].relation_arena.lifecycle[slot];
                match lifecycle {
                    RecordLifecycleState::PinnedByBranch => branch_pinned_relations += 1,
                    RecordLifecycleState::PinnedByReplayRetention => replay_pinned_relations += 1,
                    RecordLifecycleState::PinnedBySnapshot => snapshot_pinned_relations += 1,
                    RecordLifecycleState::Reclaimable => reclaimable_relations += 1,
                    _ => {}
                }
            }
        }

        RetentionPlan {
            retention_fence_version: retention_fence,
            active_snapshot_count: self.snapshots.active.len(),
            branch_pinned_entities,
            replay_pinned_entities,
            snapshot_pinned_entities,
            branch_pinned_relations,
            replay_pinned_relations,
            snapshot_pinned_relations,
            reclaimable_entities,
            reclaimable_relations,
        }
    }

    pub fn run_retention_pass(&mut self) -> RetentionPassOutcome {
        let mut outcome = RetentionPassOutcome {
            entity_reclaimable: 0,
            entity_reclaimed: 0,
            entity_chunks_scanned: 0,
            relation_reclaimable: 0,
            relation_reclaimed: 0,
            relation_chunks_scanned: 0,
        };

        let entity_chunk_size = self.config.storage_layout.entity_chunk_size.max(1);
        let relation_chunk_size = self.config.storage_layout.relation_chunk_size.max(1);
        let retention_fence = self.retention_fence_version(self.current_version_id());

        let partition_ids = self.partitions.keys().copied().collect::<Vec<_>>();
        for partition_id in partition_ids {
            let entity_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.entity_arena.lifecycle.len())
                .unwrap_or(0);
            for slot_start in (0..entity_len).step_by(entity_chunk_size) {
                outcome.entity_chunks_scanned += 1;
                let slot_end = (slot_start + entity_chunk_size).min(entity_len);
                for slot in slot_start..slot_end {
                    self.instrumentation
                        .complexity_counters
                        .borrow_mut()
                        .retention_entity_slots_scanned += 1;
                    let retired_at = self
                        .partitions
                        .get(&partition_id)
                        .and_then(|partition| partition.entity_arena.retired_at[slot]);
                    if let Some(version) = retired_at {
                        self.refresh_entity_retention_state(
                            partition_id,
                            slot,
                            Some(version),
                            retention_fence,
                        );
                        if self.partitions.get(&partition_id).is_some_and(|partition| {
                            partition.entity_arena.lifecycle[slot]
                                == RecordLifecycleState::Reclaimable
                        }) {
                            outcome.entity_reclaimable += 1;
                            if self.config.mvcc.auto_reclaim_deleted_records
                                && outcome.entity_reclaimed < self.config.mvcc.reclaim_batch_size
                            {
                                let partition = self
                                    .partitions
                                    .get_mut(&partition_id)
                                    .expect("entity partition for reclaim");
                                partition.entity_arena.lifecycle[slot] =
                                    RecordLifecycleState::Reusable;
                                partition.entity_arena.kind_ids[slot] = None;
                                partition.entity_arena.payloads[slot] = None;
                                partition.entity_arena.snapshot_pins[slot] = 0;
                                partition.entity_arena.branch_pins[slot] = 0;
                                partition.entity_arena.replay_pins[slot] = 0;
                                partition.entity_arena.retired_at[slot] = None;
                                partition.entity_arena.free_list.push(slot as u64);
                                outcome.entity_reclaimed += 1;
                            }
                        }
                    }
                }
            }

            let relation_len = self
                .partitions
                .get(&partition_id)
                .map(|partition| partition.relation_arena.lifecycle.len())
                .unwrap_or(0);
            for slot_start in (0..relation_len).step_by(relation_chunk_size) {
                outcome.relation_chunks_scanned += 1;
                let slot_end = (slot_start + relation_chunk_size).min(relation_len);
                for slot in slot_start..slot_end {
                    self.instrumentation
                        .complexity_counters
                        .borrow_mut()
                        .retention_relation_slots_scanned += 1;
                    let retired_at = self
                        .partitions
                        .get(&partition_id)
                        .and_then(|partition| partition.relation_arena.retired_at[slot]);
                    if let Some(version) = retired_at {
                        self.refresh_relation_retention_state(
                            partition_id,
                            slot,
                            Some(version),
                            retention_fence,
                        );
                        if self.partitions.get(&partition_id).is_some_and(|partition| {
                            partition.relation_arena.lifecycle[slot]
                                == RecordLifecycleState::Reclaimable
                        }) {
                            outcome.relation_reclaimable += 1;
                            if self.config.mvcc.auto_reclaim_deleted_records
                                && outcome.relation_reclaimed < self.config.mvcc.reclaim_batch_size
                            {
                                let partition = self
                                    .partitions
                                    .get_mut(&partition_id)
                                    .expect("relation partition for reclaim");
                                partition.relation_arena.lifecycle[slot] =
                                    RecordLifecycleState::Reusable;
                                partition.relation_arena.kind_ids[slot] = None;
                                partition.relation_arena.payloads[slot] = None;
                                partition.relation_arena.payload_history.remove(&slot);
                                partition.relation_arena.branch_pins[slot] = 0;
                                partition.relation_arena.replay_pins[slot] = 0;
                                partition.relation_arena.snapshot_pins[slot] = 0;
                                partition.relation_arena.endpoints[slot] = None;
                                partition.relation_arena.retired_at[slot] = None;
                                partition.relation_arena.free_list.push(slot as u64);
                                outcome.relation_reclaimed += 1;
                            }
                        }
                    }
                }
            }
        }

        outcome
    }
}
