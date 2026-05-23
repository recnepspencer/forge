use crate::domain_capabilities::authoring::{
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryContinuityContributionAuthoring,
};
use crate::domain_capabilities::canonical_runtime::{
    materialize_runtime_admission_decision, materialize_runtime_continuity_evidence,
};
use crate::domain_capabilities::dx::checked::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryAdmissionContributionPosture, ForgeQueryContinuityContributionPosture,
};
use crate::domain_capabilities::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetKind,
};
use crate::intent_admission::ForgeQueryIntentAdmissionDecision;
use crate::runtime::ForgeQueryContinuityMutationEvidence;

use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanDomainContributionSurface {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryAdmittedPlanBoundContributionTarget,
}

impl ForgeQueryAdmittedPlanDomainContributionSurface {
    pub fn advises(self, semantic_code: impl Into<String>) -> ForgeQueryAdmittedPlanAdmissionDraft {
        ForgeQueryAdmittedPlanAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture::Advisory,
            semantic_code,
        )
    }

    pub fn violates(
        self,
        semantic_code: impl Into<String>,
    ) -> ForgeQueryAdmittedPlanAdmissionDraft {
        ForgeQueryAdmittedPlanAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture::Violation,
            semantic_code,
        )
    }

    pub fn preserves_continuity(
        self,
        semantic_code: impl Into<String>,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
    ) -> ForgeQueryAdmittedPlanContinuityDraft {
        ForgeQueryAdmittedPlanContinuityDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            prior_authoritative_identity: prior_authoritative_identity.into(),
            successor_authoritative_identity: successor_authoritative_identity.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanAdmissionDraft {
    domain: String,
    target: ForgeQueryAdmittedPlanBoundContributionTarget,
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: String,
}

impl ForgeQueryAdmittedPlanAdmissionDraft {
    pub(crate) fn new(
        domain: String,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
        posture: ForgeQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
    ) -> Self {
        Self {
            domain,
            target,
            posture,
            semantic_code: semantic_code.into(),
        }
    }

    pub fn because(self, detail: impl Into<String>) -> ForgeQueryAdmittedPlanAdmissionContribution {
        ForgeQueryAdmittedPlanAdmissionContribution {
            domain: self.domain,
            target: self.target,
            posture: self.posture,
            semantic_code: self.semantic_code,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanAdmissionContribution {
    domain: String,
    target: ForgeQueryAdmittedPlanBoundContributionTarget,
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: String,
    detail: String,
}

impl ForgeQueryAdmittedPlanAdmissionContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryIntentAdmissionDecision> {
        let target = self.target.clone();
        let requested = match self.posture {
            ForgeQueryAdmissionContributionPosture::Advisory => {
                ForgeQueryAdmissionContributionAuthoring::advisory(
                    qualify_semantic_code(&self.domain, &self.semantic_code),
                    self.detail,
                )
                .bind_to_admitted_plan_target(self.target)
            }
            ForgeQueryAdmissionContributionPosture::Violation => {
                ForgeQueryAdmissionContributionAuthoring::violation(
                    qualify_semantic_code(&self.domain, &self.semantic_code),
                    self.detail,
                )
                .bind_to_admitted_plan_target(self.target)
            }
            ForgeQueryAdmissionContributionPosture::SupportOnly => unreachable!(),
        };

        materialize_common_lane(
            "admission",
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            self.posture.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_runtime_admission_decision,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<ForgeQueryIntentAdmissionDecision, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanContinuityDraft {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryAdmittedPlanBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) prior_authoritative_identity: String,
    pub(crate) successor_authoritative_identity: String,
}

impl ForgeQueryAdmittedPlanContinuityDraft {
    pub fn because(
        self,
        detail: impl Into<String>,
    ) -> ForgeQueryAdmittedPlanContinuityContribution {
        ForgeQueryAdmittedPlanContinuityContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            prior_authoritative_identity: self.prior_authoritative_identity,
            successor_authoritative_identity: self.successor_authoritative_identity,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedPlanContinuityContribution {
    domain: String,
    target: ForgeQueryAdmittedPlanBoundContributionTarget,
    semantic_code: String,
    prior_authoritative_identity: String,
    successor_authoritative_identity: String,
    detail: String,
}

impl ForgeQueryAdmittedPlanContinuityContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryContinuityMutationEvidence> {
        let target = self.target.clone();
        let requested = ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            self.prior_authoritative_identity,
            self.successor_authoritative_identity,
            qualify_semantic_code(&self.domain, &self.semantic_code),
            self.detail,
        )
        .bind_to_admitted_plan_target(self.target);

        materialize_common_lane(
            "continuity-lineage",
            ForgeQueryDomainCapabilityTargetKind::AdmittedIntentPlan,
            ForgeQueryContinuityContributionPosture::Preserved.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_runtime_continuity_evidence,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<ForgeQueryContinuityMutationEvidence, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}
