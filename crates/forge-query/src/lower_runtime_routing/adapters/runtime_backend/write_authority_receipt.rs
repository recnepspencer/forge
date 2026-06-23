use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRetainedEvidenceIdentity,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
    ForgeQueryLowerRuntimeRouteSubjectIdentity, ForgeQueryLowerRuntimeSeamKey,
    ForgeQueryLowerRuntimeSubjectIdentity,
};
use crate::memory_workspace::ForgeQueryMutationReceipt;
use crate::runtime::{ForgeQueryBackendAdmissibleMutation, ForgeQueryWriteCommand};

use super::subject_digest::{
    backend_admissible_mutation_subject_identity, write_command_subject_identity,
};

pub(super) const WRITE_AUTHORITY_CAPABILITY_LABEL: &str = "write-authority-backend-execution";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteAuthorityExecutionReceipt {
    mutation_receipt: ForgeQueryMutationReceipt,
    route_plan: ForgeQueryLowerRuntimeRoutePlan,
    boundary_execution_receipt: ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    boundary_envelope: ForgeQueryLowerRuntimeBoundaryEnvelope,
}

impl WriteAuthorityExecutionReceipt {
    pub(crate) fn from_backend_admissible_mutation(
        mutation: &ForgeQueryBackendAdmissibleMutation,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        Self::from_subject_identity(
            backend_admissible_mutation_subject_identity(mutation),
            mutation_receipt,
        )
    }

    pub(crate) fn from_command(
        command: &ForgeQueryWriteCommand,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        Self::from_subject_identity(write_command_subject_identity(command), mutation_receipt)
    }

    fn from_subject_identity(
        subject_identity: ForgeQueryLowerRuntimeSubjectIdentity,
        mutation_receipt: ForgeQueryMutationReceipt,
    ) -> Self {
        let capability_request = ForgeQueryLowerRuntimeCapabilityRequest::new(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            subject_identity,
        );
        let commit_evidence_identity = mutation_receipt.commit_identity.evidence_identity();
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "write-authority-commit",
                &commit_evidence_identity,
            );
        let eligibility =
            ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
                capability_request,
                &commit_evidence_identity,
            );
        let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
            eligibility,
            ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
                "write-authority-route",
                &commit_evidence_identity,
            ),
        );
        let boundary_execution_receipt =
            ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan_with_retained_evidence_identity(
                &route_plan,
                &retained_evidence_identity,
            );
        let boundary_envelope =
            ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan_with_retained_evidence_identity(
                ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
                &route_plan,
                &boundary_execution_receipt,
                &retained_evidence_identity,
            );
        Self {
            mutation_receipt,
            route_plan,
            boundary_execution_receipt,
            boundary_envelope,
        }
    }

    pub fn mutation_receipt(&self) -> &ForgeQueryMutationReceipt {
        &self.mutation_receipt
    }

    pub fn route_plan(&self) -> &ForgeQueryLowerRuntimeRoutePlan {
        &self.route_plan
    }

    pub fn boundary_execution_receipt(&self) -> &ForgeQueryLowerRuntimeBoundaryExecutionReceipt {
        &self.boundary_execution_receipt
    }

    pub fn boundary_envelope(&self) -> &ForgeQueryLowerRuntimeBoundaryEnvelope {
        &self.boundary_envelope
    }

    pub(crate) fn drift_from_backend_admissible_mutation(
        &self,
        mutation: &ForgeQueryBackendAdmissibleMutation,
    ) -> Option<String> {
        self.drift_from_subject_identity(backend_admissible_mutation_subject_identity(mutation))
    }

    fn drift_from_subject_identity(
        &self,
        expected_subject: ForgeQueryLowerRuntimeSubjectIdentity,
    ) -> Option<String> {
        if let Some(message) = self.route_plan.eligibility().request().drift_from_contract(
            ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
            ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
            ForgeQueryLowerRuntimeAuthorityOwner::Query,
            WRITE_AUTHORITY_CAPABILITY_LABEL,
            &expected_subject,
        ) {
            return Some(message);
        }
        let commit_evidence_identity = self.mutation_receipt.commit_identity.evidence_identity();
        let retained_evidence_identity =
            ForgeQueryLowerRuntimeRetainedEvidenceIdentity::from_evidence_identity(
                "write-authority-commit",
                &commit_evidence_identity,
            );
        self.boundary_execution_receipt
            .drift_from_route_plan_with_retained_evidence_identity(
                &self.route_plan,
                &retained_evidence_identity,
            )
    }
}
