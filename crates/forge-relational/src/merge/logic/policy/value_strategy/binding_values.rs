use crate::merge::data::{VisibleMergeRecord, VisibleMergeRecordKind};
use forge_foundational::facade::{
    AspectKey, AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
};

use crate::merge::logic::policy::contexts::{
    BindingSide, PolicyReadViewContext, RuntimeAspectValueBinding, ValueLookupFailure,
};

pub(in crate::merge::logic::policy) fn binding_aspect_i64(
    _runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    side: BindingSide,
    source_view: &PolicyReadViewContext<'_>,
    target_view: &PolicyReadViewContext<'_>,
) -> Result<i64, ValueLookupFailure> {
    aspect_value_i64(binding_aspect_value(
        record,
        classification,
        binding,
        side,
        source_view,
        target_view,
    )?)
}

pub(in crate::merge::logic::policy) fn binding_aspect_i64_from_view(
    _runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_view: &PolicyReadViewContext<'_>,
) -> Result<i64, ValueLookupFailure> {
    aspect_value_i64(binding_aspect_value_from_view(
        record,
        classification,
        binding,
        base_view,
    )?)
}

fn binding_aspect_value(
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    side: BindingSide,
    source_view: &PolicyReadViewContext<'_>,
    target_view: &PolicyReadViewContext<'_>,
) -> Result<AspectValue, ValueLookupFailure> {
    let source_record_ref = &record.record_ref;
    let target_record_ref = classification
        .target_record
        .as_ref()
        .unwrap_or(&record.record_ref);
    match (&record.record_kind, binding, side) {
        (
            VisibleMergeRecordKind::Entity,
            RuntimeAspectValueBinding::EntityScalar(aspect_key),
            BindingSide::Source,
        ) => match source_record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => source_view
                .entity_for_record(*entity_id, record.source_lineage_id.or(record.lineage_id))
                .ok_or(ValueLookupFailure::MissingRecordBasis)
                .and_then(|entity| {
                    scalar_from_authoritative_state(
                        entity.authoritative_aspect_state.as_ref(),
                        aspect_key,
                    )
                }),
            _ => Err(ValueLookupFailure::MissingRecordBasis),
        },
        (
            VisibleMergeRecordKind::Entity,
            RuntimeAspectValueBinding::EntityScalar(aspect_key),
            BindingSide::Target,
        ) => match target_record_ref {
            crate::transactions::data::RecordRef::Entity(entity_id) => target_view
                .entity_for_record(*entity_id, record.target_lineage_id.or(record.lineage_id))
                .ok_or(ValueLookupFailure::MissingRecordBasis)
                .and_then(|entity| {
                    scalar_from_authoritative_state(
                        entity.authoritative_aspect_state.as_ref(),
                        aspect_key,
                    )
                }),
            _ => Err(ValueLookupFailure::MissingRecordBasis),
        },
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationScalar(aspect_key),
            BindingSide::Source,
        ) => match source_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => source_view
                .relation_for_record(*relation_id)
                .ok_or(ValueLookupFailure::MissingRecordBasis)
                .and_then(|relation| {
                    scalar_from_authoritative_state(
                        relation.authoritative_aspect_state.as_ref(),
                        aspect_key,
                    )
                }),
            _ => Err(ValueLookupFailure::MissingRecordBasis),
        },
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationScalar(aspect_key),
            BindingSide::Target,
        ) => match target_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => target_view
                .relation_for_record(*relation_id)
                .ok_or(ValueLookupFailure::MissingRecordBasis)
                .and_then(|relation| {
                    scalar_from_authoritative_state(
                        relation.authoritative_aspect_state.as_ref(),
                        aspect_key,
                    )
                }),
            _ => Err(ValueLookupFailure::MissingRecordBasis),
        },
        (
            _,
            RuntimeAspectValueBinding::EntityStruct | RuntimeAspectValueBinding::RelationStruct,
            _,
        ) => Err(ValueLookupFailure::InvalidValueShape),
        _ => Err(ValueLookupFailure::MissingRecordBasis),
    }
}

fn binding_aspect_value_from_view(
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_view: &PolicyReadViewContext<'_>,
) -> Result<AspectValue, ValueLookupFailure> {
    let base_record_ref = classification
        .target_record
        .as_ref()
        .unwrap_or(&record.record_ref);
    match (&record.record_kind, binding) {
        (VisibleMergeRecordKind::Entity, RuntimeAspectValueBinding::EntityScalar(aspect_key)) => {
            match base_record_ref {
                crate::transactions::data::RecordRef::Entity(entity_id) => base_view
                    .entity_for_record(
                        *entity_id,
                        record
                            .source_lineage_id
                            .or(record.target_lineage_id)
                            .or(record.lineage_id),
                    )
                    .ok_or(ValueLookupFailure::MissingRecordBasis)
                    .and_then(|entity| {
                        scalar_from_authoritative_state(
                            entity.authoritative_aspect_state.as_ref(),
                            aspect_key,
                        )
                    }),
                _ => Err(ValueLookupFailure::MissingRecordBasis),
            }
        }
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationScalar(aspect_key),
        ) => match base_record_ref {
            crate::transactions::data::RecordRef::Relation(relation_id) => base_view
                .relation_for_record(*relation_id)
                .ok_or(ValueLookupFailure::MissingRecordBasis)
                .and_then(|relation| {
                    scalar_from_authoritative_state(
                        relation.authoritative_aspect_state.as_ref(),
                        aspect_key,
                    )
                }),
            _ => Err(ValueLookupFailure::MissingRecordBasis),
        },
        (
            _,
            RuntimeAspectValueBinding::EntityStruct | RuntimeAspectValueBinding::RelationStruct,
        ) => Err(ValueLookupFailure::InvalidValueShape),
        _ => Err(ValueLookupFailure::MissingRecordBasis),
    }
}

fn scalar_from_authoritative_state(
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
    aspect_key: &AspectKey,
) -> Result<AspectValue, ValueLookupFailure> {
    let Some(entry) = authoritative_state.and_then(|state| state.get(aspect_key)) else {
        return Err(ValueLookupFailure::MissingField);
    };
    match entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Ok(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => Err(ValueLookupFailure::InvalidValueShape),
    }
}

fn aspect_value_i64(value: AspectValue) -> Result<i64, ValueLookupFailure> {
    match value {
        AspectValue::Int8(value) => Ok(i64::from(value)),
        AspectValue::Int16(value) => Ok(i64::from(value)),
        AspectValue::Int32(value) => Ok(i64::from(value)),
        AspectValue::Int64(value) => Ok(value),
        AspectValue::UInt8(value) => Ok(i64::from(value)),
        AspectValue::UInt16(value) => Ok(i64::from(value)),
        AspectValue::UInt32(value) => Ok(i64::from(value)),
        AspectValue::UInt64(value) => {
            i64::try_from(value).map_err(|_| ValueLookupFailure::InvalidValueShape)
        }
        _ => Err(ValueLookupFailure::InvalidValueShape),
    }
}
