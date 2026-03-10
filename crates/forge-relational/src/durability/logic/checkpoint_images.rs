use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::durability::data::{
    DurableBitSet, EntityArenaCheckpointImage, PartitionCheckpointImage,
    RelationArenaCheckpointImage, RelationEndpointsImage, VersionedPayloadImage,
};
use crate::storage::logic::state::{
    AdjacencySet, DenseSlotBitSet, EntityArena, PartitionState, RelationArena, RelationEndpoints,
    VersionedPayload,
};

pub(super) fn partition_to_image(partition: PartitionState) -> PartitionCheckpointImage {
    PartitionCheckpointImage {
        partition_id: partition.partition_id,
        entity_arena: EntityArenaCheckpointImage {
            generations: partition.entity_arena.generations,
            lifecycle: partition.entity_arena.lifecycle,
            kind_ids: partition.entity_arena.kind_ids,
            payloads: partition.entity_arena.payloads,
            payload_history: partition
                .entity_arena
                .payload_history
                .into_iter()
                .map(|entries| {
                    entries
                        .into_iter()
                        .map(|entry| VersionedPayloadImage {
                            effective_at: entry.effective_at,
                            retired_at: entry.retired_at,
                            value: entry.value,
                        })
                        .collect()
                })
                .collect(),
            created_at: partition.entity_arena.created_at,
            retired_at: partition.entity_arena.retired_at,
            aspect_versions: partition.entity_arena.aspect_versions,
            structural_fingerprints: partition.entity_arena.structural_fingerprints,
            lineage_ids: partition.entity_arena.lineage_ids,
            diagnostics_enrichment: partition.entity_arena.diagnostics_enrichment,
            branch_pins: partition.entity_arena.branch_pins,
            replay_pins: partition.entity_arena.replay_pins,
            snapshot_pins: partition.entity_arena.snapshot_pins,
            live_bitset: DurableBitSet {
                words: partition.entity_arena.live_bitset.words().to_vec(),
            },
            reclaimable_bitset: DurableBitSet {
                words: partition.entity_arena.reclaimable_bitset.words().to_vec(),
            },
            free_list: partition.entity_arena.free_list,
        },
        relation_arena: RelationArenaCheckpointImage {
            generations: partition.relation_arena.generations,
            lifecycle: partition.relation_arena.lifecycle,
            kind_ids: partition.relation_arena.kind_ids,
            payloads: partition.relation_arena.payloads,
            payload_history: partition
                .relation_arena
                .payload_history
                .into_iter()
                .map(|(slot, entries)| {
                    (
                        slot,
                        entries
                            .into_iter()
                            .map(|entry| VersionedPayloadImage {
                                effective_at: entry.effective_at,
                                retired_at: entry.retired_at,
                                value: entry.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
            created_at: partition.relation_arena.created_at,
            retired_at: partition.relation_arena.retired_at,
            endpoints: partition
                .relation_arena
                .endpoints
                .into_iter()
                .map(|endpoints| {
                    endpoints.map(|endpoints| RelationEndpointsImage {
                        source: endpoints.source,
                        target: endpoints.target,
                    })
                })
                .collect(),
            diagnostics_enrichment: partition.relation_arena.diagnostics_enrichment,
            snapshot_pins: partition.relation_arena.snapshot_pins,
            live_bitset: DurableBitSet {
                words: partition.relation_arena.live_bitset.words().to_vec(),
            },
            reclaimable_bitset: DurableBitSet {
                words: partition.relation_arena.reclaimable_bitset.words().to_vec(),
            },
            free_list: partition.relation_arena.free_list,
        },
        adjacency: partition
            .adjacency
            .into_iter()
            .map(|adjacency| adjacency.ids())
            .collect(),
        reverse_adjacency: partition
            .reverse_adjacency
            .into_iter()
            .map(|adjacency| adjacency.ids())
            .collect(),
    }
}

pub(super) fn partition_from_image(image: PartitionCheckpointImage) -> PartitionState {
    PartitionState {
        partition_id: image.partition_id,
        adjacency_policy: AdjacencyPolicy {
            backend: AdjacencyBackend::CompressedFanoutAdjacency,
            small_degree_inline_capacity: 4,
        },
        entity_arena: EntityArena {
            partition_ids: vec![image.partition_id; image.entity_arena.generations.len()],
            generations: image.entity_arena.generations,
            lifecycle: image.entity_arena.lifecycle,
            kind_ids: image.entity_arena.kind_ids,
            payloads: image.entity_arena.payloads,
            payload_history: image
                .entity_arena
                .payload_history
                .into_iter()
                .map(|entries| {
                    entries
                        .into_iter()
                        .map(|entry| VersionedPayload {
                            effective_at: entry.effective_at,
                            retired_at: entry.retired_at,
                            value: entry.value,
                        })
                        .collect()
                })
                .collect(),
            created_at: image.entity_arena.created_at,
            retired_at: image.entity_arena.retired_at,
            aspect_versions: image.entity_arena.aspect_versions,
            structural_fingerprints: image.entity_arena.structural_fingerprints,
            lineage_ids: image.entity_arena.lineage_ids,
            diagnostics_enrichment: image.entity_arena.diagnostics_enrichment,
            branch_pins: image.entity_arena.branch_pins,
            replay_pins: image.entity_arena.replay_pins,
            snapshot_pins: image.entity_arena.snapshot_pins,
            live_bitset: DenseSlotBitSet::from_words(image.entity_arena.live_bitset.words),
            reclaimable_bitset: DenseSlotBitSet::from_words(
                image.entity_arena.reclaimable_bitset.words,
            ),
            free_list: image.entity_arena.free_list,
        },
        relation_arena: RelationArena {
            partition_ids: vec![image.partition_id; image.relation_arena.generations.len()],
            generations: image.relation_arena.generations,
            lifecycle: image.relation_arena.lifecycle,
            kind_ids: image.relation_arena.kind_ids,
            payloads: image.relation_arena.payloads,
            payload_history: image
                .relation_arena
                .payload_history
                .into_iter()
                .map(|(slot, entries)| {
                    (
                        slot,
                        entries
                            .into_iter()
                            .map(|entry| VersionedPayload {
                                effective_at: entry.effective_at,
                                retired_at: entry.retired_at,
                                value: entry.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
            created_at: image.relation_arena.created_at,
            retired_at: image.relation_arena.retired_at,
            endpoints: image
                .relation_arena
                .endpoints
                .into_iter()
                .map(|endpoints| {
                    endpoints.map(|endpoints| RelationEndpoints {
                        source: endpoints.source,
                        target: endpoints.target,
                    })
                })
                .collect(),
            diagnostics_enrichment: image.relation_arena.diagnostics_enrichment,
            snapshot_pins: image.relation_arena.snapshot_pins,
            live_bitset: DenseSlotBitSet::from_words(image.relation_arena.live_bitset.words),
            reclaimable_bitset: DenseSlotBitSet::from_words(
                image.relation_arena.reclaimable_bitset.words,
            ),
            free_list: image.relation_arena.free_list,
        },
        adjacency: image
            .adjacency
            .into_iter()
            .map(AdjacencySet::Compressed)
            .collect(),
        reverse_adjacency: image
            .reverse_adjacency
            .into_iter()
            .map(AdjacencySet::Compressed)
            .collect(),
    }
}
