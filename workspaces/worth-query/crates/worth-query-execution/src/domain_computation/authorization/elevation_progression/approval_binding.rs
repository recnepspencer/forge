use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_relational::facade::identity::{EntityId, KindId};

use super::WorthQueryElevationRequestBinding;
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;

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
    pub(in crate::domain_computation) required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
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
