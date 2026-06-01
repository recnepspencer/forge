use serde::{Deserialize, Serialize};

use crate::lineage::data::CorrespondencePromotionRejectionClass;

use super::{LineageDecisionKind, LineageDecisionRecord};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct LineageDecisionLog {
    decisions: Vec<LineageDecisionRecord>,
}

impl LineageDecisionLog {
    #[cfg(test)]
    pub(crate) fn single(decision: LineageDecisionRecord) -> Self {
        Self::new(vec![decision])
    }

    pub(crate) fn new(mut decisions: Vec<LineageDecisionRecord>) -> Self {
        decisions.sort_by(canonical_decision_cmp);
        Self { decisions }
    }

    pub(crate) fn decisions(&self) -> &[LineageDecisionRecord] {
        &self.decisions
    }
}

fn canonical_decision_cmp(
    left: &LineageDecisionRecord,
    right: &LineageDecisionRecord,
) -> std::cmp::Ordering {
    left.branch_id()
        .cmp(right.branch_id())
        .then_with(|| {
            left.event_id()
                .unwrap_or(u64::MAX)
                .cmp(&right.event_id().unwrap_or(u64::MAX))
        })
        .then_with(|| {
            left.candidate_id()
                .map(|id| id.0)
                .unwrap_or(u64::MAX)
                .cmp(&right.candidate_id().map(|id| id.0).unwrap_or(u64::MAX))
        })
        .then_with(|| {
            canonical_decision_kind_rank(left.kind().clone())
                .cmp(&canonical_decision_kind_rank(right.kind().clone()))
        })
        .then_with(|| {
            left.rejection_class()
                .map(canonical_rejection_class_rank)
                .unwrap_or(u8::MAX)
                .cmp(
                    &right
                        .rejection_class()
                        .map(canonical_rejection_class_rank)
                        .unwrap_or(u8::MAX),
                )
        })
        .then_with(|| left.sources().cmp(right.sources()))
        .then_with(|| left.targets().cmp(right.targets()))
}

fn canonical_decision_kind_rank(kind: LineageDecisionKind) -> u8 {
    match kind {
        LineageDecisionKind::CreateAccepted => 0,
        LineageDecisionKind::ReplaceAccepted => 1,
        LineageDecisionKind::RetireAccepted => 2,
        LineageDecisionKind::CorrespondencePromotionAccepted => 3,
        LineageDecisionKind::CorrespondencePromotionRejected => 4,
    }
}

fn canonical_rejection_class_rank(class: CorrespondencePromotionRejectionClass) -> u8 {
    match class {
        CorrespondencePromotionRejectionClass::CandidateMissing => 0,
        CorrespondencePromotionRejectionClass::MissingLineageReference => 1,
        CorrespondencePromotionRejectionClass::EmptyEndpointSet => 2,
        CorrespondencePromotionRejectionClass::DuplicateEndpointReference => 3,
        CorrespondencePromotionRejectionClass::OverlappingSourceAndTarget => 4,
        CorrespondencePromotionRejectionClass::CommitBranchMismatch => 5,
        CorrespondencePromotionRejectionClass::BranchScopeMismatch => 6,
        CorrespondencePromotionRejectionClass::CommitNotBranchHead => 7,
        CorrespondencePromotionRejectionClass::AuthorityPublicationFailed => 8,
    }
}
