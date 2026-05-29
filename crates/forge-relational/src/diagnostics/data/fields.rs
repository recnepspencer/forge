use forge_foundational::facade::{
    AspectKey, AspectMask, AspectValue, AspectValueLocator, DiagnosticMask, FieldKey,
    StructAspectValue,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

mod aspect_value_diagnostic_terms;
mod json_projection;
mod projected_json_recovery;

use json_projection::diagnostic_value_to_json;
use projected_json_recovery::{
    canonicalize_diagnostic_value, diagnostic_value_from_projected_json,
};

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
        let projected_root = diagnostic_value_to_json(&root);
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

    pub fn into_projected_json(self) -> Value {
        self.projected_root
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

impl PartialEq for RelationalDiagnosticFields {
    fn eq(&self, other: &Self) -> bool {
        self.projected_root == other.projected_root
    }
}

impl Eq for RelationalDiagnosticFields {}

#[cfg(test)]
#[path = "fields/fields_tests.rs"]
mod fields_tests;
