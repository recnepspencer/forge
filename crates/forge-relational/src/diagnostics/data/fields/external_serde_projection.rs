use std::collections::BTreeMap;
use std::fmt;

use forge_foundational::facade::{AspectMask, AspectValueLocator, DiagnosticMask};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::aspect_value_diagnostic_terms::{
    aspect_value_diagnostic_value, struct_aspect_value_diagnostic_value,
};
use super::{RelationalDiagnosticFields, RelationalDiagnosticValue};
use crate::identity::data::PartitionId;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExternalDiagnosticProjectionValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<ExternalDiagnosticProjectionValue>),
    Object(BTreeMap<String, ExternalDiagnosticProjectionValue>),
}

pub(super) fn serialize_diagnostic_fields<S>(
    fields: &RelationalDiagnosticFields,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    project_diagnostic_value(fields.root()).serialize(serializer)
}

pub(super) fn deserialize_diagnostic_fields<'de, D>(
    deserializer: D,
) -> Result<RelationalDiagnosticFields, D::Error>
where
    D: Deserializer<'de>,
{
    ExternalDiagnosticProjectionValue::deserialize(deserializer)
        .map(recover_projected_diagnostic_fields)
}

pub(super) fn diagnostic_projection_equal(
    left: &RelationalDiagnosticValue,
    right: &RelationalDiagnosticValue,
) -> bool {
    project_diagnostic_value(left) == project_diagnostic_value(right)
}

pub(super) fn typed_external_projection_value(
    value: &RelationalDiagnosticValue,
) -> RelationalDiagnosticValue {
    recover_projected_diagnostic_value(project_diagnostic_value(value))
}

fn recover_projected_diagnostic_fields(
    projection: ExternalDiagnosticProjectionValue,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticFields::from_diagnostic_value(recover_projected_diagnostic_value(
        projection,
    ))
}

fn recover_projected_diagnostic_value(
    projection: ExternalDiagnosticProjectionValue,
) -> RelationalDiagnosticValue {
    match projection {
        ExternalDiagnosticProjectionValue::Null => RelationalDiagnosticValue::Null,
        ExternalDiagnosticProjectionValue::Bool(value) => RelationalDiagnosticValue::Bool(value),
        ExternalDiagnosticProjectionValue::Unsigned(value) => {
            RelationalDiagnosticValue::Unsigned(value)
        }
        ExternalDiagnosticProjectionValue::Signed(value) => {
            RelationalDiagnosticValue::Signed(value)
        }
        ExternalDiagnosticProjectionValue::String(value) => {
            RelationalDiagnosticValue::String(value)
        }
        ExternalDiagnosticProjectionValue::Array(values) => RelationalDiagnosticValue::array(
            values.into_iter().map(recover_projected_diagnostic_value),
        ),
        ExternalDiagnosticProjectionValue::Object(fields) => RelationalDiagnosticValue::object(
            fields
                .into_iter()
                .map(|(key, value)| (key, recover_projected_diagnostic_value(value))),
        ),
    }
}

fn project_diagnostic_value(
    value: &RelationalDiagnosticValue,
) -> ExternalDiagnosticProjectionValue {
    match value {
        RelationalDiagnosticValue::Null => ExternalDiagnosticProjectionValue::Null,
        RelationalDiagnosticValue::Bool(value) => ExternalDiagnosticProjectionValue::Bool(*value),
        RelationalDiagnosticValue::Unsigned(value) => {
            ExternalDiagnosticProjectionValue::Unsigned(*value)
        }
        RelationalDiagnosticValue::Signed(value) => {
            ExternalDiagnosticProjectionValue::Signed(*value)
        }
        RelationalDiagnosticValue::String(value) => {
            ExternalDiagnosticProjectionValue::String(value.clone())
        }
        RelationalDiagnosticValue::Array(values) => ExternalDiagnosticProjectionValue::Array(
            values.iter().map(project_diagnostic_value).collect(),
        ),
        RelationalDiagnosticValue::Object(fields) => ExternalDiagnosticProjectionValue::Object(
            fields
                .iter()
                .map(|(key, value)| (key.clone(), project_diagnostic_value(value)))
                .collect(),
        ),
        RelationalDiagnosticValue::AspectKey(value) => {
            ExternalDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::FieldKey(value) => {
            ExternalDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::FieldPath(fields) => ExternalDiagnosticProjectionValue::Array(
            fields
                .iter()
                .map(|field| ExternalDiagnosticProjectionValue::String(field.as_str().to_string()))
                .collect(),
        ),
        RelationalDiagnosticValue::AspectValue(value) => {
            project_diagnostic_value(&aspect_value_diagnostic_value(value))
        }
        RelationalDiagnosticValue::AspectValueLocator(locator) => {
            aspect_value_locator_projection(locator)
        }
        RelationalDiagnosticValue::StructAspectValue(value) => {
            project_diagnostic_value(&struct_aspect_value_diagnostic_value(value))
        }
        RelationalDiagnosticValue::DiagnosticMask(mask) => diagnostic_mask_projection(mask),
        RelationalDiagnosticValue::PartitionId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::KindId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::VersionId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::LineageId(value) => unsigned(value.as_u64()),
        RelationalDiagnosticValue::CommitId(value) => unsigned(value.0),
        RelationalDiagnosticValue::BranchId(value) => {
            ExternalDiagnosticProjectionValue::String(value.0.clone())
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
            ExternalDiagnosticProjectionValue::String(value.0.clone())
        }
        RelationalDiagnosticValue::SchemaVersionId(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::ContractId(value) => {
            ExternalDiagnosticProjectionValue::String(value.as_str().to_string())
        }
        RelationalDiagnosticValue::SchemaBoundaryFingerprint(value) => {
            ExternalDiagnosticProjectionValue::String(format!("{value:?}"))
        }
        RelationalDiagnosticValue::DescriptorSemanticsVersion(value) => unsigned(value.0 as u64),
        RelationalDiagnosticValue::DescriptorCanonicalBasisVersion(value) => {
            unsigned(value.0 as u64)
        }
        RelationalDiagnosticValue::EntityId(value) => record_id_projection(
            "entity",
            value.partition_id,
            value.local_slot_value(),
            value.generation_value(),
        ),
        RelationalDiagnosticValue::RelationId(value) => record_id_projection(
            "relation",
            value.partition_id,
            value.local_slot_value(),
            value.generation_value(),
        ),
    }
}

fn aspect_value_locator_projection(
    locator: &AspectValueLocator,
) -> ExternalDiagnosticProjectionValue {
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
                ExternalDiagnosticProjectionValue::Array(
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

fn diagnostic_mask_projection(
    mask: &AspectMask<DiagnosticMask>,
) -> ExternalDiagnosticProjectionValue {
    if mask.is_whole_aspect() {
        return object([("mask_kind", string("whole_aspect"))]);
    }

    object([
        ("mask_kind", string("fields")),
        (
            "field_paths",
            ExternalDiagnosticProjectionValue::Array(
                mask.paths()
                    .iter()
                    .map(|field_path| {
                        ExternalDiagnosticProjectionValue::Array(
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

fn record_id_projection(
    record_kind: &'static str,
    partition_id: PartitionId,
    local_slot: u64,
    generation: u32,
) -> ExternalDiagnosticProjectionValue {
    object([
        ("record_kind", string(record_kind)),
        (
            "partition_id",
            ExternalDiagnosticProjectionValue::Unsigned(partition_id.as_u64()),
        ),
        (
            "local_slot",
            ExternalDiagnosticProjectionValue::Unsigned(local_slot),
        ),
        (
            "generation",
            ExternalDiagnosticProjectionValue::Unsigned(generation as u64),
        ),
    ])
}

fn object(
    fields: impl IntoIterator<Item = (&'static str, ExternalDiagnosticProjectionValue)>,
) -> ExternalDiagnosticProjectionValue {
    ExternalDiagnosticProjectionValue::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
    )
}

fn string(value: impl Into<String>) -> ExternalDiagnosticProjectionValue {
    ExternalDiagnosticProjectionValue::String(value.into())
}

fn unsigned(value: u64) -> ExternalDiagnosticProjectionValue {
    ExternalDiagnosticProjectionValue::Unsigned(value)
}

impl Serialize for ExternalDiagnosticProjectionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Unsigned(value) => serializer.serialize_u64(*value),
            Self::Signed(value) => serializer.serialize_i64(*value),
            Self::String(value) => serializer.serialize_str(value),
            Self::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            Self::Object(fields) => {
                let mut map = serializer.serialize_map(Some(fields.len()))?;
                for (key, value) in fields {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ExternalDiagnosticProjectionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ExternalDiagnosticProjectionVisitor)
    }
}

struct ExternalDiagnosticProjectionVisitor;

impl<'de> Visitor<'de> for ExternalDiagnosticProjectionVisitor {
    type Value = ExternalDiagnosticProjectionValue;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a diagnostic serde projection value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::Signed(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::Unsigned(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::String(value.to_string()))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ExternalDiagnosticProjectionValue::String(value.to_string()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(ExternalDiagnosticProjectionValue::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element()? {
            values.push(value);
        }
        Ok(ExternalDiagnosticProjectionValue::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut fields = BTreeMap::new();
        while let Some((key, value)) = map.next_entry()? {
            fields.insert(key, value);
        }
        Ok(ExternalDiagnosticProjectionValue::Object(fields))
    }
}
