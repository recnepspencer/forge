use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::LineageId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CorrespondenceCandidateId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageResolutionStatus {
    Advisory,
    Promoted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrespondencePromotionRejectionClass {
    CandidateMissing,
    MissingLineageReference,
    EmptyEndpointSet,
    DuplicateEndpointReference,
    OverlappingSourceAndTarget,
    CommitBranchMismatch,
    BranchScopeMismatch,
    CommitNotBranchHead,
    AuthorityPublicationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceCandidate {
    pub candidate_id: CorrespondenceCandidateId,
    pub branch_id: BranchId,
    pub sources: Vec<LineageId>,
    pub targets: Vec<LineageId>,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceResolution {
    pub candidate_id: CorrespondenceCandidateId,
    pub status: LineageResolutionStatus,
    pub promoted_event_id: Option<u64>,
    pub promoted_commit_id: Option<CommitId>,
    pub rejection_class: Option<CorrespondencePromotionRejectionClass>,
}

pub type CorrespondencePromotionOutcome = CorrespondenceResolution;
