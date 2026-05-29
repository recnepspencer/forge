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
mod serde_projection;
mod serde_recovery;

use serde_projection::diagnostic_value_to_serde_value;
use serde_recovery::{canonicalize_serde_value, diagnostic_value_from_serde_value};

#[derive(Debug, Clone)]
pub struct RelationalDiagnosticFields {
    root: RelationalDiagnosticValue,
}

impl RelationalDiagnosticFields {
    fn from_serde_projection(root: Value) -> Self {
        let canonical_serde_projection = canonicalize_serde_value(&root);
        let root = diagnostic_value_from_serde_value(&canonical_serde_projection);
        Self { root }
    }

    pub fn from_diagnostic_value(root: RelationalDiagnosticValue) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &RelationalDiagnosticValue {
        &self.root
    }

    pub fn into_serde_projection(self) -> Value {
        diagnostic_value_to_serde_value(&self.root)
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
        diagnostic_value_to_serde_value(&self.root).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RelationalDiagnosticFields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Value::deserialize(deserializer).map(Self::from_serde_projection)
    }
}

impl From<RelationalDiagnosticValue> for RelationalDiagnosticFields {
    fn from(root: RelationalDiagnosticValue) -> Self {
        Self::from_diagnostic_value(root)
    }
}

impl PartialEq for RelationalDiagnosticFields {
    fn eq(&self, other: &Self) -> bool {
        diagnostic_value_to_serde_value(&self.root) == diagnostic_value_to_serde_value(&other.root)
    }
}

impl Eq for RelationalDiagnosticFields {}

#[cfg(test)]
#[path = "fields/fields_tests.rs"]
mod fields_tests;
