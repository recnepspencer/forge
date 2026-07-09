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
    ExecutionFailed,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrespondencePromotionExecutionFailureClass {
    AnchorDriftedFromBranchHead,
    AuthorityPublicationFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum CorrespondenceResolutionOutcome {
    Promoted {
        promoted_event_id: u64,
        promoted_commit_id: CommitId,
    },
    Rejected {
        rejection_class: CorrespondencePromotionRejectionClass,
    },
    ExecutionFailed {
        promoted_event_id: u64,
        execution_failure_class: CorrespondencePromotionExecutionFailureClass,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceCandidate {
    pub(crate) candidate_id: CorrespondenceCandidateId,
    pub(crate) branch_id: BranchId,
    pub(crate) sources: Vec<LineageId>,
    pub(crate) targets: Vec<LineageId>,
    pub(crate) note: String,
}

impl CorrespondenceCandidate {
    pub fn candidate_id(&self) -> CorrespondenceCandidateId {
        self.candidate_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn sources(&self) -> &[LineageId] {
        &self.sources
    }

    pub fn targets(&self) -> &[LineageId] {
        &self.targets
    }

    pub fn note(&self) -> &str {
        &self.note
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondenceResolution {
    candidate_id: CorrespondenceCandidateId,
    outcome: CorrespondenceResolutionOutcome,
}

impl CorrespondenceResolution {
    #[cfg(test)]
    pub(crate) fn promoted(
        candidate_id: CorrespondenceCandidateId,
        promoted_event_id: u64,
        promoted_commit_id: CommitId,
    ) -> Self {
        Self {
            candidate_id,
            outcome: CorrespondenceResolutionOutcome::Promoted {
                promoted_event_id,
                promoted_commit_id,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn rejected(
        candidate_id: CorrespondenceCandidateId,
        rejection_class: CorrespondencePromotionRejectionClass,
    ) -> Self {
        Self {
            candidate_id,
            outcome: CorrespondenceResolutionOutcome::Rejected { rejection_class },
        }
    }

    #[cfg(test)]
    pub(crate) fn execution_failed(
        candidate_id: CorrespondenceCandidateId,
        promoted_event_id: u64,
        execution_failure_class: CorrespondencePromotionExecutionFailureClass,
    ) -> Self {
        Self {
            candidate_id,
            outcome: CorrespondenceResolutionOutcome::ExecutionFailed {
                promoted_event_id,
                execution_failure_class,
            },
        }
    }

    pub fn candidate_id(&self) -> CorrespondenceCandidateId {
        self.candidate_id
    }

    pub fn status(&self) -> LineageResolutionStatus {
        match self.outcome {
            CorrespondenceResolutionOutcome::Promoted { .. } => LineageResolutionStatus::Promoted,
            CorrespondenceResolutionOutcome::Rejected { .. } => LineageResolutionStatus::Rejected,
            CorrespondenceResolutionOutcome::ExecutionFailed { .. } => {
                LineageResolutionStatus::ExecutionFailed
            }
        }
    }

    pub fn promoted_event_id(&self) -> Option<u64> {
        match self.outcome {
            CorrespondenceResolutionOutcome::Promoted {
                promoted_event_id, ..
            }
            | CorrespondenceResolutionOutcome::ExecutionFailed {
                promoted_event_id, ..
            } => Some(promoted_event_id),
            CorrespondenceResolutionOutcome::Rejected { .. } => None,
        }
    }

    pub fn promoted_commit_id(&self) -> Option<CommitId> {
        match self.outcome {
            CorrespondenceResolutionOutcome::Promoted {
                promoted_commit_id, ..
            } => Some(promoted_commit_id),
            CorrespondenceResolutionOutcome::Rejected { .. }
            | CorrespondenceResolutionOutcome::ExecutionFailed { .. } => None,
        }
    }

    pub fn rejection_class(&self) -> Option<CorrespondencePromotionRejectionClass> {
        match self.outcome {
            CorrespondenceResolutionOutcome::Rejected { rejection_class } => Some(rejection_class),
            CorrespondenceResolutionOutcome::Promoted { .. }
            | CorrespondenceResolutionOutcome::ExecutionFailed { .. } => None,
        }
    }

    pub fn execution_failure_class(&self) -> Option<CorrespondencePromotionExecutionFailureClass> {
        match self.outcome {
            CorrespondenceResolutionOutcome::ExecutionFailed {
                execution_failure_class,
                ..
            } => Some(execution_failure_class),
            CorrespondenceResolutionOutcome::Promoted { .. }
            | CorrespondenceResolutionOutcome::Rejected { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorrespondencePromotionOutcome(CorrespondenceResolution);

impl CorrespondencePromotionOutcome {
    #[cfg(test)]
    pub(crate) fn rejected(
        candidate_id: CorrespondenceCandidateId,
        rejection_class: CorrespondencePromotionRejectionClass,
    ) -> Self {
        Self(CorrespondenceResolution::rejected(
            candidate_id,
            rejection_class,
        ))
    }

    pub fn as_resolution(&self) -> &CorrespondenceResolution {
        &self.0
    }

    pub fn candidate_id(&self) -> CorrespondenceCandidateId {
        self.0.candidate_id()
    }

    pub fn status(&self) -> LineageResolutionStatus {
        self.0.status()
    }

    pub fn promoted_event_id(&self) -> Option<u64> {
        self.0.promoted_event_id()
    }

    pub fn promoted_commit_id(&self) -> Option<CommitId> {
        self.0.promoted_commit_id()
    }

    pub fn rejection_class(&self) -> Option<CorrespondencePromotionRejectionClass> {
        self.0.rejection_class()
    }

    pub fn execution_failure_class(&self) -> Option<CorrespondencePromotionExecutionFailureClass> {
        self.0.execution_failure_class()
    }
}

impl From<CorrespondenceResolution> for CorrespondencePromotionOutcome {
    fn from(value: CorrespondenceResolution) -> Self {
        Self(value)
    }
}
