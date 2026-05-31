use crate::capabilities::AspectPlanSource;
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;
use crate::storage::data::{EntityReadRecord, RelationReadRecord};

use super::contracts::{assert_declared_projection_aspects, ProjectionAspectScope};
use super::projection_records::{
    EntityProjectionRecord, EntityRecordProjection, RelationProjectionRecord,
    RelationRecordProjection,
};
use super::read_record_identity_ordering::{
    authoritative_entity_records_are_identity_ordered,
    authoritative_relation_records_are_identity_ordered,
};
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::super::reader::VisibilityReadContext;

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
        let projection_scope = self.assert_entity_projection_contract::<T>();
        self.authoritative_entity_records(T::KIND)
            .into_iter()
            .filter_map(|record| {
                T::from_record(EntityProjectionRecord::new(&record, &projection_scope))
            })
            .collect()
    }

    pub fn entities_in<T: EntityRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        let projection_scope = self.assert_entity_projection_contract::<T>();
        self.authoritative_entity_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| {
                T::from_record(EntityProjectionRecord::new(&record, &projection_scope))
            })
            .collect()
    }

    pub fn entity<T: EntityRecordProjection>(&self, entity_id: EntityId) -> Option<T> {
        let projection_scope = self.assert_entity_projection_contract::<T>();
        self.authoritative_entity_record(entity_id)
            .and_then(|record| {
                T::from_record(EntityProjectionRecord::new(&record, &projection_scope))
            })
    }

    pub fn entity_records_with_projection_scope<T>(
        &self,
        kind_id: KindId,
        projection_scope: ProjectionAspectScope,
        mut project: impl FnMut(EntityProjectionRecord<'_>) -> Option<T>,
    ) -> Vec<T> {
        self.assert_entity_projection_scope(kind_id, &projection_scope);
        self.authoritative_entity_records(kind_id)
            .into_iter()
            .filter_map(|record| project(EntityProjectionRecord::new(&record, &projection_scope)))
            .collect()
    }

    pub fn entity_record_with_projection_scope<T>(
        &self,
        entity_id: EntityId,
        projection_scope: ProjectionAspectScope,
        mut project: impl FnMut(EntityProjectionRecord<'_>) -> Option<T>,
    ) -> Option<T> {
        let record = self.authoritative_entity_record(entity_id)?;
        self.assert_entity_projection_scope(record.kind.kind_id, &projection_scope);
        project(EntityProjectionRecord::new(&record, &projection_scope))
    }

    pub fn relations<T: RelationRecordProjection>(&self) -> Vec<T> {
        let projection_scope = self.assert_relation_projection_contract::<T>();
        self.authoritative_relation_records(T::KIND)
            .into_iter()
            .filter_map(|record| {
                T::from_record(RelationProjectionRecord::new(&record, &projection_scope))
            })
            .collect()
    }

    pub fn relations_in<T: RelationRecordProjection>(&self, partition_id: PartitionId) -> Vec<T> {
        let projection_scope = self.assert_relation_projection_contract::<T>();
        self.authoritative_relation_records_in(partition_id, T::KIND)
            .into_iter()
            .filter_map(|record| {
                T::from_record(RelationProjectionRecord::new(&record, &projection_scope))
            })
            .collect()
    }

    pub fn relation<T: RelationRecordProjection>(&self, relation_id: RelationId) -> Option<T> {
        let projection_scope = self.assert_relation_projection_contract::<T>();
        self.authoritative_relation_record(relation_id)
            .and_then(|record| {
                T::from_record(RelationProjectionRecord::new(&record, &projection_scope))
            })
    }

    pub fn relation_records_with_projection_scope<T>(
        &self,
        kind_id: KindId,
        projection_scope: ProjectionAspectScope,
        mut project: impl FnMut(RelationProjectionRecord<'_>) -> Option<T>,
    ) -> Vec<T> {
        self.assert_relation_projection_scope(kind_id, &projection_scope);
        self.authoritative_relation_records(kind_id)
            .into_iter()
            .filter_map(|record| project(RelationProjectionRecord::new(&record, &projection_scope)))
            .collect()
    }

    pub fn relation_record_with_projection_scope<T>(
        &self,
        relation_id: RelationId,
        projection_scope: ProjectionAspectScope,
        mut project: impl FnMut(RelationProjectionRecord<'_>) -> Option<T>,
    ) -> Option<T> {
        let record = self.authoritative_relation_record(relation_id)?;
        self.assert_relation_projection_scope(record.kind.kind_id, &projection_scope);
        project(RelationProjectionRecord::new(&record, &projection_scope))
    }

    pub(crate) fn authoritative_entity_records(&self, kind_id: KindId) -> Vec<EntityReadRecord> {
        self.reader()
            .visible_entities_of_kind(kind_id, self.version_id)
    }

    pub(crate) fn authoritative_entity_record(
        &self,
        entity_id: EntityId,
    ) -> Option<EntityReadRecord> {
        self.reader()
            .authoritative_entity_record_at_version(entity_id, self.version_id)
    }

    pub(crate) fn entity_record_kind_id(&self, entity_id: EntityId) -> Option<KindId> {
        self.authoritative_entity_record(entity_id)
            .map(|record| record.kind.kind_id)
    }

    pub(crate) fn all_authoritative_entity_records(&self) -> Vec<EntityReadRecord> {
        let records = self
            .reader()
            .all_authoritative_entity_records_at_version(self.version_id);
        debug_assert!(authoritative_entity_records_are_identity_ordered(&records));
        records
    }

    pub(crate) fn authoritative_entity_records_in(
        &self,
        partition_id: PartitionId,
        kind_id: KindId,
    ) -> Vec<EntityReadRecord> {
        self.reader()
            .visible_entities_of_kind_in_partition(partition_id, kind_id, self.version_id)
    }

    pub(crate) fn authoritative_relation_records(
        &self,
        kind_id: KindId,
    ) -> Vec<RelationReadRecord> {
        self.reader()
            .visible_relations_of_kind(kind_id, self.version_id)
    }

    pub(crate) fn authoritative_relation_record(
        &self,
        relation_id: RelationId,
    ) -> Option<RelationReadRecord> {
        self.reader()
            .authoritative_relation_record_at_version(relation_id, self.version_id)
    }

    pub(crate) fn all_authoritative_relation_records(&self) -> Vec<RelationReadRecord> {
        let records = self
            .reader()
            .all_authoritative_relation_records_at_version(self.version_id);
        debug_assert!(authoritative_relation_records_are_identity_ordered(
            &records
        ));
        records
    }

    pub(crate) fn authoritative_relation_records_in(
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

    fn assert_entity_projection_contract<T: EntityRecordProjection>(
        &self,
    ) -> ProjectionAspectScope {
        let projection_scope = T::projection_scope();
        assert_declared_projection_aspects(
            &projection_scope,
            self.runtime.entity_aspect_plan(T::KIND),
            "entity",
            T::KIND,
        );
        projection_scope
    }

    fn assert_relation_projection_contract<T: RelationRecordProjection>(
        &self,
    ) -> ProjectionAspectScope {
        let projection_scope = T::projection_scope();
        assert_declared_projection_aspects(
            &projection_scope,
            self.runtime.relation_aspect_plan(T::KIND),
            "relation",
            T::KIND,
        );
        projection_scope
    }

    fn assert_entity_projection_scope(
        &self,
        kind_id: KindId,
        projection_scope: &ProjectionAspectScope,
    ) {
        assert_declared_projection_aspects(
            projection_scope,
            self.runtime.entity_aspect_plan(kind_id),
            "entity",
            kind_id,
        );
    }

    fn assert_relation_projection_scope(
        &self,
        kind_id: KindId,
        projection_scope: &ProjectionAspectScope,
    ) {
        assert_declared_projection_aspects(
            projection_scope,
            self.runtime.relation_aspect_plan(kind_id),
            "relation",
            kind_id,
        );
    }
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
