use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_relational::facade::identity::EntityId;

use crate::domain_computation::primary_graph::{
    WorthQueryApprovedElevation, WorthQueryElevationClosureKind,
};

pub(in crate::domain_computation) struct WorthQueryElevationCloseDraft {
    pub(in crate::domain_computation) elevation: EntityId,
    pub(in crate::domain_computation) review: EntityId,
    pub(in crate::domain_computation) closer: EntityId,
    pub(in crate::domain_computation) closure_kind: WorthQueryElevationClosureKind,
    pub(in crate::domain_computation) closed_at: AspectValue,
    pub(in crate::domain_computation) closed_status: AspectValue,
    pub(in crate::domain_computation) approved_status: AspectValue,
    pub(in crate::domain_computation) elevation_entity: String,
    pub(in crate::domain_computation) status_field: AspectFieldLocator,
    pub(in crate::domain_computation) approver_relation: worth_relational::facade::identity::KindId,
    pub(in crate::domain_computation) reviewer_relation: worth_relational::facade::identity::KindId,
    pub(in crate::domain_computation) required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
}

pub(in crate::domain_computation) struct WorthQueryElevationCloseBinding {
    pub(in crate::domain_computation) approved: WorthQueryApprovedElevation,
    pub(in crate::domain_computation) draft: WorthQueryElevationCloseDraft,
}

impl WorthQueryElevationCloseDraft {
    pub(in crate::domain_computation) fn bind(
        self,
        approved: WorthQueryApprovedElevation,
    ) -> WorthQueryElevationCloseBinding {
        WorthQueryElevationCloseBinding {
            approved,
            draft: self,
        }
    }
}

impl WorthQueryElevationCloseBinding {
    pub(in crate::domain_computation) fn into_approved(self) -> WorthQueryApprovedElevation {
        self.approved
    }
}
