use crate::identity::data::{EntityId, KindId, RelationId};
use crate::storage::data::{EntityReadRecord, RecordLifecycleState, RelationReadRecord};
use forge_foundational::facade::{
    AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView, FieldKey,
    StructAspectValue,
};

use super::contracts::ProjectionAspectScope;

pub trait EntityRecordProjection: Sized {
    const KIND: KindId;

    fn projection_scope() -> ProjectionAspectScope {
        ProjectionAspectScope::empty()
    }

    fn from_record(record: EntityProjectionRecord<'_>) -> Option<Self>;
}

pub trait RelationRecordProjection: Sized {
    const KIND: KindId;

    fn projection_scope() -> ProjectionAspectScope {
        ProjectionAspectScope::empty()
    }

    fn from_record(record: RelationProjectionRecord<'_>) -> Option<Self>;
}

#[derive(Debug, Clone, Copy)]
pub struct EntityProjectionRecord<'a> {
    record: &'a EntityReadRecord,
    projection_scope: &'a ProjectionAspectScope,
}

impl<'a> EntityProjectionRecord<'a> {
    pub(crate) const fn new(
        record: &'a EntityReadRecord,
        projection_scope: &'a ProjectionAspectScope,
    ) -> Self {
        Self {
            record,
            projection_scope,
        }
    }

    pub const fn entity_id(self) -> EntityId {
        self.record.entity_id
    }

    pub const fn kind_id(self) -> KindId {
        self.record.kind.kind_id
    }

    pub fn kind_name(self) -> &'a str {
        &self.record.kind.kind_name
    }

    pub const fn lifecycle(self) -> RecordLifecycleState {
        self.record.lifecycle
    }

    fn authoritative_aspect_state(self) -> Option<&'a AuthoritativeRecordAspectState> {
        self.record.authoritative_aspect_state.as_ref()
    }

    pub fn aspect_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
    ) -> Option<&'a AspectValue> {
        if !self.projection_scope.contains_whole_aspect(aspect_key) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(value) => Some(value),
            ContractValidatedAspectValueView::Struct(_) => None,
        }
    }

    pub fn struct_aspect_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
    ) -> Option<&'a StructAspectValue> {
        if !self.projection_scope.contains_whole_aspect(aspect_key) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => Some(value),
        }
    }

    pub fn aspect_field_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
        field: &FieldKey,
    ) -> Option<&'a AspectValue> {
        if !self.projection_scope.contains_field(aspect_key, field) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => value.get(field),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RelationProjectionRecord<'a> {
    record: &'a RelationReadRecord,
    projection_scope: &'a ProjectionAspectScope,
}

impl<'a> RelationProjectionRecord<'a> {
    pub(crate) const fn new(
        record: &'a RelationReadRecord,
        projection_scope: &'a ProjectionAspectScope,
    ) -> Self {
        Self {
            record,
            projection_scope,
        }
    }

    pub const fn relation_id(self) -> RelationId {
        self.record.relation_id
    }

    pub const fn kind_id(self) -> KindId {
        self.record.kind.kind_id
    }

    pub fn kind_name(self) -> &'a str {
        &self.record.kind.kind_name
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

    fn authoritative_aspect_state(self) -> Option<&'a AuthoritativeRecordAspectState> {
        self.record.authoritative_aspect_state.as_ref()
    }

    pub fn aspect_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
    ) -> Option<&'a AspectValue> {
        if !self.projection_scope.contains_whole_aspect(aspect_key) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(value) => Some(value),
            ContractValidatedAspectValueView::Struct(_) => None,
        }
    }

    pub fn struct_aspect_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
    ) -> Option<&'a StructAspectValue> {
        if !self.projection_scope.contains_whole_aspect(aspect_key) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => Some(value),
        }
    }

    pub fn aspect_field_value(
        self,
        aspect_key: &forge_foundational::facade::AspectKey,
        field: &FieldKey,
    ) -> Option<&'a AspectValue> {
        if !self.projection_scope.contains_field(aspect_key, field) {
            return None;
        }
        match self.authoritative_aspect_state()?.get(aspect_key)?.view() {
            ContractValidatedAspectValueView::Scalar(_) => None,
            ContractValidatedAspectValueView::Struct(value) => value.get(field),
        }
    }
}
