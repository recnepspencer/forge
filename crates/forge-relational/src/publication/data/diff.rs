use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::history::data::CommitId;
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
    pub aspects: Vec<AspectKey>,
    pub detail: PatchDetail,
}

impl PatchRecord {
    pub fn canonicalized(&self) -> Self {
        let mut aspects = self.aspects.clone();
        aspects.sort_by(|left, right| format!("{left:?}").cmp(&format!("{right:?}")));
        aspects.dedup();
        Self {
            kind: self.kind.clone(),
            entity_id: self.entity_id,
            relation_id: self.relation_id,
            aspects,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStreamRequest {
    pub after_position: Option<PatchStreamPosition>,
    pub max_commits: usize,
}

impl Default for PatchStreamRequest {
    fn default() -> Self {
        Self {
            after_position: None,
            max_commits: usize::MAX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatchStreamReadErrorClass {
    UnknownResumePosition,
    InvalidBatchSize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStreamReadError {
    pub class: PatchStreamReadErrorClass,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchStreamBatch {
    pub patches: Vec<RelationalPatchRecord>,
    pub resumed_after: Option<PatchStreamPosition>,
    pub next_position: Option<PatchStreamPosition>,
    pub latest_position: Option<PatchStreamPosition>,
    pub latest_commit_id: Option<CommitId>,
}
