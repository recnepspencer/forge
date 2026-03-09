use serde::{Deserialize, Serialize};

use crate::data::identity::VersionId;

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
}
