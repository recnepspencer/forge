//! Effect-only execution of a complete partition-publication plan.

use crate::storage::overlay::{summarize_entity_chunk_plan, EntityWorkingSetLayout};

use super::super::StorageAuthority;
use super::plan::{
    PartitionPublicationPlan, PartitionPublicationStrategy, PlannedPartitionPublication,
};

pub(super) fn execute_partition_publication(
    authority: &mut StorageAuthority<'_>,
    plan: PartitionPublicationPlan,
) {
    for partition in plan.partitions {
        execute_partition(authority, partition);
    }
}

fn execute_partition(
    authority: &mut StorageAuthority<'_>,
    mut publication: PlannedPartitionPublication,
) {
    match publication.strategy {
        PartitionPublicationStrategy::MissingPartition { entity_layout } => {
            if let Some(layout) = entity_layout {
                record_entity_replacement(authority.runtime, &publication, layout);
            }
            publication.partition_state.clear_runtime_pin_counters();
            authority
                .runtime
                .partitions
                .insert(publication.partition_id, publication.partition_state);
        }
        PartitionPublicationStrategy::ExistingEntityOnly(layout) => {
            let base = authority
                .runtime
                .partitions
                .get_mut(&publication.partition_id)
                .expect("planned existing entity partition must remain present");
            let published_chunks = merge_entity_slots(base, &mut publication, layout);
            count_published_chunks(authority.runtime, published_chunks);
        }
        PartitionPublicationStrategy::ExistingHybridGraph { entity_layout } => {
            let base = authority
                .runtime
                .partitions
                .remove(&publication.partition_id)
                .expect("planned existing hybrid partition must remain present");
            publish_hybrid_partition(authority.runtime, publication, base, entity_layout);
        }
        PartitionPublicationStrategy::ExistingWholePartition => {
            let base = authority
                .runtime
                .partitions
                .remove(&publication.partition_id)
                .expect("planned existing whole partition must remain present");
            publish_whole_partition(authority, publication, base);
        }
    }
}

fn merge_entity_slots(
    base: &mut crate::storage::overlay::PartitionState,
    publication: &mut PlannedPartitionPublication,
    layout: EntityWorkingSetLayout,
) -> Option<usize> {
    let chunk_plan = summarize_entity_chunk_plan(publication.journal.entity_slots.len(), layout);
    match layout {
        EntityWorkingSetLayout::AoSoACandidate { chunk_width } => {
            let published = base.entity_arena.merge_slot_chunks_from_owned(
                &mut publication.partition_state.entity_arena,
                &publication.journal.entity_slots,
                chunk_width,
            );
            Some(published.max(chunk_plan.chunk_count))
        }
        EntityWorkingSetLayout::CanonicalSoA => {
            base.entity_arena.merge_slots_from_owned(
                &mut publication.partition_state.entity_arena,
                &publication.journal.entity_slots,
            );
            None
        }
    }
}

fn count_published_chunks(
    runtime: &crate::runtime::RelationalRuntime,
    published_chunks: Option<usize>,
) {
    if let Some(published_chunks) = published_chunks {
        runtime
            .performance_access()
            .count_aosoa_publish_chunks(published_chunks);
    }
}

fn record_entity_replacement(
    runtime: &crate::runtime::RelationalRuntime,
    publication: &PlannedPartitionPublication,
    layout: EntityWorkingSetLayout,
) {
    if matches!(layout, EntityWorkingSetLayout::AoSoACandidate { .. }) {
        let summary = summarize_entity_chunk_plan(publication.journal.entity_slots.len(), layout);
        runtime.performance_access().count_aosoa_publish_soa_merge(
            summary.chunk_count.max(1),
            publication.journal.entity_slots.len(),
        );
    }
}

fn publish_hybrid_partition(
    runtime: &mut crate::runtime::RelationalRuntime,
    mut publication: PlannedPartitionPublication,
    mut base: crate::storage::overlay::PartitionState,
    entity_layout: Option<EntityWorkingSetLayout>,
) {
    if let Some(layout) = entity_layout {
        let published_chunks = merge_entity_slots(&mut base, &mut publication, layout);
        count_published_chunks(runtime, published_chunks);
    }
    publication.partition_state.entity_arena = base.entity_arena;
    if publication.partition_state.relation_overlay_is_sparse
        && !publication.journal.relation_slots.is_empty()
    {
        base.relation_arena.merge_slots_from_owned(
            &mut publication.partition_state.relation_arena,
            &publication.journal.relation_slots,
        );
        publication.partition_state.relation_arena = base.relation_arena;
    } else if publication.journal.relation_slots.is_empty() {
        publication.partition_state.relation_arena = base.relation_arena;
    } else {
        publication
            .partition_state
            .relation_arena
            .preserve_runtime_pins_from(&base.relation_arena);
    }
    if publication.journal.adjacency_slots.is_empty() {
        publication.partition_state.adjacency = base.adjacency;
    }
    if publication.journal.reverse_adjacency_slots.is_empty() {
        publication.partition_state.reverse_adjacency = base.reverse_adjacency;
    }
    runtime
        .partitions
        .insert(publication.partition_id, publication.partition_state);
}

fn publish_whole_partition(
    authority: &mut StorageAuthority<'_>,
    mut publication: PlannedPartitionPublication,
    base: crate::storage::overlay::PartitionState,
) {
    if publication.journal.entity_slots.is_empty() {
        publication.partition_state.entity_arena = base.entity_arena;
    } else {
        publication
            .partition_state
            .entity_arena
            .preserve_runtime_pins_from(&base.entity_arena);
    }
    if publication.journal.relation_slots.is_empty() {
        publication.partition_state.relation_arena = base.relation_arena;
    } else {
        publication
            .partition_state
            .relation_arena
            .preserve_runtime_pins_from(&base.relation_arena);
    }
    if publication.journal.adjacency_slots.is_empty() {
        publication.partition_state.adjacency = base.adjacency;
    }
    if publication.journal.reverse_adjacency_slots.is_empty() {
        publication.partition_state.reverse_adjacency = base.reverse_adjacency;
    }
    authority
        .runtime
        .partitions
        .insert(publication.partition_id, publication.partition_state);
}
