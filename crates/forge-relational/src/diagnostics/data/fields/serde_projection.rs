use forge_foundational::facade::{AspectMask, AspectValueLocator, DiagnosticMask};
use serde_json::{Map, Value};

use super::aspect_value_diagnostic_terms::{
    aspect_value_diagnostic_value, struct_aspect_value_diagnostic_value,
};
use super::RelationalDiagnosticValue;
use crate::identity::data::{EntityId, PartitionId, RelationId};

pub(super) fn diagnostic_value_to_serde_value(value: &RelationalDiagnosticValue) -> Value {
    match value {
        RelationalDiagnosticValue::Null => Value::Null,
        RelationalDiagnosticValue::Bool(value) => Value::Bool(*value),
        RelationalDiagnosticValue::Unsigned(value) => Value::from(*value),
        RelationalDiagnosticValue::Signed(value) => Value::from(*value),
        RelationalDiagnosticValue::String(value) => Value::String(value.clone()),
        RelationalDiagnosticValue::Array(values) => {
            Value::Array(values.iter().map(diagnostic_value_to_serde_value).collect())
        }
        RelationalDiagnosticValue::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(key, value)| (key.clone(), diagnostic_value_to_serde_value(value)))
                .collect(),
        ),
        RelationalDiagnosticValue::AspectKey(value) => Value::String(value.as_str().to_string()),
        RelationalDiagnosticValue::FieldKey(value) => Value::String(value.as_str().to_string()),
        RelationalDiagnosticValue::FieldPath(fields) => Value::Array(
            fields
                .iter()
                .map(|field| Value::String(field.as_str().to_string()))
                .collect(),
        ),
        RelationalDiagnosticValue::AspectValue(value) => {
            diagnostic_value_to_serde_value(&aspect_value_diagnostic_value(value))
        }
        RelationalDiagnosticValue::AspectValueLocator(locator) => {
            aspect_value_locator_serde_value(locator)
        }
        RelationalDiagnosticValue::StructAspectValue(value) => {
            diagnostic_value_to_serde_value(&struct_aspect_value_diagnostic_value(value))
        }
        RelationalDiagnosticValue::DiagnosticMask(mask) => diagnostic_mask_serde_value(mask),
        RelationalDiagnosticValue::PartitionId(value) => Value::from(value.as_u64()),
        RelationalDiagnosticValue::KindId(value) => Value::from(value.as_u64()),
        RelationalDiagnosticValue::VersionId(value) => Value::from(value.as_u64()),
        RelationalDiagnosticValue::LineageId(value) => Value::from(value.as_u64()),
        RelationalDiagnosticValue::CommitId(value) => Value::from(value.0),
        RelationalDiagnosticValue::BranchId(value) => Value::String(value.0.clone()),
        RelationalDiagnosticValue::SnapshotId(value) => Value::from(value.0),
        RelationalDiagnosticValue::DurableCheckpointId(value) => Value::from(value.0),
        RelationalDiagnosticValue::DurableSegmentId(value) => Value::from(value.0),
        RelationalDiagnosticValue::DerivedIndexId(value) => Value::from(value.0),
        RelationalDiagnosticValue::DerivedIndexGenerationId(value) => Value::from(value.0),
        RelationalDiagnosticValue::CorrespondenceCandidateId(value) => Value::from(value.0),
        RelationalDiagnosticValue::PatchStreamPosition(value) => Value::from(value.0),
        RelationalDiagnosticValue::ReplaySchemaVersion(value) => Value::from(value.0),
        RelationalDiagnosticValue::SchemaId(value) => Value::String(value.0.clone()),
        RelationalDiagnosticValue::SchemaVersionId(value) => Value::from(value.0),
        RelationalDiagnosticValue::ContractId(value) => Value::String(value.as_str().to_string()),
        RelationalDiagnosticValue::SchemaBoundaryFingerprint(value) => {
            Value::String(format!("{value:?}"))
        }
        RelationalDiagnosticValue::DescriptorSemanticsVersion(value) => Value::from(value.0),
        RelationalDiagnosticValue::DescriptorCanonicalBasisVersion(value) => Value::from(value.0),
        RelationalDiagnosticValue::EntityId(value) => entity_id_serde_value(*value),
        RelationalDiagnosticValue::RelationId(value) => relation_id_serde_value(*value),
    }
}

fn aspect_value_locator_serde_value(locator: &AspectValueLocator) -> Value {
    match locator {
        AspectValueLocator::WholeAspect(aspect) => Value::Object(Map::from_iter([
            (
                "locator_kind".to_string(),
                Value::String("whole_aspect".to_string()),
            ),
            (
                "authority".to_string(),
                Value::String(format!("{:?}", aspect.authority())),
            ),
            (
                "aspect_key".to_string(),
                Value::String(aspect.aspect_key().as_str().to_string()),
            ),
        ])),
        AspectValueLocator::StructField(field) => Value::Object(Map::from_iter([
            (
                "locator_kind".to_string(),
                Value::String("struct_field".to_string()),
            ),
            (
                "authority".to_string(),
                Value::String(format!("{:?}", field.aspect().authority())),
            ),
            (
                "aspect_key".to_string(),
                Value::String(field.aspect().aspect_key().as_str().to_string()),
            ),
            (
                "field_path".to_string(),
                Value::Array(
                    field
                        .field_path()
                        .fields()
                        .iter()
                        .map(|field| Value::String(field.as_str().to_string()))
                        .collect(),
                ),
            ),
        ])),
    }
}

fn diagnostic_mask_serde_value(mask: &AspectMask<DiagnosticMask>) -> Value {
    if mask.is_whole_aspect() {
        return Value::Object(Map::from_iter([(
            "mask_kind".to_string(),
            Value::String("whole_aspect".to_string()),
        )]));
    }

    Value::Object(Map::from_iter([
        ("mask_kind".to_string(), Value::String("fields".to_string())),
        (
            "field_paths".to_string(),
            Value::Array(
                mask.paths()
                    .iter()
                    .map(|field_path| {
                        Value::Array(
                            field_path
                                .fields()
                                .iter()
                                .map(|field| Value::String(field.as_str().to_string()))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        ),
    ]))
}

fn entity_id_serde_value(entity_id: EntityId) -> Value {
    record_id_serde_value(
        "entity",
        entity_id.partition_id,
        entity_id.local_slot_value(),
        entity_id.generation_value(),
    )
}

fn relation_id_serde_value(relation_id: RelationId) -> Value {
    record_id_serde_value(
        "relation",
        relation_id.partition_id,
        relation_id.local_slot_value(),
        relation_id.generation_value(),
    )
}

fn record_id_serde_value(
    record_kind: &'static str,
    partition_id: PartitionId,
    local_slot: u64,
    generation: u32,
) -> Value {
    Value::Object(Map::from_iter([
        (
            "record_kind".to_string(),
            Value::String(record_kind.to_string()),
        ),
        (
            "partition_id".to_string(),
            Value::from(partition_id.as_u64()),
        ),
        ("local_slot".to_string(), Value::from(local_slot)),
        ("generation".to_string(), Value::from(generation as u64)),
    ]))
}
