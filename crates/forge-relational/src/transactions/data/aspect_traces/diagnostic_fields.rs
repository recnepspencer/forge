use crate::diagnostics::data::{
    aspect_shape_diagnostic_value, RelationalDiagnosticFields, RelationalDiagnosticValue,
};
use crate::identity::data::{EntityId, RelationId};
use crate::publication::patch::data::RecordStructuralChange;

use super::{
    AspectEmissionTrace, AspectEvaluationTrace, AspectEvaluationTraceRow,
    AspectLifecycleTransitionClass, AspectTraceEvidence, AspectTracePatchOperation,
    AspectTracePatchSetValue, RecordRef,
};

pub(super) fn evaluation_trace_diagnostic_fields(
    trace: &AspectEvaluationTrace,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        ("target", record_ref_value(&trace.target)),
        (
            "kind_id",
            RelationalDiagnosticValue::Unsigned(trace.kind_id.as_u64()),
        ),
        (
            "plan_revision",
            RelationalDiagnosticValue::string(trace.plan_revision.0.to_string()),
        ),
        (
            "structural_change",
            structural_change_value(trace.structural_change),
        ),
        (
            "changed_aspects",
            canonical_aspect_set_value(&trace.changed_aspects),
        ),
        (
            "contains_opaque_aspect",
            RelationalDiagnosticValue::Bool(trace.contains_opaque_aspect),
        ),
        (
            "binding_rows",
            RelationalDiagnosticValue::array(
                trace
                    .binding_rows
                    .iter()
                    .map(aspect_evaluation_trace_row_value),
            ),
        ),
    ])
    .into()
}

pub(super) fn emission_trace_diagnostic_fields(
    trace: &AspectEmissionTrace,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        ("target", record_ref_value(&trace.target)),
        (
            "patch_position",
            RelationalDiagnosticValue::Unsigned(trace.patch_position.0),
        ),
        (
            "patch_record_index",
            RelationalDiagnosticValue::Unsigned(trace.patch_record_index),
        ),
        (
            "structural_change",
            structural_change_value(trace.structural_change),
        ),
        (
            "changed_aspects",
            canonical_aspect_set_value(&trace.changed_aspects),
        ),
        (
            "contains_opaque_aspect",
            RelationalDiagnosticValue::Bool(trace.contains_opaque_aspect),
        ),
    ])
    .into()
}

fn aspect_evaluation_trace_row_value(row: &AspectEvaluationTraceRow) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "aspect_key",
            RelationalDiagnosticValue::AspectKey(row.aspect_key.clone()),
        ),
        ("changed", RelationalDiagnosticValue::Bool(row.changed)),
        (
            "aspect_shape",
            aspect_shape_diagnostic_value(&row.aspect_shape),
        ),
        ("evidence", aspect_trace_evidence_value(&row.evidence)),
    ])
}

fn aspect_trace_evidence_value(evidence: &AspectTraceEvidence) -> RelationalDiagnosticValue {
    match evidence {
        AspectTraceEvidence::ScalarAspectPresenceOrValue {
            old_present,
            new_present,
            old_value,
            new_value,
        } => RelationalDiagnosticValue::object([
            (
                "evidence_kind",
                RelationalDiagnosticValue::string("scalar_aspect_presence_or_value"),
            ),
            ("old_present", RelationalDiagnosticValue::Bool(*old_present)),
            ("new_present", RelationalDiagnosticValue::Bool(*new_present)),
            (
                "old_value",
                RelationalDiagnosticValue::optional(
                    old_value
                        .clone()
                        .map(RelationalDiagnosticValue::AspectValue),
                ),
            ),
            (
                "new_value",
                RelationalDiagnosticValue::optional(
                    new_value
                        .clone()
                        .map(RelationalDiagnosticValue::AspectValue),
                ),
            ),
        ]),
        AspectTraceEvidence::StructAspectPresenceOrValue {
            old_present,
            new_present,
            old_value,
            new_value,
        } => RelationalDiagnosticValue::object([
            (
                "evidence_kind",
                RelationalDiagnosticValue::string("struct_aspect_presence_or_value"),
            ),
            ("old_present", RelationalDiagnosticValue::Bool(*old_present)),
            ("new_present", RelationalDiagnosticValue::Bool(*new_present)),
            (
                "old_value",
                RelationalDiagnosticValue::optional(
                    old_value
                        .clone()
                        .map(RelationalDiagnosticValue::StructAspectValue),
                ),
            ),
            (
                "new_value",
                RelationalDiagnosticValue::optional(
                    new_value
                        .clone()
                        .map(RelationalDiagnosticValue::StructAspectValue),
                ),
            ),
        ]),
        AspectTraceEvidence::EndpointIdentity { old, new } => RelationalDiagnosticValue::object([
            (
                "evidence_kind",
                RelationalDiagnosticValue::string("endpoint_identity"),
            ),
            (
                "old",
                RelationalDiagnosticValue::optional(old.map(entity_id_value)),
            ),
            (
                "new",
                RelationalDiagnosticValue::optional(new.map(entity_id_value)),
            ),
        ]),
        AspectTraceEvidence::Lifecycle { transition } => RelationalDiagnosticValue::object([
            (
                "evidence_kind",
                RelationalDiagnosticValue::string("lifecycle"),
            ),
            ("transition", lifecycle_transition_value(*transition)),
        ]),
        AspectTraceEvidence::AuthoritativePatchOperation { operation } => {
            RelationalDiagnosticValue::object([
                (
                    "evidence_kind",
                    RelationalDiagnosticValue::string("authoritative_patch_operation"),
                ),
                ("operation", patch_operation_value(operation)),
            ])
        }
    }
}

fn patch_operation_value(operation: &AspectTracePatchOperation) -> RelationalDiagnosticValue {
    match operation {
        AspectTracePatchOperation::WholeAspectSet { value } => RelationalDiagnosticValue::object([
            (
                "operation_kind",
                RelationalDiagnosticValue::string("whole_aspect_set"),
            ),
            ("value", patch_set_value(value)),
        ]),
        AspectTracePatchOperation::WholeAspectClear => RelationalDiagnosticValue::object([(
            "operation_kind",
            RelationalDiagnosticValue::string("whole_aspect_clear"),
        )]),
        AspectTracePatchOperation::FieldLevelPatch {
            field_sets,
            field_clears,
        } => RelationalDiagnosticValue::object([
            (
                "operation_kind",
                RelationalDiagnosticValue::string("field_level_patch"),
            ),
            (
                "field_sets",
                RelationalDiagnosticValue::array(field_sets.iter().map(|(field, value)| {
                    RelationalDiagnosticValue::object([
                        ("field", RelationalDiagnosticValue::FieldKey(field.clone())),
                        (
                            "value",
                            RelationalDiagnosticValue::AspectValue(value.clone()),
                        ),
                    ])
                })),
            ),
            (
                "field_clears",
                RelationalDiagnosticValue::array(
                    field_clears
                        .iter()
                        .cloned()
                        .map(RelationalDiagnosticValue::FieldKey),
                ),
            ),
        ]),
    }
}

fn patch_set_value(value: &AspectTracePatchSetValue) -> RelationalDiagnosticValue {
    match value {
        AspectTracePatchSetValue::Scalar(value) => {
            RelationalDiagnosticValue::AspectValue(value.clone())
        }
        AspectTracePatchSetValue::Struct(value) => {
            RelationalDiagnosticValue::StructAspectValue(value.clone())
        }
    }
}

fn record_ref_value(record_ref: &RecordRef) -> RelationalDiagnosticValue {
    match record_ref {
        RecordRef::Entity(entity_id) => RelationalDiagnosticValue::object([
            ("record_kind", RelationalDiagnosticValue::string("entity")),
            ("id", entity_id_value(*entity_id)),
        ]),
        RecordRef::Relation(relation_id) => RelationalDiagnosticValue::object([
            ("record_kind", RelationalDiagnosticValue::string("relation")),
            ("id", relation_id_value(*relation_id)),
        ]),
    }
}

fn entity_id_value(entity_id: EntityId) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "partition_id",
            RelationalDiagnosticValue::Unsigned(entity_id.partition_value_u64()),
        ),
        (
            "local_slot",
            RelationalDiagnosticValue::Unsigned(entity_id.local_slot_value()),
        ),
        (
            "generation",
            RelationalDiagnosticValue::Unsigned(entity_id.generation_value() as u64),
        ),
    ])
}

fn relation_id_value(relation_id: RelationId) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "partition_id",
            RelationalDiagnosticValue::Unsigned(relation_id.partition_value_u64()),
        ),
        (
            "local_slot",
            RelationalDiagnosticValue::Unsigned(relation_id.local_slot_value()),
        ),
        (
            "generation",
            RelationalDiagnosticValue::Unsigned(relation_id.generation_value() as u64),
        ),
    ])
}

fn canonical_aspect_set_value(
    aspects: &[forge_foundational::facade::AspectKey],
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        aspects
            .iter()
            .cloned()
            .map(RelationalDiagnosticValue::AspectKey),
    )
}

fn structural_change_value(change: RecordStructuralChange) -> RelationalDiagnosticValue {
    match change {
        RecordStructuralChange::Created => RelationalDiagnosticValue::string("created"),
        RecordStructuralChange::Updated => RelationalDiagnosticValue::string("updated"),
        RecordStructuralChange::Deleted => RelationalDiagnosticValue::string("deleted"),
        RecordStructuralChange::RetainedForAudit => {
            RelationalDiagnosticValue::string("retained_for_audit")
        }
    }
}

fn lifecycle_transition_value(
    transition: AspectLifecycleTransitionClass,
) -> RelationalDiagnosticValue {
    match transition {
        AspectLifecycleTransitionClass::NoTransition => {
            RelationalDiagnosticValue::string("no_transition")
        }
        AspectLifecycleTransitionClass::Create => RelationalDiagnosticValue::string("create"),
        AspectLifecycleTransitionClass::Update => RelationalDiagnosticValue::string("update"),
        AspectLifecycleTransitionClass::Delete => RelationalDiagnosticValue::string("delete"),
        AspectLifecycleTransitionClass::RetainForAudit => {
            RelationalDiagnosticValue::string("retain_for_audit")
        }
    }
}
