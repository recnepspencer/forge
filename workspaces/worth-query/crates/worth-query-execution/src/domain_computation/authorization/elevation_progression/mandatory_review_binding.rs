use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_relational::facade::identity::{EntityId, KindId};

use crate::domain_computation::primary_graph::WorthQueryMandatoryReview;

pub(in crate::domain_computation) struct WorthQueryMandatoryReviewDraft {
    pub(in crate::domain_computation) elevation: EntityId,
    pub(in crate::domain_computation) review: EntityId,
    pub(in crate::domain_computation) reviewer: EntityId,
    pub(in crate::domain_computation) reviewed_at: AspectValue,
    pub(in crate::domain_computation) terminal_status: AspectValue,
    pub(in crate::domain_computation) completed_status: AspectValue,
    pub(in crate::domain_computation) review_entity: String,
    pub(in crate::domain_computation) review_status_field: AspectFieldLocator,
    pub(in crate::domain_computation) approver_relation: KindId,
    pub(in crate::domain_computation) reviewer_relation: KindId,
    pub(in crate::domain_computation) required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
}

pub(in crate::domain_computation) struct WorthQueryMandatoryReviewBinding {
    pub(in crate::domain_computation) mandatory: WorthQueryMandatoryReview,
    pub(in crate::domain_computation) draft: WorthQueryMandatoryReviewDraft,
}

impl WorthQueryMandatoryReviewDraft {
    pub(in crate::domain_computation) fn bind(
        self,
        mandatory: WorthQueryMandatoryReview,
    ) -> WorthQueryMandatoryReviewBinding {
        WorthQueryMandatoryReviewBinding {
            mandatory,
            draft: self,
        }
    }
}

impl WorthQueryMandatoryReviewBinding {
    pub(in crate::domain_computation) fn into_mandatory(self) -> WorthQueryMandatoryReview {
        self.mandatory
    }
}
