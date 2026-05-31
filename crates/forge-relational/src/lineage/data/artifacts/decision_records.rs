use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::data::{CorrespondenceCandidateId, CorrespondencePromotionRejectionClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageDecisionKind {
    CreateAccepted,
    ReplaceAccepted,
    RetireAccepted,
    CorrespondencePromotionAccepted,
    CorrespondencePromotionRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDecisionRecord {
    pub(crate) branch_id: BranchId,
    pub(crate) kind: LineageDecisionKind,
    pub(crate) event_id: Option<u64>,
    pub(crate) candidate_id: Option<CorrespondenceCandidateId>,
    pub(crate) sources: Vec<LineageId>,
    pub(crate) targets: Vec<LineageId>,
    pub(crate) rejection_class: Option<CorrespondencePromotionRejectionClass>,
}

impl LineageDecisionRecord {
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn kind(&self) -> &LineageDecisionKind {
        &self.kind
    }

    pub fn event_id(&self) -> Option<u64> {
        self.event_id
    }

    pub fn candidate_id(&self) -> Option<CorrespondenceCandidateId> {
        self.candidate_id
    }

    pub fn sources(&self) -> &[LineageId] {
        &self.sources
    }

    pub fn targets(&self) -> &[LineageId] {
        &self.targets
    }

    pub fn rejection_class(&self) -> Option<CorrespondencePromotionRejectionClass> {
        self.rejection_class
    }
}
