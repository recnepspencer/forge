use crate::domain_capabilities::authoring::ForgeQueryAftermathContributionAuthoring;
use crate::domain_capabilities::canonical_runtime::{
    materialize_admitted_projection_consumption, materialize_projection_consumption_contract,
    materialize_projection_consumption_eligibility, materialize_projection_consumption_review,
    materialize_projection_consumption_support_report,
    ForgeQueryAftermathProjectionConsumptionReview,
};
use crate::domain_capabilities::dx::checked::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::payloads::ForgeQueryAftermathContributionPosture;
use crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind;
use crate::projection_consumption::{
    AdmittedProjectionConsumption, MaterializedProjectionContract, ProjectMaterializedFacts,
    ProjectionConsumptionBindingContext, ProjectionConsumptionEligibility,
    ProjectionConsumptionSource, ProjectionConsumptionSupportReport,
};

use super::admitted_plan::ForgeQueryAdmittedPlanDomainContributionSurface;
use super::projection_contract_request::ForgeQueryProjectionContractRequest;
use super::shared::{materialize_common_lane, qualify_semantic_code};

impl ForgeQueryAdmittedPlanDomainContributionSurface {
    pub fn establishes_projection_contract(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryProjectionContractRequest,
    ) -> ForgeQueryAdmittedPlanAftermathDraft {
        let (source, binding, requested_facts) = request.into_parts();
        ForgeQueryAdmittedPlanAftermathDraft::new(
            self.domain,
            self.target,
            ForgeQueryAftermathContributionPosture::EstablishesFact,
            semantic_code,
            source,
            binding,
            requested_facts,
        )
    }

    pub fn consumes_projection_contract(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryProjectionContractRequest,
    ) -> ForgeQueryAdmittedPlanAftermathDraft {
        let (source, binding, requested_facts) = request.into_parts();
        ForgeQueryAdmittedPlanAftermathDraft::new(
            self.domain,
            self.target,
            ForgeQueryAftermathContributionPosture::ConsumesFact,
            semantic_code,
            source,
            binding,
            requested_facts,
        )
    }

    pub fn declares_projection_residue(
        self,
        semantic_code: impl Into<String>,
        request: ForgeQueryProjectionContractRequest,
    ) -> ForgeQueryAdmittedPlanAftermathDraft {
        let (source, binding, requested_facts) = request.into_parts();
        ForgeQueryAdmittedPlanAftermathDraft::new(
            self.domain,
            self.target,
            ForgeQueryAftermathContributionPosture::DeclaresResidue,
            semantic_code,
            source,
            binding,
            requested_facts,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanAftermathDraft {
    domain: String,
    target: crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    posture: ForgeQueryAftermathContributionPosture,
    semantic_code: String,
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
}

impl ForgeQueryAdmittedPlanAftermathDraft {
    fn new(
        domain: String,
        target: crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
        posture: ForgeQueryAftermathContributionPosture,
        semantic_code: impl Into<String>,
        source: ProjectionConsumptionSource,
        binding: ProjectionConsumptionBindingContext,
        requested_facts: ProjectMaterializedFacts,
    ) -> Self {
        Self {
            domain,
            target,
            posture,
            semantic_code: semantic_code.into(),
            source,
            binding,
            requested_facts,
        }
    }

    pub fn because(self, detail: impl Into<String>) -> ForgeQueryAdmittedPlanAftermathContribution {
        ForgeQueryAdmittedPlanAftermathContribution {
            domain: self.domain,
            target: self.target,
            posture: self.posture,
            semantic_code: self.semantic_code,
            source: self.source,
            binding: self.binding,
            requested_facts: self.requested_facts,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanAftermathContribution {
    domain: String,
    target: crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    posture: ForgeQueryAftermathContributionPosture,
    semantic_code: String,
    source: ProjectionConsumptionSource,
    binding: ProjectionConsumptionBindingContext,
    requested_facts: ProjectMaterializedFacts,
    detail: String,
}

impl ForgeQueryAdmittedPlanAftermathContribution {
    pub fn try_review(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryAftermathProjectionConsumptionReview>
    {
        self.materialize_checked(materialize_projection_consumption_review)
    }

    pub fn review(
        self,
    ) -> Result<
        ForgeQueryAftermathProjectionConsumptionReview,
        ForgeQueryDomainCapabilityMaterializationError,
    > {
        self.try_review().into_result()
    }

    pub fn try_materialize_support_report(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ProjectionConsumptionSupportReport> {
        self.materialize_checked(materialize_projection_consumption_support_report)
    }

    pub fn materialize_support_report(
        self,
    ) -> Result<ProjectionConsumptionSupportReport, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize_support_report().into_result()
    }

    pub fn try_materialize_eligibility(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ProjectionConsumptionEligibility> {
        self.materialize_checked(materialize_projection_consumption_eligibility)
    }

    pub fn materialize_eligibility(
        self,
    ) -> Result<ProjectionConsumptionEligibility, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize_eligibility().into_result()
    }

    pub fn try_materialize_admitted(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<AdmittedProjectionConsumption> {
        self.materialize_checked(materialize_admitted_projection_consumption)
    }

    pub fn materialize_admitted(
        self,
    ) -> Result<AdmittedProjectionConsumption, ForgeQueryDomainCapabilityMaterializationError> {
        self.try_materialize_admitted().into_result()
    }

    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<MaterializedProjectionContract> {
        self.materialize_checked(materialize_projection_consumption_contract)
    }

    pub fn materialize(
        self,
    ) -> Result<MaterializedProjectionContract, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }

    fn into_requested(
        self,
    ) -> crate::domain_capabilities::ForgeQueryRequestedAftermathContribution<
        crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
    > {
        let semantic_code = qualify_semantic_code(&self.domain, &self.semantic_code);
        match self.posture {
            ForgeQueryAftermathContributionPosture::EstablishesFact => {
                ForgeQueryAftermathContributionAuthoring::establishes_projection_contract(
                    semantic_code,
                    self.detail,
                    self.source,
                    self.binding,
                    self.requested_facts,
                )
                .bind_to_admitted_plan_target(self.target)
            }
            ForgeQueryAftermathContributionPosture::ConsumesFact => {
                ForgeQueryAftermathContributionAuthoring::consumes_projection_contract(
                    semantic_code,
                    self.detail,
                    self.source,
                    self.binding,
                    self.requested_facts,
                )
                .bind_to_admitted_plan_target(self.target)
            }
            ForgeQueryAftermathContributionPosture::DeclaresResidue => {
                ForgeQueryAftermathContributionAuthoring::declares_projection_residue(
                    semantic_code,
                    self.detail,
                    self.source,
                    self.binding,
                    self.requested_facts,
                )
                .bind_to_admitted_plan_target(self.target)
            }
        }
    }

    fn materialize_checked<Success>(
        self,
        materialize: impl FnOnce(
            crate::domain_capabilities::ForgeQueryMaterializationReadyAftermathContribution<
                crate::domain_capabilities::ForgeQueryAdmittedPlanBoundContributionTarget,
            >,
        ) -> crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<Success>,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<Success> {
        let posture = self.posture;
        let target = self.target.clone();
        let requested = self.into_requested();

        materialize_common_lane(
            "consequence-aftermath",
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            posture.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize,
        )
    }
}
