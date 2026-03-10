use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::identity::{EntityId, RelationId};
use crate::data::payload::RecordPayload;
use crate::data::symbols::InternedString;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectKey(pub InternedString);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchOrdering {
    CanonicalCommitOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchPublicationMode {
    CommitNative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PatchStreamPosition(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchFragmentBudget {
    pub worker_local_fragments: bool,
    pub deterministic_merge_required: bool,
}

impl Default for PatchFragmentBudget {
    fn default() -> Self {
        Self {
            worker_local_fragments: true,
            deterministic_merge_required: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchCompatibilityClass {
    StructuredCompatible,
    DenseCompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchDetail {
    StructuredJson(Value),
    Payload(RecordPayload),
    DenseBitset(Vec<u64>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchRecordKind {
    EntityCreated,
    EntityUpdated,
    EntityDeleted,
    RelationCreated,
    RelationDeleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchRecord {
    pub kind: PatchRecordKind,
    pub entity_id: Option<EntityId>,
    pub relation_id: Option<RelationId>,
    pub detail: PatchDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalPatchRecord {
    pub ordering: PatchOrdering,
    pub publication_mode: PatchPublicationMode,
    pub position: PatchStreamPosition,
    pub compatibility: PatchCompatibilityClass,
    pub records: Vec<PatchRecord>,
}
