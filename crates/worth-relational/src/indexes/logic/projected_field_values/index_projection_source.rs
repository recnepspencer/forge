use crate::identity::data::{EntityId, KindId, RelationId};
use crate::logic::runtime::VisibilityProjectionView;
use crate::storage::data::{
    EntityReadRecord, RecordLifecycleState, RelationReadRecord, RelationalReadView,
};

pub(in crate::indexes::logic) enum IndexProjectionSource<'view, 'runtime> {
    Current(&'view VisibilityProjectionView<'runtime>),
    Reconstructed(&'view RelationalReadView),
}

impl IndexProjectionSource<'_, '_> {
    pub(in crate::indexes::logic) fn for_each_entity(
        &self,
        kind_id: KindId,
        mut visit: impl FnMut(&EntityReadRecord),
    ) {
        match self {
            Self::Current(projection) => {
                for record in projection.authoritative_entity_records(kind_id) {
                    visit(&record);
                }
            }
            Self::Reconstructed(read) => {
                for record in read.entities().iter().filter(|record| {
                    record.kind.kind_id == kind_id && record.lifecycle == RecordLifecycleState::Live
                }) {
                    visit(record);
                }
            }
        }
    }

    pub(in crate::indexes::logic) fn for_each_relation(
        &self,
        kind_id: KindId,
        mut visit: impl FnMut(&RelationReadRecord),
    ) {
        match self {
            Self::Current(projection) => {
                for record in projection.authoritative_relation_records(kind_id) {
                    visit(&record);
                }
            }
            Self::Reconstructed(read) => {
                for record in read.relations().iter().filter(|record| {
                    record.kind.kind_id == kind_id && record.lifecycle == RecordLifecycleState::Live
                }) {
                    visit(record);
                }
            }
        }
    }

    pub(in crate::indexes::logic) fn with_entity<T>(
        &self,
        entity_id: EntityId,
        inspect: impl FnOnce(&EntityReadRecord) -> T,
    ) -> Option<T> {
        match self {
            Self::Current(projection) => projection
                .authoritative_entity_record(entity_id)
                .as_ref()
                .map(inspect),
            Self::Reconstructed(read) => read.get_entity(entity_id).map(inspect),
        }
    }

    pub(in crate::indexes::logic) fn with_relation<T>(
        &self,
        relation_id: RelationId,
        inspect: impl FnOnce(&RelationReadRecord) -> T,
    ) -> Option<T> {
        match self {
            Self::Current(projection) => projection
                .authoritative_relation_record(relation_id)
                .as_ref()
                .map(inspect),
            Self::Reconstructed(read) => read.get_relation(relation_id).map(inspect),
        }
    }
}
