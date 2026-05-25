use crate::domain_capabilities::authoring::ForgeQueryAdmissionContributionAuthoring;
use crate::domain_capabilities::canonical_runtime::{
    materialize_canonical_admission_artifact, ForgeQueryCanonicalAdmissionArtifact,
};
use crate::domain_capabilities::dx::checked::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture;
use crate::domain_capabilities::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityTargetKind,
};

use super::intent::ForgeQueryIntentDomainContributionSurface;
use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionDraft {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: String,
}

impl ForgeQueryIntentDomainContributionSurface {
    pub fn advises(self, semantic_code: impl Into<String>) -> ForgeQueryIntentAdmissionDraft {
        ForgeQueryIntentAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture::Advisory,
            semantic_code,
        )
    }

    pub fn violates_invariant(
        self,
        semantic_code: impl Into<String>,
    ) -> ForgeQueryIntentAdmissionDraft {
        ForgeQueryIntentAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQueryAdmissionContributionPosture::Violation,
            semantic_code,
        )
    }
}

impl ForgeQueryIntentAdmissionDraft {
    pub(crate) fn new(
        domain: String,
        target: ForgeQueryDeclarationBoundContributionTarget,
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

    pub fn because(self, detail: impl Into<String>) -> ForgeQueryIntentAdmissionContribution {
        ForgeQueryIntentAdmissionContribution {
            domain: self.domain,
            target: self.target,
            posture: self.posture,
            semantic_code: self.semantic_code,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionContribution {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    posture: ForgeQueryAdmissionContributionPosture,
    semantic_code: String,
    detail: String,
}

impl ForgeQueryIntentAdmissionContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<
        ForgeQueryCanonicalAdmissionArtifact<ForgeQueryDeclarationBoundContributionTarget>,
    > {
        let semantic_code = qualify_semantic_code(&self.domain, &self.semantic_code);
        let target = self.target.clone();
        let requested = match self.posture {
            ForgeQueryAdmissionContributionPosture::Advisory => {
                ForgeQueryAdmissionContributionAuthoring::advisory(semantic_code, self.detail)
                    .bind_to_declaration_target(self.target)
            }
            ForgeQueryAdmissionContributionPosture::Violation => {
                ForgeQueryAdmissionContributionAuthoring::violation(semantic_code, self.detail)
                    .bind_to_declaration_target(self.target)
            }
            ForgeQueryAdmissionContributionPosture::SupportOnly => unreachable!(),
        };

        materialize_common_lane(
            "admission",
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            self.posture.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            |ready| {
                forge_proof::TransitionOutcome::Success(materialize_canonical_admission_artifact(
                    ready,
                ))
            },
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        ForgeQueryCanonicalAdmissionArtifact<ForgeQueryDeclarationBoundContributionTarget>,
        ForgeQueryDomainCapabilityMaterializationError,
    > {
        self.try_materialize().into_result()
    }
}
