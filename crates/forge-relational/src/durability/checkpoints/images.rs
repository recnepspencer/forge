use std::marker::PhantomData;

use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::durability::data::{
    DurableBitSet, EntityCheckpointImageKind, EntityExtraImage, PartitionCheckpointImage,
    RecordArenaCheckpointImage, RecordArenaCheckpointKind, RelationCheckpointImageKind,
    RelationEndpointsImage, VersionedEntityMetadataImage, VersionedPayloadImage,
    VersionedRelationMetadataImage,
};
use crate::storage::logic::state::{
    AdjacencySet, DenseSlotBitSet, EntityExtra, EntityRecordKind, PartitionState, RecordArena,
    RecordKind, RelationEndpoints, RelationRecordKind, VersionedEntityMetadata, VersionedPayload,
    VersionedRelationMetadata,
};

trait CheckpointArenaKind: RecordKind {
    type ImageKind: RecordArenaCheckpointKind;

    fn extra_to_image(
        extra: Self::Extra,
    ) -> <Self::ImageKind as RecordArenaCheckpointKind>::ExtraImage;
    fn extra_from_image(
        extra: <Self::ImageKind as RecordArenaCheckpointKind>::ExtraImage,
    ) -> Self::Extra;
    fn meta_to_image(meta: Self::Meta)
        -> <Self::ImageKind as RecordArenaCheckpointKind>::MetaImage;
    fn meta_from_image(
        meta: <Self::ImageKind as RecordArenaCheckpointKind>::MetaImage,
    ) -> Self::Meta;
}

impl CheckpointArenaKind for EntityRecordKind {
    type ImageKind = EntityCheckpointImageKind;

    fn extra_to_image(extra: Self::Extra) -> EntityExtraImage {
        EntityExtraImage {
            structural_fingerprint: extra.structural_fingerprint,
            lineage_id: extra.lineage_id,
        }
    }

    fn extra_from_image(extra: EntityExtraImage) -> Self::Extra {
        EntityExtra {
            structural_fingerprint: extra.structural_fingerprint,
            lineage_id: extra.lineage_id,
        }
    }

    fn meta_to_image(meta: Self::Meta) -> VersionedEntityMetadataImage {
        VersionedEntityMetadataImage {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            lineage_id: meta.lineage_id,
        }
    }

    fn meta_from_image(meta: VersionedEntityMetadataImage) -> Self::Meta {
        VersionedEntityMetadata {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            lineage_id: meta.lineage_id,
        }
    }
}

impl CheckpointArenaKind for RelationRecordKind {
    type ImageKind = RelationCheckpointImageKind;

    fn extra_to_image(extra: Self::Extra) -> Option<RelationEndpointsImage> {
        extra.map(|endpoints| RelationEndpointsImage {
            source: endpoints.source,
            target: endpoints.target,
        })
    }

    fn extra_from_image(extra: Option<RelationEndpointsImage>) -> Self::Extra {
        extra.map(|endpoints| RelationEndpoints {
            source: endpoints.source,
            target: endpoints.target,
        })
    }

    fn meta_to_image(meta: Self::Meta) -> VersionedRelationMetadataImage {
        VersionedRelationMetadataImage {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            endpoints: RelationEndpointsImage {
                source: meta.endpoints.source,
                target: meta.endpoints.target,
            },
        }
    }

    fn meta_from_image(meta: VersionedRelationMetadataImage) -> Self::Meta {
        VersionedRelationMetadata {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            endpoints: RelationEndpoints {
                source: meta.endpoints.source,
                target: meta.endpoints.target,
            },
        }
    }
}

fn arena_to_image<K: CheckpointArenaKind>(
    arena: RecordArena<K>,
) -> RecordArenaCheckpointImage<K::ImageKind> {
    RecordArenaCheckpointImage {
        generations: arena.generations,
        lifecycle: arena.lifecycle,
        kind_ids: arena.kind_ids,
        payloads: arena.payloads,
        payload_history: arena
            .payload_history
            .into_iter()
            .map(|entries| {
                entries
                    .into_iter()
                    .map(|entry| VersionedPayloadImage {
                        effective_at: entry.effective_at,
                        retired_at: entry.retired_at,
                        generation: entry.generation,
                        value: entry.value,
                    })
                    .collect()
            })
            .collect(),
        metadata_history: arena
            .metadata_history
            .into_iter()
            .map(|entries| entries.into_iter().map(K::meta_to_image).collect())
            .collect(),
        created_at: arena.created_at,
        retired_at: arena.retired_at,
        aspect_versions: arena.aspect_versions,
        extra: arena.extra.into_iter().map(K::extra_to_image).collect(),
        diagnostics_enrichment: arena.diagnostics_enrichment,
        branch_pins: arena.branch_pins,
        replay_pins: arena.replay_pins,
        snapshot_pins: arena.snapshot_pins,
        live_bitset: DurableBitSet {
            words: arena.live_bitset.words().to_vec(),
        },
        reclaimable_bitset: DurableBitSet {
            words: arena.reclaimable_bitset.words().to_vec(),
        },
        free_list: arena.free_list,
        marker: PhantomData,
    }
}

fn arena_from_image<K: CheckpointArenaKind>(
    partition_id: crate::identity::data::PartitionId,
    image: RecordArenaCheckpointImage<K::ImageKind>,
) -> RecordArena<K> {
    RecordArena {
        partition_ids: vec![partition_id; image.generations.len()],
        generations: image.generations,
        lifecycle: image.lifecycle,
        kind_ids: image.kind_ids,
        payloads: image.payloads,
        payload_history: image
            .payload_history
            .into_iter()
            .map(|entries| {
                entries
                    .into_iter()
                    .map(|entry| VersionedPayload {
                        effective_at: entry.effective_at,
                        retired_at: entry.retired_at,
                        generation: entry.generation,
                        value: entry.value,
                    })
                    .collect()
            })
            .collect(),
        metadata_history: image
            .metadata_history
            .into_iter()
            .map(|entries| entries.into_iter().map(K::meta_from_image).collect())
            .collect(),
        created_at: image.created_at,
        retired_at: image.retired_at,
        extra: image.extra.into_iter().map(K::extra_from_image).collect(),
        aspect_versions: image.aspect_versions,
        diagnostics_enrichment: image.diagnostics_enrichment,
        branch_pins: image.branch_pins,
        replay_pins: image.replay_pins,
        snapshot_pins: image.snapshot_pins,
        live_bitset: DenseSlotBitSet::from_words(image.live_bitset.words),
        reclaimable_bitset: DenseSlotBitSet::from_words(image.reclaimable_bitset.words),
        free_list: image.free_list,
    }
}

pub(crate) fn partition_to_image(partition: PartitionState) -> PartitionCheckpointImage {
    PartitionCheckpointImage {
        partition_id: partition.partition_id,
        entity_arena: arena_to_image::<EntityRecordKind>(partition.entity_arena),
        relation_arena: arena_to_image::<RelationRecordKind>(partition.relation_arena),
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

pub(crate) fn partition_from_image(image: PartitionCheckpointImage) -> PartitionState {
    PartitionState {
        partition_id: image.partition_id,
        adjacency_policy: AdjacencyPolicy {
            backend: AdjacencyBackend::CompressedFanoutAdjacency,
            small_degree_inline_capacity: 4,
        },
        relation_overlay_is_sparse: false,
        entity_arena: arena_from_image::<EntityRecordKind>(image.partition_id, image.entity_arena),
        relation_arena: arena_from_image::<RelationRecordKind>(
            image.partition_id,
            image.relation_arena,
        ),
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
