use crate::capabilities::AspectPlanSource;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::publication::patch::data::AspectKey;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};
use forge_foundational::facade::{
    AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
    StructAspectValue,
};

use super::contracts::assert_declared_projection_aspects;
use super::relation_canonicalization::relation_records_are_canonical;
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::super::reader::VisibilityReadContext;

pub trait EntityRecordProjection: Sized {
    const KIND: KindId;

    fn required_aspects() -> &'static [AspectKey] {
        &[]
    }

    fn from_record(record: EntityProjectionRecord<'_>) -> Option<Self>;
}

pub trait RelationRecordProjection: Sized {
    const KIND: KindId;

    fn required_aspects() -> &'static [AspectKey] {
        &[]
    }

    fn from_record(record: RelationProjectionRecord<'_>) -> Option<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct EntityProjectionRecord<'a> {
    record: &'a EntityReadRecord,
}

impl<'a> EntityProjectionRecord<'a> {
    const fn new(record: &'a EntityReadRecord) -> Self {
        Self { record }
    }

    pub const fn entity_id(self) -> EntityId {
        self.record.entity_id
    }

    pub const fn lifecycle(self) -> RecordLifecycleState {
        self.record.lifecycle
    }

    pub fn authoritative_aspect_state(self) -> Option<&'a AuthoritativeRecordAspectState> {
        self.record.authoritative_aspect_state.as_ref()
    }

    pub fn aspect_value(self, aspect_key: &AspectKey) -> Option<&'a AspectValue> {
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(value) => Some(value),
            ContractValidatedAspectValueView::Struct(_) => None,
        }
    }

    pub fn struct_aspect_value(self, aspect_key: &AspectKey) -> Option<&'a StructAspectValue> {
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => Some(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RelationProjectionRecord<'a> {
    record: &'a RelationReadRecord,
}

impl<'a> RelationProjectionRecord<'a> {
    const fn new(record: &'a RelationReadRecord) -> Self {
        Self { record }
    }

    pub const fn relation_id(self) -> RelationId {
        self.record.relation_id
    }

    pub const fn source(self) -> EntityId {
        self.record.source
    }

    pub const fn target(self) -> EntityId {
        self.record.target
    }

    pub const fn lifecycle(self) -> RecordLifecycleState {
        self.record.lifecycle
    }

    pub fn authoritative_aspect_state(self) -> Option<&'a AuthoritativeRecordAspectState> {
        self.record.authoritative_aspect_state.as_ref()
    }

    pub fn aspect_value(self, aspect_key: &AspectKey) -> Option<&'a AspectValue> {
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(value) => Some(value),
            ContractValidatedAspectValueView::Struct(_) => None,
        }
    }

    pub fn struct_aspect_value(self, aspect_key: &AspectKey) -> Option<&'a StructAspectValue> {
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => Some(value),
        }
    }
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
        self.assert_entity_projection_contract::<T>();
        self.entity_records(T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(EntityProjectionRecord::new(&record)))
            .collect()
    }

    pub fn entities_in<T: EntityRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        self.assert_entity_projection_contract::<T>();
        self.entity_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(EntityProjectionRecord::new(&record)))
            .collect()
    }

    pub fn entity<T: EntityRecordProjection>(&self, entity_id: EntityId) -> Option<T> {
        self.assert_entity_projection_contract::<T>();
        self.entity_record(entity_id)
            .and_then(|record| T::from_record(EntityProjectionRecord::new(&record)))
    }

    pub fn relations<T: RelationRecordProjection>(&self) -> Vec<T> {
        self.assert_relation_projection_contract::<T>();
        self.relation_records(T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(RelationProjectionRecord::new(&record)))
            .collect()
    }

    pub fn relations_in<T: RelationRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        self.assert_relation_projection_contract::<T>();
        self.relation_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| T::from_record(RelationProjectionRecord::new(&record)))
            .collect()
    }

    pub fn relation<T: RelationRecordProjection>(&self, relation_id: RelationId) -> Option<T> {
        self.assert_relation_projection_contract::<T>();
        self.relation_record(relation_id)
            .and_then(|record| T::from_record(RelationProjectionRecord::new(&record)))
    }

    pub fn entity_records(&self, kind_id: KindId) -> Vec<EntityReadRecord> {
        self.reader()
            .visible_entities_of_kind(kind_id, self.version_id)
    }

    pub fn entity_record(&self, entity_id: EntityId) -> Option<EntityReadRecord> {
        self.reader()
            .entity_record_at_version(entity_id, self.version_id)
    }

    pub fn all_entity_records(&self) -> Vec<EntityReadRecord> {
        let records = self.reader().all_entity_records_at_version(self.version_id);
        debug_assert!(entity_records_are_canonical(&records));
        records
    }

    pub fn entity_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Vec<EntityReadRecord> {
        self.reader()
            .visible_entities_of_kind_in_partition(partition_id, kind_id, self.version_id)
    }

    pub fn relation_records(&self, kind_id: KindId) -> Vec<RelationReadRecord> {
        self.reader()
            .visible_relations_of_kind(kind_id, self.version_id)
    }

    pub fn relation_record(&self, relation_id: RelationId) -> Option<RelationReadRecord> {
        self.reader()
            .relation_record_at_version(relation_id, self.version_id)
    }

    pub fn all_relation_records(&self) -> Vec<RelationReadRecord> {
        let records = self
            .reader()
            .all_relation_records_at_version(self.version_id);
        debug_assert!(relation_records_are_canonical(&records));
        records
    }

    pub fn relation_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Vec<RelationReadRecord> {
        self.reader()
            .visible_relations_of_kind_in_partition(partition_id, kind_id, self.version_id)
    }

    fn reader(&self) -> VisibilityReadContext<'runtime> {
        VisibilityReadContext::new(self.runtime)
    }

    fn assert_entity_projection_contract<T: EntityRecordProjection>(&self) {
        assert_declared_projection_aspects(
            self.runtime,
            T::required_aspects(),
            self.runtime.entity_aspect_plan(T::KIND),
            "entity",
            T::KIND,
        );
    }

    fn assert_relation_projection_contract<T: RelationRecordProjection>(&self) {
        assert_declared_projection_aspects(
            self.runtime,
            T::required_aspects(),
            self.runtime.relation_aspect_plan(T::KIND),
            "relation",
            T::KIND,
        );
    }
}

fn entity_records_are_canonical(records: &[EntityReadRecord]) -> bool {
    records
        .windows(2)
        .all(|window| window[0].entity_id <= window[1].entity_id)
}

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn project_version(&self, version_id: VersionId) -> VisibilityProjectionView<'runtime> {
        VisibilityProjectionView::new(self.runtime(), version_id)
    }

    pub fn project_snapshot(
        &self,
        handle: &SnapshotHandle,
    ) -> Option<VisibilityProjectionView<'runtime>> {
        let snapshot = resolve_snapshot_handle(self.runtime(), handle)?;
        Some(VisibilityProjectionView::new(
            self.runtime(),
            snapshot.version_id,
        ))
    }
}
