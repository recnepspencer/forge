use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::identity::data::{EntityId, RelationId};
use crate::payloads::data::{canonicalize_json, RecordPayload};
use crate::symbols::data::InternedString;

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

impl PatchDetail {
    pub fn canonicalized(&self) -> Self {
        match self {
            Self::StructuredJson(value) => Self::StructuredJson(canonicalize_json(value)),
            Self::Payload(payload) => Self::Payload(payload.canonicalized()),
            Self::DenseBitset(bits) => Self::DenseBitset(bits.clone()),
        }
    }
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

impl PatchRecord {
    pub fn canonicalized(&self) -> Self {
        Self {
            kind: self.kind.clone(),
            entity_id: self.entity_id,
            relation_id: self.relation_id,
            detail: self.detail.canonicalized(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalPatchRecord {
    pub ordering: PatchOrdering,
    pub publication_mode: PatchPublicationMode,
    pub position: PatchStreamPosition,
    pub compatibility: PatchCompatibilityClass,
    pub records: Vec<PatchRecord>,
}

impl RelationalPatchRecord {
    pub fn canonicalized(&self) -> Self {
        Self {
            ordering: self.ordering,
            publication_mode: self.publication_mode,
            position: self.position,
            compatibility: self.compatibility,
            records: self
                .records
                .iter()
                .map(PatchRecord::canonicalized)
                .collect(),
        }
    }
}
