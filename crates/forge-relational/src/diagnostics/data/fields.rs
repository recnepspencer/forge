use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectMask, AspectMaskLocator, AspectValue, AspectValueLocator,
    CanonicalBasisReadyArtifact, CanonicalFieldPath, DiagnosticMask, FieldKey, StructAspectValue,
};
use serde::{Deserialize, Serialize};
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
    ContractId, DescriptorCanonicalBasisVersion, DescriptorSemanticsVersion,
    SchemaBoundaryFingerprint, SchemaId, SchemaVersionId,
};
use crate::snapshots::data::SnapshotId;

mod aspect_value_diagnostic_terms;
mod external_serde_projection;

use external_serde_projection::{
    serialize_diagnostic_fields, typed_external_serde_projection_tree,
};

#[derive(Debug, Clone)]
pub struct RelationalDiagnosticFields {
    root: RelationalDiagnosticValue,
}

impl RelationalDiagnosticFields {
    pub fn from_diagnostic_value(root: RelationalDiagnosticValue) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &RelationalDiagnosticValue {
        &self.root
    }

    pub fn to_external_serde_projection_tree(&self) -> RelationalDiagnosticValue {
        typed_external_serde_projection_tree(self.root())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalDiagnosticValue {
    Null,
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    String(String),
    CanonicalBytes(Vec<u8>),
    Array(Vec<RelationalDiagnosticValue>),
    Object(BTreeMap<String, RelationalDiagnosticValue>),
    AspectKey(AspectKey),
    FieldKey(FieldKey),
    FieldPath(CanonicalFieldPath),
    AspectValue(AspectValue),
    AspectFieldLocator(AspectFieldLocator),
    AspectValueLocator(AspectValueLocator),
    StructAspectValue(StructAspectValue),
    DiagnosticMask(AspectMask<DiagnosticMask>),
    DiagnosticMaskLocator(AspectMaskLocator<DiagnosticMask>),
    CanonicalBasis(CanonicalBasisReadyArtifact),
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
    DescriptorCanonicalBasisVersion(DescriptorCanonicalBasisVersion),
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
        serialize_diagnostic_fields(self, serializer)
    }
}

impl<'de> Deserialize<'de> for RelationalDiagnosticFields {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Err(serde::de::Error::custom(
            "relational diagnostic fields are typed authority and cannot be recovered from external serde projection",
        ))
    }
}

impl From<RelationalDiagnosticValue> for RelationalDiagnosticFields {
    fn from(root: RelationalDiagnosticValue) -> Self {
        Self::from_diagnostic_value(root)
    }
}

impl PartialEq for RelationalDiagnosticFields {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for RelationalDiagnosticFields {}

#[cfg(test)]
#[path = "fields/fields_tests.rs"]
mod fields_tests;
