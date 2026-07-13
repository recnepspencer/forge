use super::access_request::AdmittedPlanningRequest;
use super::decision::decide_access_plan;
use super::selection_issuance::issue_selection_outcome;
use super::selection_outcome::AccessPlanSelectionOutcome;
use super::{
    AdmittedPhysicalMutationRequest, AdmittedPhysicalReadRequest, AdmittedPhysicalRecoveryRequest,
    PhysicalAccessRequestAdmissionDenied,
};
use crate::access::shape::AccessShapeContract;
use crate::artifact_family::AdmittedPhysicalArtifactFamily;
use crate::keyspace::AdmittedConcretePhysicalKey;
use crate::materialization::AdmittedLayoutMaterialization;
use forge_store_budgets::PreExecutionBudgetEnvelope;

/// The sole operation authorized to decide and issue an access-plan outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPlanSelector;

impl AccessPlanSelector {
    pub(crate) fn admit_read_request(
        &self,
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        materialization: AdmittedLayoutMaterialization,
        access_shape: AccessShapeContract,
    ) -> Result<AdmittedPhysicalReadRequest, PhysicalAccessRequestAdmissionDenied> {
        AdmittedPhysicalReadRequest::admit(family, concrete_key, materialization, access_shape)
    }

    pub(crate) fn admit_recovery_request(
        &self,
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        materialization: AdmittedLayoutMaterialization,
        access_shape: AccessShapeContract,
    ) -> Result<AdmittedPhysicalRecoveryRequest, PhysicalAccessRequestAdmissionDenied> {
        AdmittedPhysicalRecoveryRequest::admit(family, concrete_key, materialization, access_shape)
    }

    pub(crate) fn admit_mutation_request(
        &self,
        family: AdmittedPhysicalArtifactFamily,
        concrete_key: AdmittedConcretePhysicalKey,
        access_shape: AccessShapeContract,
    ) -> Result<AdmittedPhysicalMutationRequest, PhysicalAccessRequestAdmissionDenied> {
        AdmittedPhysicalMutationRequest::admit(family, concrete_key, access_shape)
    }

    pub(crate) fn select_admitted_with_budget<Request>(
        &self,
        request: Request,
        admitted_budget: PreExecutionBudgetEnvelope,
    ) -> AccessPlanSelectionOutcome
    where
        Request: AdmittedPlanningRequest,
    {
        let (family, key_domain, request_identity, materialization, intent) = request.into_parts();
        issue_selection_outcome(decide_access_plan(
            family,
            key_domain,
            request_identity,
            materialization,
            intent,
            admitted_budget,
        ))
    }
}
