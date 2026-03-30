use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::schema::data::SchemaVersionId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DerivedIndexId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DerivedIndexGenerationId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexKind {
    EntityPayloadField { field: String },
    RelationPayloadField { field: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexDefinition {
    pub index_id: DerivedIndexId,
    pub name: String,
    pub kind: DerivedIndexKind,
    pub branch_scoped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexPayload {
    EntityField(BTreeMap<String, Vec<EntityId>>),
    RelationField(BTreeMap<String, Vec<RelationId>>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedIndexPublicationStatus {
    Published,
    BuildFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexCompatibility {
    pub branch_id: BranchId,
    pub version_id: VersionId,
    pub schema_version: SchemaVersionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexGeneration {
    pub generation_id: DerivedIndexGenerationId,
    pub index_id: DerivedIndexId,
    pub source_commit_id: CommitId,
    pub source_branch_id: BranchId,
    pub compatibility: DerivedIndexCompatibility,
    pub status: DerivedIndexPublicationStatus,
    pub payload: DerivedIndexPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexBuildRequest {
    pub source_commit_id: CommitId,
    pub branch_id: BranchId,
    pub index_ids: Vec<DerivedIndexId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedIndexBuildOutcome {
    pub source_commit_id: CommitId,
    pub generations: Vec<DerivedIndexGeneration>,
    pub failed_indexes: Vec<DerivedIndexId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadWithStorageFallbackOutcome {
    pub result: crate::storage::data::PacketResult,
    pub used_index_generation: Option<DerivedIndexGenerationId>,
}
