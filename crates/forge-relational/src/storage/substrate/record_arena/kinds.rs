use std::fmt::Debug;

use crate::identity::data::{
    EntityDomain, KindId, PartitionId, RecordId, RelationDomain, VersionId,
};
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::storage::overlay::PartitionState;

use super::{
    EntityExtra, RecordArena, RelationExtra, VersionedEntityMetadata, VersionedRelationMetadata,
};

pub(crate) trait RecordKind: Clone + Debug + 'static {
    type Domain: Copy + Ord + Debug + 'static;
    type Meta: Clone + Debug;
    type Extra: Clone + Debug;
    type ReadRecord: Clone + Debug;

    fn arena(partition: &PartitionState) -> &RecordArena<Self>;
    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self>;
    fn empty_extra() -> Self::Extra;
    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize);
    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId);
    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        extra: &Self::Extra,
    ) -> Self::Meta;
}

pub(crate) trait HistoricalMetadata {
    fn effective_at(&self) -> VersionId;
    fn retired_at(&self) -> Option<VersionId>;
}

#[derive(Debug, Clone)]
pub(crate) struct EntityRecordKind;

impl RecordKind for EntityRecordKind {
    type Domain = EntityDomain;
    type Meta = VersionedEntityMetadata;
    type Extra = EntityExtra;
    type ReadRecord = EntityReadRecord;

    fn arena(partition: &PartitionState) -> &RecordArena<Self> {
        &partition.entity_arena
    }

    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self> {
        &mut partition.entity_arena
    }

    fn empty_extra() -> Self::Extra {
        EntityExtra::default()
    }

    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize) {
        extra.reserve(additional);
    }

    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId) {
        metadata.retired_at = Some(version_id);
    }

    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        _extra: &Self::Extra,
    ) -> Self::Meta {
        VersionedEntityMetadata {
            effective_at: version_id,
            retired_at: None,
            generation,
            kind_id,
        }
    }
}

impl HistoricalMetadata for VersionedEntityMetadata {
    fn effective_at(&self) -> VersionId {
        self.effective_at
    }

    fn retired_at(&self) -> Option<VersionId> {
        self.retired_at
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RelationRecordKind;

impl RecordKind for RelationRecordKind {
    type Domain = RelationDomain;
    type Meta = VersionedRelationMetadata;
    type Extra = RelationExtra;
    type ReadRecord = RelationReadRecord;

    fn arena(partition: &PartitionState) -> &RecordArena<Self> {
        &partition.relation_arena
    }

    fn arena_mut(partition: &mut PartitionState) -> &mut RecordArena<Self> {
        &mut partition.relation_arena
    }

    fn empty_extra() -> Self::Extra {
        None
    }

    fn reserve_extra(extra: &mut Vec<Self::Extra>, additional: usize) {
        extra.reserve(additional);
    }

    fn retire_metadata(metadata: &mut Self::Meta, version_id: VersionId) {
        metadata.retired_at = Some(version_id);
    }

    fn metadata_for_create(
        kind_id: KindId,
        generation: u32,
        version_id: VersionId,
        extra: &Self::Extra,
    ) -> Self::Meta {
        VersionedRelationMetadata {
            effective_at: version_id,
            retired_at: None,
            generation,
            kind_id,
            endpoints: extra.clone().expect("relation metadata requires endpoints"),
        }
    }
}

impl HistoricalMetadata for VersionedRelationMetadata {
    fn effective_at(&self) -> VersionId {
        self.effective_at
    }

    fn retired_at(&self) -> Option<VersionId> {
        self.retired_at
    }
}

pub(crate) fn partition_of<K: RecordKind>(id: &RecordId<K::Domain>) -> PartitionId {
    id.partition_id
}

pub(crate) fn slot_of<K: RecordKind>(id: &RecordId<K::Domain>) -> usize {
    id.local_slot.0 as usize
}

pub(crate) fn generation_of<K: RecordKind>(id: &RecordId<K::Domain>) -> u32 {
    id.generation.0
}
