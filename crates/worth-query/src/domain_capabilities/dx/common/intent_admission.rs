use crate::domain_capabilities::authoring::WorthQueryAdmissionContributionAuthoring;
use crate::domain_capabilities::canonical_runtime::{
    materialize_canonical_admission_artifact, WorthQueryCanonicalAdmissionArtifact,
};
use crate::domain_capabilities::dx::checked::{
    WorthQueryCheckedDomainCapabilityOutcome, WorthQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::payloads::WorthQueryAdmissionContributionPosture;
use crate::domain_capabilities::{
    WorthQueryDeclarationBoundContributionTarget, WorthQueryDomainCapabilityTargetKind,
};

use super::intent::WorthQueryIntentDomainContributionSurface;
use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionDraft {
    domain: String,
    target: WorthQueryDeclarationBoundContributionTarget,
    posture: WorthQueryAdmissionContributionPosture,
    semantic_code: String,
}

impl WorthQueryIntentDomainContributionSurface {
    pub fn advises(self, semantic_code: impl Into<String>) -> WorthQueryIntentAdmissionDraft {
        WorthQueryIntentAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::WorthQueryAdmissionContributionPosture::Advisory,
            semantic_code,
        )
    }

    pub fn violates_invariant(
        self,
        semantic_code: impl Into<String>,
    ) -> WorthQueryIntentAdmissionDraft {
        WorthQueryIntentAdmissionDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::WorthQueryAdmissionContributionPosture::Violation,
            semantic_code,
        )
    }
}

impl WorthQueryIntentAdmissionDraft {
    pub(crate) fn new(
        domain: String,
        target: WorthQueryDeclarationBoundContributionTarget,
        posture: WorthQueryAdmissionContributionPosture,
        semantic_code: impl Into<String>,
    ) -> Self {
        Self {
            domain,
            target,
            posture,
            semantic_code: semantic_code.into(),
        }
    }

    pub fn because(self, detail: impl Into<String>) -> WorthQueryIntentAdmissionContribution {
        WorthQueryIntentAdmissionContribution {
            domain: self.domain,
            target: self.target,
            posture: self.posture,
            semantic_code: self.semantic_code,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionContribution {
    domain: String,
    target: WorthQueryDeclarationBoundContributionTarget,
    posture: WorthQueryAdmissionContributionPosture,
    semantic_code: String,
    detail: String,
}

impl WorthQueryIntentAdmissionContribution {
    pub fn try_materialize(
        self,
    ) -> WorthQueryCheckedDomainCapabilityOutcome<
        WorthQueryCanonicalAdmissionArtifact<WorthQueryDeclarationBoundContributionTarget>,
    > {
        let semantic_code = qualify_semantic_code(&self.domain, &self.semantic_code);
        let target = self.target.clone();
        let requested = match self.posture {
            WorthQueryAdmissionContributionPosture::Advisory => {
                WorthQueryAdmissionContributionAuthoring::advisory(semantic_code, self.detail)
                    .bind_to_declaration_target(self.target)
            }
            WorthQueryAdmissionContributionPosture::Violation => {
                WorthQueryAdmissionContributionAuthoring::violation(semantic_code, self.detail)
                    .bind_to_declaration_target(self.target)
            }
            WorthQueryAdmissionContributionPosture::SupportOnly => unreachable!(),
        };

        materialize_common_lane(
            "admission",
            WorthQueryDomainCapabilityTargetKind::IntentDeclaration,
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
                worth_proof::TransitionOutcome::Success(materialize_canonical_admission_artifact(
                    ready,
                ))
            },
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        WorthQueryCanonicalAdmissionArtifact<WorthQueryDeclarationBoundContributionTarget>,
        WorthQueryDomainCapabilityMaterializationError,
    > {
        self.try_materialize().into_result()
    }
}
