mod base_commit_patch_values;
mod scalar_values;
mod visible_record_basis;

use crate::history::data::CommitId;
use crate::merge::data::{VisibleMergeRecord, VisibleMergeRecordKind};
use crate::merge::logic::policy::contexts::{
    BindingSide, PolicyReadViewContext, RuntimeAspectValueBinding, ValueLookupFailure,
};
use forge_foundational::facade::AspectValue;

use base_commit_patch_values::scalar_from_base_commit_patch;
use scalar_values::{aspect_value_i64, scalar_from_authoritative_state};
use visible_record_basis::{
    entity_basis_for_visible_record, entity_basis_record_refs, relation_basis_for_visible_record,
    relation_basis_record_refs,
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
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_commit_id: CommitId,
    base_view: &PolicyReadViewContext<'_>,
) -> Result<i64, ValueLookupFailure> {
    aspect_value_i64(binding_aspect_value_from_view(
        runtime,
        record,
        classification,
        binding,
        base_commit_id,
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
    runtime: &crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
    classification: &crate::merge::data::MergeConflictClassification,
    binding: &RuntimeAspectValueBinding,
    base_commit_id: CommitId,
    base_view: &PolicyReadViewContext<'_>,
) -> Result<AspectValue, ValueLookupFailure> {
    match (&record.record_kind, binding) {
        (VisibleMergeRecordKind::Entity, RuntimeAspectValueBinding::EntityScalar(aspect_key)) => {
            if let Some(entity) = entity_basis_for_visible_record(record, classification, base_view)
            {
                return scalar_from_authoritative_state(
                    entity.authoritative_aspect_state.as_ref(),
                    aspect_key,
                );
            }
            scalar_from_base_commit_patch(
                runtime,
                base_commit_id,
                &entity_basis_record_refs(record, classification),
                aspect_key,
            )
            .ok_or(ValueLookupFailure::MissingRecordBasis)
        }
        (
            VisibleMergeRecordKind::Relation,
            RuntimeAspectValueBinding::RelationScalar(aspect_key),
        ) => {
            if let Some(relation) =
                relation_basis_for_visible_record(record, classification, base_view)
            {
                return scalar_from_authoritative_state(
                    relation.authoritative_aspect_state.as_ref(),
                    aspect_key,
                );
            }
            scalar_from_base_commit_patch(
                runtime,
                base_commit_id,
                &relation_basis_record_refs(record, classification),
                aspect_key,
            )
            .ok_or(ValueLookupFailure::MissingRecordBasis)
        }
        (
            _,
            RuntimeAspectValueBinding::EntityStruct | RuntimeAspectValueBinding::RelationStruct,
        ) => Err(ValueLookupFailure::InvalidValueShape),
        _ => Err(ValueLookupFailure::MissingRecordBasis),
    }
}
