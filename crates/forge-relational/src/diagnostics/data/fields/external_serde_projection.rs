use forge_foundational::facade::{AspectMask, AspectValueLocator, DiagnosticMask};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::aspect_value_diagnostic_terms::{
    aspect_value_diagnostic_value, struct_aspect_value_diagnostic_value,
};
use super::{RelationalDiagnosticFields, RelationalDiagnosticValue};
use crate::identity::data::PartitionId;

#[path = "external_serde_projection/projected_value.rs"]
mod projected_value;

use projected_value::ExternalSerdeDiagnosticProjectionValue;

pub(super) fn serialize_diagnostic_fields<S>(
    fields: &RelationalDiagnosticFields,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    project_diagnostic_value_for_external_serde(fields.root()).serialize(serializer)
}

pub(super) fn deserialize_diagnostic_fields<'de, D>(
    deserializer: D,
) -> Result<RelationalDiagnosticFields, D::Error>
where
    D: Deserializer<'de>,
{
    ExternalSerdeDiagnosticProjectionValue::deserialize(deserializer)
        .map(recover_external_serde_diagnostic_fields)
}

pub(super) fn external_serde_projection_equal(
    left: &RelationalDiagnosticValue,
    right: &RelationalDiagnosticValue,
) -> bool {
    project_diagnostic_value_for_external_serde(left)
        == project_diagnostic_value_for_external_serde(right)
}

pub(super) fn typed_external_serde_projection_tree(
    value: &RelationalDiagnosticValue,
) -> RelationalDiagnosticValue {
    recover_external_serde_diagnostic_value(project_diagnostic_value_for_external_serde(value))
}

fn recover_external_serde_diagnostic_fields(
    projection: ExternalSerdeDiagnosticProjectionValue,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticFields::from_diagnostic_value(recover_external_serde_diagnostic_value(
        projection,
    ))
}

fn recover_external_serde_diagnostic_value(
    projection: ExternalSerdeDiagnosticProjectionValue,
) -> RelationalDiagnosticValue {
    match projection {
        ExternalSerdeDiagnosticProjectionValue::Null => RelationalDiagnosticValue::Null,
        ExternalSerdeDiagnosticProjectionValue::Bool(value) => {
            RelationalDiagnosticValue::Bool(value)
        }
        ExternalSerdeDiagnosticProjectionValue::Unsigned(value) => {
            RelationalDiagnosticValue::Unsigned(value)
        }
        ExternalSerdeDiagnosticProjectionValue::Signed(value) => {
            RelationalDiagnosticValue::Signed(value)
        }
        ExternalSerdeDiagnosticProjectionValue::String(value) => {
            RelationalDiagnosticValue::String(value)
        }
        ExternalSerdeDiagnosticProjectionValue::Array(values) => RelationalDiagnosticValue::array(
            values
                .into_iter()
                .map(recover_external_serde_diagnostic_value),
        ),
        ExternalSerdeDiagnosticProjectionValue::Object(fields) => {
            RelationalDiagnosticValue::object(
                fields
                    .into_iter()
                    .map(|(key, value)| (key, recover_external_serde_diagnostic_value(value))),
            )
        }
    }
}

fn project_diagnostic_value_for_external_serde(
    value: &RelationalDiagnosticValue,
) -> ExternalSerdeDiagnosticProjectionValue {
    match value {
        RelationalDiagnosticValue::Null => ExternalSerdeDiagnosticProjectionValue::Null,
        RelationalDiagnosticValue::Bool(value) => {
            ExternalSerdeDiagnosticProjectionValue::Bool(*value)
        }
        RelationalDiagnosticValue::Unsigned(value) => {
            ExternalSerdeDiagnosticProjectionValue::Unsigned(*value)
        }
        RelationalDiagnosticValue::Signed(value) => {
            ExternalSerdeDiagnosticProjectionValue::Signed(*value)
        }
        RelationalDiagnosticValue::String(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.clone())
        }
        RelationalDiagnosticValue::Array(values) => ExternalSerdeDiagnosticProjectionValue::Array(
            values
                .iter()
                .map(project_diagnostic_value_for_external_serde)
                .collect(),
        ),
        RelationalDiagnosticValue::Object(fields) => {
            ExternalSerdeDiagnosticProjectionValue::Object(
                fields
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            project_diagnostic_value_for_external_serde(value),
                        )
                    })
                    .collect(),
            )
        }
        RelationalDiagnosticValue::AspectKey(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::FieldKey(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::FieldPath(fields) => {
            ExternalSerdeDiagnosticProjectionValue::Array(
                fields
                    .iter()
                    .map(|field| {
                        ExternalSerdeDiagnosticProjectionValue::String(field.as_str().to_string())
                    })
                    .collect(),
            )
        }
        RelationalDiagnosticValue::AspectValue(value) => {
            project_diagnostic_value_for_external_serde(&aspect_value_diagnostic_value(value))
        }
        RelationalDiagnosticValue::AspectValueLocator(locator) => {
            aspect_value_locator_external_serde_projection(locator)
        }
        RelationalDiagnosticValue::StructAspectValue(value) => {
            project_diagnostic_value_for_external_serde(&struct_aspect_value_diagnostic_value(
                value,
            ))
        }
        RelationalDiagnosticValue::DiagnosticMask(mask) => {
            diagnostic_mask_external_serde_projection(mask)
        }
        RelationalDiagnosticValue::PartitionId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::KindId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::VersionId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::LineageId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::CommitId(value) => unsigned(value.0),
        RelationalDiagnosticValue::BranchId(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.0.clone())
        }
        RelationalDiagnosticValue::SnapshotId(value) => unsigned(value.0),
        RelationalDiagnosticValue::DurableCheckpointId(value) => unsigned(value.0),
        RelationalDiagnosticValue::DurableSegmentId(value) => unsigned(value.0),
        RelationalDiagnosticValue::DerivedIndexId(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::DerivedIndexGenerationId(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::CorrespondenceCandidateId(value) => unsigned(value.0),
        RelationalDiagnosticValue::PatchStreamPosition(value) => unsigned(value.0),
        RelationalDiagnosticValue::ReplaySchemaVersion(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::SchemaId(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.0.clone())
        }
        RelationalDiagnosticValue::SchemaVersionId(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::ContractId(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::SchemaBoundaryFingerprint(value) => {
            ExternalSerdeDiagnosticProjectionValue::String(format!("{value:?}"))
        }
        RelationalDiagnosticValue::DescriptorSemanticsVersion(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::DescriptorCanonicalBasisVersion(value) => {
            unsigned(value.0 as u64)
        }
        RelationalDiagnosticValue::EntityId(value) => record_id_external_serde_projection(
            "entity",
            value.partition_id,
            value.local_slot_value(),
            value.generation_value(),
        ),
        RelationalDiagnosticValue::RelationId(value) => record_id_external_serde_projection(
            "relation",
            value.partition_id,
            value.local_slot_value(),
            value.generation_value(),
        ),
    }
}

fn aspect_value_locator_external_serde_projection(
    locator: &AspectValueLocator,
) -> ExternalSerdeDiagnosticProjectionValue {
    match locator {
        AspectValueLocator::WholeAspect(aspect) => object([
            ("locator_kind", string("whole_aspect")),
            ("authority", string(format!("{:?}", aspect.authority()))),
            ("aspect_key", string(aspect.aspect_key().as_str())),
        ]),
        AspectValueLocator::StructField(field) => object([
            ("locator_kind", string("struct_field")),
            (
                "authority",
                string(format!("{:?}", field.aspect().authority())),
            ),
            ("aspect_key", string(field.aspect().aspect_key().as_str())),
            (
                "field_path",
                ExternalSerdeDiagnosticProjectionValue::Array(
                    field
                        .field_path()
                        .fields()
                        .iter()
                        .map(|field| string(field.as_str()))
                        .collect(),
                ),
            ),
        ]),
    }
}

fn diagnostic_mask_external_serde_projection(
    mask: &AspectMask<DiagnosticMask>,
) -> ExternalSerdeDiagnosticProjectionValue {
    if mask.is_whole_aspect() {
        return object([("mask_kind", string("whole_aspect"))]);
    }

    object([
        ("mask_kind", string("fields")),
        (
            "field_paths",
            ExternalSerdeDiagnosticProjectionValue::Array(
                mask.paths()
                    .iter()
                    .map(|field_path| {
                        ExternalSerdeDiagnosticProjectionValue::Array(
                            field_path
                                .fields()
                                .iter()
                                .map(|field| string(field.as_str()))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        ),
    ])
}

fn record_id_external_serde_projection(
    record_kind: &'static str,
    partition_id: PartitionId,
    local_slot: u64,
    generation: u32,
) -> ExternalSerdeDiagnosticProjectionValue {
    object([
        ("record_kind", string(record_kind)),
        (
            "partition_id",
            ExternalSerdeDiagnosticProjectionValue::Unsigned(partition_id.as_u64()),
        ),
        (
            "local_slot",
            ExternalSerdeDiagnosticProjectionValue::Unsigned(local_slot),
        ),
        (
            "generation",
            ExternalSerdeDiagnosticProjectionValue::Unsigned(generation as u64),
        ),
    ])
}

fn object(
    fields: impl IntoIterator<Item = (&'static str, ExternalSerdeDiagnosticProjectionValue)>,
) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )
}

fn string(value: impl Into<String>) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::String(value.into())
}

fn unsigned(value: u64) -> ExternalSerdeDiagnosticProjectionValue {
    ExternalSerdeDiagnosticProjectionValue::Unsigned(value)
}
