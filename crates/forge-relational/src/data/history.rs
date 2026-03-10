use serde::{Deserialize, Serialize};

use crate::data::identity::VersionId;
use crate::data::identity::{EntityId, RelationId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CommitId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BranchId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionGraphPolicy {
    CanonicalSerializedPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryRetentionClass {
    Ephemeral,
    Durable,
    AuditGrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitReference {
    pub commit_id: CommitId,
    pub version_id: VersionId,
    pub branch_id: BranchId,
    pub parents: Vec<CommitId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionNode {
    pub commit: CommitReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchHead {
    pub branch_id: BranchId,
    pub head: Option<CommitReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchCreateError {
    BranchAlreadyExists,
    SourceBranchMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionGraphSnapshot {
    pub branches: Vec<BranchHead>,
    pub commits: Vec<VersionNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MergeConflictRecord {
    Entity(EntityId),
    Relation(RelationId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeInspection {
    pub source_branch: BranchId,
    pub target_branch: BranchId,
    pub source_head: Option<CommitReference>,
    pub target_head: Option<CommitReference>,
    pub merge_base: Option<CommitId>,
    pub source_only_commits: Vec<CommitId>,
    pub target_only_commits: Vec<CommitId>,
    pub conflicting_records: Vec<MergeConflictRecord>,
    pub can_merge: bool,
}
