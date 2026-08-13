use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::{
    ApplicationOperationDecisionReadTarget, ApplicationOperationProgramTarget,
};
use worth_relational::facade::identity::{EntityId, KindId};

use super::WorthQueryElevationApprovalBindingPermit;
use super::{super::WorthQueryElevationRequestBinding, WorthQueryElevationApprovalDraft};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;
use crate::domain_computation::primary_graph::WorthQueryRequestedElevation;

#[derive(Debug)]
pub(in crate::domain_computation) struct WorthQueryElevationApprovalBinding {
    requested: WorthQueryElevationRequestBinding,
    request_commit: WorthQueryApplicationCommitReceipt,
    elevation: EntityId,
    review: EntityId,
    approver: EntityId,
    approved_status: AspectValue,
    elevation_entity: String,
    status_field: AspectFieldLocator,
    approver_relation: KindId,
    reviewer_relation: KindId,
    required_decision_reads:
        Vec<ApplicationOperationDecisionReadTarget>,
    required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
    lifecycle_effect:
        Option<worth_query_declaration::lifecycle_effect_derivation_authority::DerivedApplicationCapabilityLifecycleEffect>,
}

impl WorthQueryElevationApprovalBinding {
    pub(in crate::domain_computation) const fn requested(
        &self,
    ) -> &WorthQueryElevationRequestBinding {
        &self.requested
    }
    pub(in crate::domain_computation) const fn request_commit(
        &self,
    ) -> &WorthQueryApplicationCommitReceipt {
        &self.request_commit
    }
    pub(in crate::domain_computation) const fn elevation(&self) -> EntityId {
        self.elevation
    }
    pub(in crate::domain_computation) const fn review(&self) -> EntityId {
        self.review
    }
    pub(in crate::domain_computation) const fn approver(&self) -> EntityId {
        self.approver
    }
    pub(in crate::domain_computation) const fn approved_status(&self) -> &AspectValue {
        &self.approved_status
    }
    pub(in crate::domain_computation) fn elevation_entity(&self) -> &str {
        &self.elevation_entity
    }
    pub(in crate::domain_computation) const fn status_field(&self) -> &AspectFieldLocator {
        &self.status_field
    }
    pub(in crate::domain_computation) const fn approver_relation(&self) -> KindId {
        self.approver_relation
    }
    pub(in crate::domain_computation) const fn reviewer_relation(&self) -> KindId {
        self.reviewer_relation
    }
    pub(in crate::domain_computation) fn required_decision_reads(
        &self,
    ) -> &[ApplicationOperationDecisionReadTarget] {
        &self.required_decision_reads
    }
    pub(in crate::domain_computation) fn required_program_targets(
        &self,
    ) -> &[ApplicationOperationProgramTarget] {
        &self.required_program_targets
    }
    pub(in crate::domain_computation) const fn lifecycle_effect(&self) -> Option<&worth_query_declaration::lifecycle_effect_derivation_authority::DerivedApplicationCapabilityLifecycleEffect>{
        self.lifecycle_effect.as_ref()
    }

    pub(in crate::domain_computation) fn into_requested(
        self,
    ) -> crate::domain_computation::primary_graph::WorthQueryRequestedElevation {
        crate::domain_computation::primary_graph::WorthQueryRequestedElevation::restore_after_approval(
            self.requested,
            self.request_commit,
            WorthQueryElevationApprovalBindingPermit::mint(),
        )
    }
}

impl WorthQueryElevationApprovalDraft {
    pub(in crate::domain_computation) fn bind(
        self,
        requested: WorthQueryRequestedElevation,
    ) -> WorthQueryElevationApprovalBinding {
        let (requested, request_commit) =
            requested.into_approval_parts(WorthQueryElevationApprovalBindingPermit::mint());
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
