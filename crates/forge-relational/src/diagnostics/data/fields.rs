use std::ops::Index;

use forge_foundational::facade::{
    AspectKey, AspectMask, AspectValue, AspectValueLocator, DiagnosticMask, FieldKey,
    StructAspectValue,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;

use crate::durability::data::{DurableCheckpointId, DurableSegmentId};
use crate::history::data::BranchId;
use crate::history::data::CommitId;
use crate::identity::data::{EntityId, KindId, LineageId, PartitionId, RelationId, VersionId};
use crate::indexes::data::{DerivedIndexGenerationId, DerivedIndexId};
use crate::lineage::data::CorrespondenceCandidateId;
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::schema::data::{
    ContractId, DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
    SchemaBoundaryFingerprint, SchemaId, SchemaVersionId,
};
use crate::snapshots::data::SnapshotId;

mod aspect_value_projection;

use aspect_value_projection::{aspect_value_json_value, struct_aspect_value_json_value};

#[derive(Debug, Clone)]
pub struct RelationalDiagnosticFields {
    root: RelationalDiagnosticValue,
    projected_root: Value,
}

impl RelationalDiagnosticFields {
    fn from_projected_json(root: Value) -> Self {
        let projected_root = canonicalize_diagnostic_value(&root);
        let root = diagnostic_value_from_projected_json(&projected_root);
        Self {
            root,
            projected_root,
        }
    }

    pub fn from_diagnostic_value(root: RelationalDiagnosticValue) -> Self {
        let projected_root = root.to_json_value();
        Self {
            root,
            projected_root,
        }
    }

    pub fn root_value(&self) -> &Value {
        &self.projected_root
    }

    pub fn root(&self) -> &RelationalDiagnosticValue {
        &self.root
    }

    pub fn get(&self, field_name: &str) -> Option<&Value> {
        self.projected_root.get(field_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalDiagnosticValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    Array(Vec<RelationalDiagnosticValue>),
    Object(BTreeMap<String, RelationalDiagnosticValue>),
    AspectKey(AspectKey),
    FieldKey(FieldKey),
    FieldPath(Vec<FieldKey>),
    AspectValue(AspectValue),
    AspectValueLocator(AspectValueLocator),
    StructAspectValue(StructAspectValue),
    DiagnosticMask(AspectMask<DiagnosticMask>),
    PartitionId(PartitionId),
    KindId(KindId),
    VersionId(VersionId),
    LineageId(LineageId),
    CommitId(CommitId),
    BranchId(BranchId),
    SnapshotId(SnapshotId),
    DurableCheckpointId(DurableCheckpointId),
    DurableSegmentId(DurableSegmentId),
    DerivedIndexId(DerivedIndexId),
    DerivedIndexGenerationId(DerivedIndexGenerationId),
    CorrespondenceCandidateId(CorrespondenceCandidateId),
    PatchStreamPosition(PatchStreamPosition),
    ReplaySchemaVersion(ReplaySchemaVersion),
    SchemaId(SchemaId),
    SchemaVersionId(SchemaVersionId),
    ContractId(ContractId),
    SchemaBoundaryFingerprint(SchemaBoundaryFingerprint),
    DescriptorSemanticsVersion(DescriptorSemanticsVersion),
    DescriptorCanonicalizationVersion(DescriptorCanonicalizationVersion),
    EntityId(EntityId),
    RelationId(RelationId),
}

impl RelationalDiagnosticValue {
    pub fn object(
        fields: impl IntoIterator<Item = (impl Into<String>, RelationalDiagnosticValue)>,
    ) -> Self {
        Self::Object(
            fields
                .into_iter()
                .map(|(key, value)| (key.into(), value))
                .collect(),
        )
    }

    pub fn array(values: impl IntoIterator<Item = RelationalDiagnosticValue>) -> Self {
        Self::Array(values.into_iter().collect())
    }

    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    pub fn unsigned(value: usize) -> Self {
        Self::Unsigned(value as u64)
    }

    pub fn optional(value: Option<RelationalDiagnosticValue>) -> Self {
        value.unwrap_or(Self::Null)
    }

    fn to_json_value(&self) -> Value {
        match self {
            Self::Null => Value::Null,
            Self::Bool(value) => Value::Bool(*value),
            Self::Unsigned(value) => Value::from(*value),
            Self::Signed(value) => Value::from(*value),
            Self::String(value) => Value::String(value.clone()),
            Self::Array(values) => Value::Array(
                values
                    .iter()
                    .map(RelationalDiagnosticValue::to_json_value)
                    .collect(),
            ),
            Self::Object(fields) => Value::Object(
                fields
                    .iter()
                    .map(|(key, value)| (key.clone(), value.to_json_value()))
                    .collect(),
            ),
            Self::AspectKey(value) => Value::String(value.as_str().to_string()),
            Self::FieldKey(value) => Value::String(value.as_str().to_string()),
            Self::FieldPath(fields) => Value::Array(
                fields
                    .iter()
                    .map(|field| Value::String(field.as_str().to_string()))
                    .collect(),
            ),
            Self::AspectValue(value) => aspect_value_json_value(value),
            Self::AspectValueLocator(locator) => aspect_value_locator_json_value(locator),
            Self::StructAspectValue(value) => struct_aspect_value_json_value(value),
            Self::DiagnosticMask(mask) => diagnostic_mask_json_value(mask),
            Self::PartitionId(value) => Value::from(value.as_u64()),
            Self::KindId(value) => Value::from(value.as_u64()),
            Self::VersionId(value) => Value::from(value.as_u64()),
            Self::LineageId(value) => Value::from(value.as_u64()),
            Self::CommitId(value) => Value::from(value.0),
            Self::BranchId(value) => Value::String(value.0.clone()),
            Self::SnapshotId(value) => Value::from(value.0),
            Self::DurableCheckpointId(value) => Value::from(value.0),
            Self::DurableSegmentId(value) => Value::from(value.0),
            Self::DerivedIndexId(value) => Value::from(value.0),
            Self::DerivedIndexGenerationId(value) => Value::from(value.0),
            Self::CorrespondenceCandidateId(value) => Value::from(value.0),
            Self::PatchStreamPosition(value) => Value::from(value.0),
            Self::ReplaySchemaVersion(value) => Value::from(value.0),
            Self::SchemaId(value) => Value::String(value.0.clone()),
            Self::SchemaVersionId(value) => Value::from(value.0),
            Self::ContractId(value) => Value::String(value.as_str().to_string()),
            Self::SchemaBoundaryFingerprint(value) => Value::String(format!("{value:?}")),
            Self::DescriptorSemanticsVersion(value) => Value::from(value.0),
            Self::DescriptorCanonicalizationVersion(value) => Value::from(value.0),
            Self::EntityId(value) => entity_id_json_value(*value),
            Self::RelationId(value) => relation_id_json_value(*value),
        }
    }
}

fn aspect_value_locator_json_value(locator: &AspectValueLocator) -> Value {
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

impl Serialize for RelationalDiagnosticFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.projected_root.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RelationalDiagnosticFields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Value::deserialize(deserializer).map(Self::from_projected_json)
    }
}

impl From<RelationalDiagnosticValue> for RelationalDiagnosticFields {
    fn from(root: RelationalDiagnosticValue) -> Self {
        Self::from_diagnostic_value(root)
    }
}

impl From<RelationalDiagnosticFields> for Value {
    fn from(fields: RelationalDiagnosticFields) -> Self {
        fields.projected_root
    }
}

impl PartialEq for RelationalDiagnosticFields {
    fn eq(&self, other: &Self) -> bool {
        self.projected_root == other.projected_root
    }
}

impl Eq for RelationalDiagnosticFields {}

impl PartialEq<Value> for RelationalDiagnosticFields {
    fn eq(&self, other: &Value) -> bool {
        &self.projected_root == other
    }
}

impl PartialEq<RelationalDiagnosticFields> for Value {
    fn eq(&self, other: &RelationalDiagnosticFields) -> bool {
        self == &other.projected_root
    }
}

impl Index<&str> for RelationalDiagnosticFields {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.projected_root[index]
    }
}

fn diagnostic_mask_json_value(mask: &AspectMask<DiagnosticMask>) -> Value {
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

fn entity_id_json_value(entity_id: EntityId) -> Value {
    record_id_json_value(
        "entity",
        entity_id.partition_id,
        entity_id.local_slot_value(),
        entity_id.generation_value(),
    )
}

fn relation_id_json_value(relation_id: RelationId) -> Value {
    record_id_json_value(
        "relation",
        relation_id.partition_id,
        relation_id.local_slot_value(),
        relation_id.generation_value(),
    )
}

fn record_id_json_value(
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

fn diagnostic_value_from_projected_json(value: &Value) -> RelationalDiagnosticValue {
    match value {
        Value::Null => RelationalDiagnosticValue::Null,
        Value::Bool(boolean) => RelationalDiagnosticValue::Bool(*boolean),
        Value::Number(number) => diagnostic_number_value(number),
        Value::String(text) => RelationalDiagnosticValue::String(text.clone()),
        Value::Array(values) => RelationalDiagnosticValue::array(
            values.iter().map(diagnostic_value_from_projected_json),
        ),
        Value::Object(entries) => RelationalDiagnosticValue::object(
            entries
                .iter()
                .map(|(key, value)| (key.clone(), diagnostic_value_from_projected_json(value))),
        ),
    }
}

fn diagnostic_number_value(number: &serde_json::Number) -> RelationalDiagnosticValue {
    if let Some(value) = number.as_u64() {
        return RelationalDiagnosticValue::Unsigned(value);
    }
    if let Some(value) = number.as_i64() {
        return RelationalDiagnosticValue::Signed(value);
    }
    RelationalDiagnosticValue::String(number.to_string())
}

fn canonicalize_diagnostic_value(value: &Value) -> Value {
    match value {
        Value::Null => Value::Null,
        Value::Bool(boolean) => Value::Bool(*boolean),
        Value::Number(number) => Value::Number(number.clone()),
        Value::String(text) => Value::String(text.clone()),
        Value::Array(values) => {
            Value::Array(values.iter().map(canonicalize_diagnostic_value).collect())
        }
        Value::Object(entries) => {
            let mut ordered = Map::new();
            let mut keys = entries.keys().cloned().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                let value = entries
                    .get(&key)
                    .expect("object key collected from map must exist");
                ordered.insert(key, canonicalize_diagnostic_value(value));
            }
            Value::Object(ordered)
        }
    }
}

#[cfg(test)]
#[path = "fields/fields_tests.rs"]
mod fields_tests;
