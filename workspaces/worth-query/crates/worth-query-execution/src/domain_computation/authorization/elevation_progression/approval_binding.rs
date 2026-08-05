use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_relational::facade::identity::{EntityId, KindId};

use super::WorthQueryElevationRequestBinding;
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;
use crate::domain_computation::primary_graph::WorthQueryRequestedElevation;

pub(in crate::domain_computation) struct WorthQueryElevationApprovalDraft {
    pub(in crate::domain_computation) elevation: EntityId,
    pub(in crate::domain_computation) review: EntityId,
    pub(in crate::domain_computation) approver: EntityId,
    pub(in crate::domain_computation) approved_status: AspectValue,
    pub(in crate::domain_computation) elevation_entity: String,
    pub(in crate::domain_computation) status_field: AspectFieldLocator,
    pub(in crate::domain_computation) approver_relation: KindId,
    pub(in crate::domain_computation) reviewer_relation: KindId,
    pub(in crate::domain_computation) required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
    pub(in crate::domain_computation) lifecycle_effect:
        Option<worth_query_declaration::lifecycle_effect_derivation_authority::DerivedApplicationCapabilityLifecycleEffect>,
}

#[derive(Debug)]
pub(in crate::domain_computation) struct WorthQueryElevationApprovalBinding {
    pub(in crate::domain_computation) requested: WorthQueryElevationRequestBinding,
    pub(in crate::domain_computation) request_commit: WorthQueryApplicationCommitReceipt,
    pub(in crate::domain_computation) elevation: EntityId,
    pub(in crate::domain_computation) review: EntityId,
    pub(in crate::domain_computation) approver: EntityId,
    pub(in crate::domain_computation) approved_status: AspectValue,
    pub(in crate::domain_computation) elevation_entity: String,
    pub(in crate::domain_computation) status_field: AspectFieldLocator,
    pub(in crate::domain_computation) approver_relation: KindId,
    pub(in crate::domain_computation) reviewer_relation: KindId,
    pub(in crate::domain_computation) required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
    pub(in crate::domain_computation) lifecycle_effect:
        Option<worth_query_declaration::lifecycle_effect_derivation_authority::DerivedApplicationCapabilityLifecycleEffect>,
}

impl WorthQueryElevationApprovalBinding {
    pub(in crate::domain_computation) fn into_requested(
        self,
    ) -> crate::domain_computation::primary_graph::WorthQueryRequestedElevation {
        crate::domain_computation::primary_graph::WorthQueryRequestedElevation::new(
            self.requested,
            self.request_commit,
        )
    }
}

impl WorthQueryElevationApprovalDraft {
    pub(in crate::domain_computation) fn bind(
        self,
        requested: WorthQueryRequestedElevation,
    ) -> WorthQueryElevationApprovalBinding {
        let (requested, request_commit) = requested.into_parts();
        WorthQueryElevationApprovalBinding {
            requested,
            request_commit,
            elevation: self.elevation,
            review: self.review,
            approver: self.approver,
            approved_status: self.approved_status,
            elevation_entity: self.elevation_entity,
            status_field: self.status_field,
            approver_relation: self.approver_relation,
            reviewer_relation: self.reviewer_relation,
            required_decision_reads: self.required_decision_reads,
            required_program_targets: self.required_program_targets,
            lifecycle_effect: self.lifecycle_effect,
        }
    }
}
