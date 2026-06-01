use std::collections::BTreeMap;

use crate::storage::overlay::{
    summarize_entity_chunk_plan, EntityWorkingSetLayout, PartitionCloneMode,
    PartitionMutationJournal, PartitionState,
};

use super::StorageAuthority;

impl<'runtime> StorageAuthority<'runtime> {
    pub(crate) fn publish_partition_commits(
        &mut self,
        clone_mode: PartitionCloneMode,
        committed_partitions: BTreeMap<
            crate::identity::data::PartitionId,
            (PartitionState, PartitionMutationJournal),
        >,
    ) {
        for (partition_id, (mut partition_state, journal)) in committed_partitions {
            let entity_only_commit = !journal.entity_slots.is_empty()
                && journal.relation_slots.is_empty()
                && journal.adjacency_slots.is_empty()
                && journal.reverse_adjacency_slots.is_empty();
            let hybrid_graph_commit = matches!(clone_mode, PartitionCloneMode::GraphSparseEntities);

            if entity_only_commit {
                let entity_working_set_layout =
                    select_publish_entity_working_set_layout(journal.entity_slots.len());
                let chunk_plan = summarize_entity_chunk_plan(
                    journal.entity_slots.len(),
                    entity_working_set_layout,
                );
                if let Some(base_partition) = self.runtime.partitions.get_mut(&partition_id) {
                    match entity_working_set_layout {
                        EntityWorkingSetLayout::AoSoACandidate { chunk_width } => {
                            let published_chunks =
                                base_partition.entity_arena.merge_slot_chunks_from_owned(
                                    &mut partition_state.entity_arena,
                                    &journal.entity_slots,
                                    chunk_width,
                                    journal.entity_free_list_changed,
                                );
                            self.runtime
                                .performance_access()
                                .count_aosoa_publish_chunks(
                                    published_chunks.max(chunk_plan.chunk_count),
                                );
                        }
                        EntityWorkingSetLayout::CanonicalSoA => {
                            base_partition.entity_arena.merge_slots_from_owned(
                                &mut partition_state.entity_arena,
                                &journal.entity_slots,
                                journal.entity_free_list_changed,
                            );
                        }
                    }
                    continue;
                }
                if matches!(
                    entity_working_set_layout,
                    EntityWorkingSetLayout::AoSoACandidate { .. }
                ) {
                    self.runtime
                        .performance_access()
                        .count_aosoa_publish_soa_merge(
                            chunk_plan.chunk_count.max(1),
                            journal.entity_slots.len(),
                        );
                }
            }

            if hybrid_graph_commit {
                if let Some(mut base_partition) = self.runtime.partitions.remove(&partition_id) {
                    if !journal.entity_slots.is_empty() {
                        let entity_working_set_layout =
                            select_publish_entity_working_set_layout(journal.entity_slots.len());
                        let chunk_plan = summarize_entity_chunk_plan(
                            journal.entity_slots.len(),
                            entity_working_set_layout,
                        );
                        match entity_working_set_layout {
                            EntityWorkingSetLayout::AoSoACandidate { chunk_width } => {
                                let published_chunks =
                                    base_partition.entity_arena.merge_slot_chunks_from_owned(
                                        &mut partition_state.entity_arena,
                                        &journal.entity_slots,
                                        chunk_width,
                                        journal.entity_free_list_changed,
                                    );
                                self.runtime
                                    .performance_access()
                                    .count_aosoa_publish_chunks(
                                        published_chunks.max(chunk_plan.chunk_count),
                                    );
                            }
                            EntityWorkingSetLayout::CanonicalSoA => {
                                base_partition.entity_arena.merge_slots_from_owned(
                                    &mut partition_state.entity_arena,
                                    &journal.entity_slots,
                                    journal.entity_free_list_changed,
                                );
                            }
                        }
                    }
                    partition_state.entity_arena = base_partition.entity_arena;
                    if partition_state.relation_overlay_is_sparse
                        && !journal.relation_slots.is_empty()
                    {
                        base_partition.relation_arena.merge_slots_from_owned(
                            &mut partition_state.relation_arena,
                            &journal.relation_slots,
                            journal.relation_free_list_changed,
                        );
                        partition_state.relation_arena = base_partition.relation_arena;
                    } else if journal.relation_slots.is_empty() {
                        partition_state.relation_arena = base_partition.relation_arena;
                    }
                    if journal.adjacency_slots.is_empty() {
                        partition_state.adjacency = base_partition.adjacency;
                    }
                    if journal.reverse_adjacency_slots.is_empty() {
                        partition_state.reverse_adjacency = base_partition.reverse_adjacency;
                    }
                    self.runtime
                        .partitions
                        .insert(partition_id, partition_state);
                    continue;
                }
            }

            if let Some(base_partition) = self.runtime.partitions.remove(&partition_id) {
                if journal.entity_slots.is_empty() {
                    partition_state.entity_arena = base_partition.entity_arena;
                }
                if journal.relation_slots.is_empty() {
                    partition_state.relation_arena = base_partition.relation_arena;
                }
                if journal.adjacency_slots.is_empty() {
                    partition_state.adjacency = base_partition.adjacency;
                }
                if journal.reverse_adjacency_slots.is_empty() {
                    partition_state.reverse_adjacency = base_partition.reverse_adjacency;
                }
            }
            self.runtime
                .partitions
                .insert(partition_id, partition_state);
        }
    }
}

fn select_publish_entity_working_set_layout(touched_entity_slots: usize) -> EntityWorkingSetLayout {
    if touched_entity_slots == 0 {
        return EntityWorkingSetLayout::CanonicalSoA;
    }

    let chunk_width = if touched_entity_slots <= 128 {
        128
    } else if touched_entity_slots <= 512 {
        256
    } else {
        512
    };

    EntityWorkingSetLayout::AoSoACandidate { chunk_width }
}
