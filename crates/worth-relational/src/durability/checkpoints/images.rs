use std::marker::PhantomData;

use crate::config::data::{AdjacencyBackend, AdjacencyPolicy};
use crate::durability::data::{
    DurabilityError, DurableBitSet, EntityCheckpointImageKind, EntityExtraImage,
    PartitionCheckpointImage, RecordArenaCheckpointImage, RecordArenaCheckpointKind,
    RecoveryFailureClass, RelationCheckpointImageKind, RelationEndpointsImage, RelationExtraImage,
    VersionedEntityMetadataImage, VersionedRelationMetadataImage,
};
use crate::identity::data::KindId;
use crate::schema::data::{AspectContractPlanCatalog, LoweredAspectContractPlan};
use crate::storage::overlay::PartitionState;
use crate::storage::partition::{AdjacencySet, DenseSlotBitSet};
use crate::storage::substrate::{
    EntityExtra, EntityRecordKind, RecordArena, RecordKind, RelationEndpoints, RelationRecordKind,
    VersionedEntityMetadata, VersionedRelationMetadata,
};

use super::aspect_state_images::{export_state, readmit_state, CheckpointAspectContractCatalog};

trait CheckpointArenaKind: RecordKind {
    type ImageKind: RecordArenaCheckpointKind;

    fn plan(
        catalog: &AspectContractPlanCatalog,
        kind_id: KindId,
    ) -> Option<&LoweredAspectContractPlan>;
    fn meta_kind(meta: &Self::Meta) -> KindId;
    fn meta_kind_from_image(
        meta: &<Self::ImageKind as RecordArenaCheckpointKind>::MetaImage,
    ) -> KindId;
    fn extra_to_image(
        extra: Self::Extra,
        plan: Option<&LoweredAspectContractPlan>,
    ) -> Result<<Self::ImageKind as RecordArenaCheckpointKind>::ExtraImage, DurabilityError>;
    fn extra_from_image(
        extra: <Self::ImageKind as RecordArenaCheckpointKind>::ExtraImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Extra, DurabilityError>;
    fn meta_to_image(
        meta: Self::Meta,
        plan: &LoweredAspectContractPlan,
    ) -> Result<<Self::ImageKind as RecordArenaCheckpointKind>::MetaImage, DurabilityError>;
    fn meta_from_image(
        meta: <Self::ImageKind as RecordArenaCheckpointKind>::MetaImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Meta, DurabilityError>;
}

fn missing_plan(kind_id: KindId) -> DurabilityError {
    DurabilityError::new(
        RecoveryFailureClass::SchemaMismatch,
        format!(
            "checkpoint record kind {} has no lowered aspect plan",
            kind_id.0
        ),
    )
}

impl CheckpointArenaKind for EntityRecordKind {
    type ImageKind = EntityCheckpointImageKind;

    fn plan(
        catalog: &AspectContractPlanCatalog,
        kind_id: KindId,
    ) -> Option<&LoweredAspectContractPlan> {
        catalog.entity_plans.get(&kind_id)
    }

    fn meta_kind(meta: &Self::Meta) -> KindId {
        meta.kind_id
    }

    fn meta_kind_from_image(meta: &VersionedEntityMetadataImage) -> KindId {
        meta.kind_id
    }

    fn extra_to_image(
        extra: Self::Extra,
        plan: Option<&LoweredAspectContractPlan>,
    ) -> Result<EntityExtraImage, DurabilityError> {
        Ok(EntityExtraImage {
            structural_fingerprint: extra.structural_fingerprint,
            lineage_id: extra.lineage_id,
            authoritative_aspect_state: export_state(extra.authoritative_aspect_state, plan)?,
        })
    }

    fn extra_from_image(
        extra: EntityExtraImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Extra, DurabilityError> {
        Ok(EntityExtra {
            structural_fingerprint: extra.structural_fingerprint,
            lineage_id: extra.lineage_id,
            authoritative_aspect_state: readmit_state(extra.authoritative_aspect_state, contracts)?,
        })
    }

    fn meta_to_image(
        meta: Self::Meta,
        plan: &LoweredAspectContractPlan,
    ) -> Result<VersionedEntityMetadataImage, DurabilityError> {
        Ok(VersionedEntityMetadataImage {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            lineage_id: meta.lineage_id,
            authoritative_aspect_state: export_state(meta.authoritative_aspect_state, Some(plan))?,
        })
    }

    fn meta_from_image(
        meta: VersionedEntityMetadataImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Meta, DurabilityError> {
        Ok(VersionedEntityMetadata {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            lineage_id: meta.lineage_id,
            authoritative_aspect_state: readmit_state(meta.authoritative_aspect_state, contracts)?,
        })
    }
}

impl CheckpointArenaKind for RelationRecordKind {
    type ImageKind = RelationCheckpointImageKind;

    fn plan(
        catalog: &AspectContractPlanCatalog,
        kind_id: KindId,
    ) -> Option<&LoweredAspectContractPlan> {
        catalog.relation_plans.get(&kind_id)
    }

    fn meta_kind(meta: &Self::Meta) -> KindId {
        meta.kind_id
    }

    fn meta_kind_from_image(meta: &VersionedRelationMetadataImage) -> KindId {
        meta.kind_id
    }

    fn extra_to_image(
        extra: Self::Extra,
        plan: Option<&LoweredAspectContractPlan>,
    ) -> Result<RelationExtraImage, DurabilityError> {
        Ok(RelationExtraImage {
            endpoints: extra.endpoints.map(|endpoints| RelationEndpointsImage {
                source: endpoints.source,
                target: endpoints.target,
            }),
            authoritative_aspect_state: export_state(extra.authoritative_aspect_state, plan)?,
        })
    }

    fn extra_from_image(
        extra: RelationExtraImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Extra, DurabilityError> {
        Ok(crate::storage::substrate::RelationExtra {
            endpoints: extra.endpoints.map(|endpoints| RelationEndpoints {
                source: endpoints.source,
                target: endpoints.target,
            }),
            authoritative_aspect_state: readmit_state(extra.authoritative_aspect_state, contracts)?,
        })
    }

    fn meta_to_image(
        meta: Self::Meta,
        plan: &LoweredAspectContractPlan,
    ) -> Result<VersionedRelationMetadataImage, DurabilityError> {
        Ok(VersionedRelationMetadataImage {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            endpoints: RelationEndpointsImage {
                source: meta.endpoints.source,
                target: meta.endpoints.target,
            },
            authoritative_aspect_state: export_state(meta.authoritative_aspect_state, Some(plan))?,
        })
    }

    fn meta_from_image(
        meta: VersionedRelationMetadataImage,
        contracts: &CheckpointAspectContractCatalog,
    ) -> Result<Self::Meta, DurabilityError> {
        Ok(VersionedRelationMetadata {
            effective_at: meta.effective_at,
            retired_at: meta.retired_at,
            generation: meta.generation,
            kind_id: meta.kind_id,
            endpoints: RelationEndpoints {
                source: meta.endpoints.source,
                target: meta.endpoints.target,
            },
            authoritative_aspect_state: readmit_state(meta.authoritative_aspect_state, contracts)?,
        })
    }
}

fn arena_to_image<K: CheckpointArenaKind>(
    arena: RecordArena<K>,
    catalog: &AspectContractPlanCatalog,
) -> Result<RecordArenaCheckpointImage<K::ImageKind>, DurabilityError> {
    let metadata_history = arena
        .metadata_history
        .into_iter()
        .map(|entries| {
            entries
                .into_iter()
                .map(|meta| {
                    let kind_id = K::meta_kind(&meta);
                    let plan = K::plan(catalog, kind_id).ok_or_else(|| missing_plan(kind_id))?;
                    K::meta_to_image(meta, plan)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let extra = arena
        .extra
        .into_iter()
        .zip(arena.kind_ids.iter().copied())
        .map(|(extra, kind_id)| {
            K::extra_to_image(extra, kind_id.and_then(|kind_id| K::plan(catalog, kind_id)))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RecordArenaCheckpointImage {
        generations: arena.generations,
        lifecycle: arena.lifecycle,
        kind_ids: arena.kind_ids,
        metadata_history,
        created_at: arena.created_at,
        retired_at: arena.retired_at,
        aspect_versions: arena.aspect_versions,
        extra,
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
    })
}

fn arena_from_image<K: CheckpointArenaKind>(
    partition_id: crate::identity::data::PartitionId,
    image: RecordArenaCheckpointImage<K::ImageKind>,
    catalog: &AspectContractPlanCatalog,
    contracts: &CheckpointAspectContractCatalog,
) -> Result<RecordArena<K>, DurabilityError> {
    let metadata_history = image
        .metadata_history
        .into_iter()
        .map(|entries| {
            entries
                .into_iter()
                .map(|meta| {
                    let kind_id = K::meta_kind_from_image(&meta);
                    K::plan(catalog, kind_id).ok_or_else(|| missing_plan(kind_id))?;
                    K::meta_from_image(meta, contracts)
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let extra = image
        .extra
        .into_iter()
        .zip(image.kind_ids.iter().copied())
        .map(|(extra, kind_id)| {
            if let Some(kind_id) = kind_id {
                K::plan(catalog, kind_id).ok_or_else(|| missing_plan(kind_id))?;
            }
            K::extra_from_image(extra, contracts)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RecordArena {
        partition_ids: vec![partition_id; image.generations.len()],
        generations: image.generations,
        lifecycle: image.lifecycle,
        kind_ids: image.kind_ids,
        metadata_history,
        created_at: image.created_at,
        retired_at: image.retired_at,
        extra,
        aspect_versions: image.aspect_versions,
        diagnostics_enrichment: image.diagnostics_enrichment,
        branch_pins: image.branch_pins,
        replay_pins: image.replay_pins,
        snapshot_pins: image.snapshot_pins,
        live_bitset: DenseSlotBitSet::from_words(image.live_bitset.words),
        reclaimable_bitset: DenseSlotBitSet::from_words(image.reclaimable_bitset.words),
        free_list: image.free_list,
    })
}

pub(crate) fn partition_to_image(
    partition: PartitionState,
    catalog: &AspectContractPlanCatalog,
) -> Result<PartitionCheckpointImage, DurabilityError> {
    Ok(PartitionCheckpointImage {
        partition_id: partition.partition_id,
        entity_arena: arena_to_image::<EntityRecordKind>(partition.entity_arena, catalog)?,
        relation_arena: arena_to_image::<RelationRecordKind>(partition.relation_arena, catalog)?,
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
    })
}

pub(crate) fn partition_from_image(
    image: PartitionCheckpointImage,
    catalog: &AspectContractPlanCatalog,
    contracts: &CheckpointAspectContractCatalog,
) -> Result<PartitionState, DurabilityError> {
    Ok(PartitionState {
        partition_id: image.partition_id,
        adjacency_policy: AdjacencyPolicy {
            backend: AdjacencyBackend::CompressedFanoutAdjacency,
            small_degree_inline_capacity: 4,
        },
        relation_overlay_is_sparse: false,
        entity_arena: arena_from_image::<EntityRecordKind>(
            image.partition_id,
            image.entity_arena,
            catalog,
            contracts,
        )?,
        relation_arena: arena_from_image::<RelationRecordKind>(
            image.partition_id,
            image.relation_arena,
            catalog,
            contracts,
        )?,
        adjacency: image
            .adjacency
            .into_iter()
            .map(AdjacencySet::compressed_from_current)
            .collect(),
        reverse_adjacency: image
            .reverse_adjacency
            .into_iter()
            .map(AdjacencySet::compressed_from_current)
            .collect(),
    })
}
