use crate::capabilities::SnapshotSource;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};
use crate::storage::logic::state::PartitionAccess;

use super::reader::VisibilityReadContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionAspect(&'static str);

impl ProjectionAspect {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn name(self) -> &'static str {
        self.0
    }
}

pub trait EntityRecordProjection: Sized {
    const KIND: KindId;

    fn required_aspects() -> &'static [ProjectionAspect] {
        &[]
    }

    fn from_record(record: &EntityReadRecord) -> Option<Self>;
}

pub trait RelationRecordProjection: Sized {
    const KIND: KindId;

    fn required_aspects() -> &'static [ProjectionAspect] {
        &[]
    }

    fn from_record(record: &RelationReadRecord) -> Option<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct VisibilityProjectionView<'runtime> {
    runtime: &'runtime RelationalRuntime,
    version_id: VersionId,
}

impl<'runtime> VisibilityProjectionView<'runtime> {
    pub(crate) const fn new(runtime: &'runtime RelationalRuntime, version_id: VersionId) -> Self {
        Self {
            runtime,
            version_id,
        }
    }

    pub const fn version_id(&self) -> VersionId {
        self.version_id
    }

    pub fn entities<T: EntityRecordProjection>(&self) -> Vec<T> {
        self.entity_records(T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(&record))
            .collect()
    }

    pub fn entities_in<T: EntityRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        self.entity_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(&record))
            .collect()
    }

    pub fn entity<T: EntityRecordProjection>(&self, entity_id: EntityId) -> Option<T> {
        let reader = self.reader();
        let state = self.runtime.storage_access().current_state();
        reader
            .entity_record_for_id_at_version(&state, entity_id, self.version_id)
            .and_then(|record| T::from_record(&record))
    }

    pub fn relations<T: RelationRecordProjection>(&self) -> Vec<T> {
        self.relation_records(T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(&record))
            .collect()
    }

    pub fn relations_in<T: RelationRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        self.relation_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(&record))
            .collect()
    }

    pub fn relation<T: RelationRecordProjection>(&self, relation_id: RelationId) -> Option<T> {
        let reader = self.reader();
        let state = self.runtime.storage_access().current_state();
        reader
            .relation_record_for_id_at_version(&state, relation_id, self.version_id)
            .and_then(|record| T::from_record(&record))
    }

    pub fn entity_records(&self, kind_id: KindId) -> Vec<EntityReadRecord> {
        let reader = self.reader();
        reader.visible_entities_of_kind(kind_id, self.version_id)
    }

    pub fn all_entity_records(&self) -> Vec<EntityReadRecord> {
        let reader = self.reader();
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let Some(slots) = reader.visible_entity_slots_in_partition_from_state(
                &state,
                partition_id,
                self.version_id,
            ) else {
                continue;
            };
            for slot in slots.iter_set_slots() {
                let entity_id = EntityId::new(partition_id, slot as u64, 0);
                if let Some(record) =
                    reader.entity_record_for_id_at_version(&state, entity_id, self.version_id)
                {
                    records.push(record);
                }
            }
        }
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn entity_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Vec<EntityReadRecord> {
        let reader = self.reader();
        reader.visible_entities_of_kind_in_partition(partition_id, kind_id, self.version_id)
    }

    pub fn relation_records(&self, kind_id: KindId) -> Vec<RelationReadRecord> {
        let reader = self.reader();
        reader.visible_relations_of_kind(kind_id, self.version_id)
    }

    pub fn all_relation_records(&self) -> Vec<RelationReadRecord> {
        let reader = self.reader();
        let state = self.runtime.storage_access().current_state();
        let mut records = Vec::new();
        for partition_id in state.partition_ids() {
            let Some(slots) = reader.visible_relation_slots_in_partition_from_state(
                &state,
                partition_id,
                self.version_id,
            ) else {
                continue;
            };
            for slot in slots.iter_set_slots() {
                let relation_id = RelationId::new(partition_id, slot as u64, 0);
                if let Some(record) =
                    reader.relation_record_for_id_at_version(&state, relation_id, self.version_id)
                {
                    records.push(record);
                }
            }
        }
        debug_assert!(relation_records_are_canonical(&records));
        records
    }

    pub fn relation_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Vec<RelationReadRecord> {
        let reader = self.reader();
        reader.visible_relations_of_kind_in_partition(partition_id, kind_id, self.version_id)
    }

    fn reader(&self) -> VisibilityReadContext<'runtime> {
        VisibilityReadContext::new(self.runtime)
    }
}

fn entity_records_are_canonical(records: &[EntityReadRecord]) -> bool {
    records
        .windows(2)
        .all(|window| window[0].entity_id <= window[1].entity_id)
}

fn relation_records_are_canonical(records: &[RelationReadRecord]) -> bool {
    records
        .windows(2)
        .all(|window| window[0].relation_id <= window[1].relation_id)
}

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn project_version(&self, version_id: VersionId) -> VisibilityProjectionView<'runtime> {
        VisibilityProjectionView::new(self.runtime(), version_id)
    }

    pub fn project_snapshot(
        &self,
        handle: &SnapshotHandle,
    ) -> Option<VisibilityProjectionView<'runtime>> {
        let version_id = if let Some((version_id, _read_policy)) =
            self.runtime().active_snapshot_binding(handle.snapshot_id)
        {
            version_id
        } else {
            self.runtime()
                .published_snapshot_version(handle.snapshot_id)?
        };
        Some(VisibilityProjectionView::new(self.runtime(), version_id))
    }
}
