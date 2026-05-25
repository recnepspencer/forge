use forge_relational::facade::runtime::InvariantCatalog;

use crate::domain_capabilities::authoring::{
    ForgeQueryInvariantCapabilityContributionAuthoring, ForgeQuerySupportContributionAuthoring,
};
use crate::domain_capabilities::canonical_runtime::{
    materialize_intent_declaration_support_traceability_artifact,
    materialize_query_invariant_catalog_registration_artifact,
    ForgeQueryIntentDeclarationSupportTraceabilityArtifact,
    ForgeQueryInvariantCatalogRegistrationArtifact,
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
    ForgeQueryInvariantCapabilityContributionPosture, ForgeQuerySupportContributionPosture,
};
use crate::domain_capabilities::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityTargetKind,
};

use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDomainContributionSurface {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryDeclarationBoundContributionTarget,
}

impl ForgeQueryIntentDomainContributionSurface {
    pub fn supports_capability(
        self,
        semantic_code: impl Into<String>,
    ) -> ForgeQueryIntentSupportDraft {
        ForgeQueryIntentSupportDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQuerySupportContributionPosture::DeclarationSupport,
            semantic_code,
        )
    }

    pub fn supports_traceability(
        self,
        semantic_code: impl Into<String>,
    ) -> ForgeQueryIntentSupportDraft {
        ForgeQueryIntentSupportDraft::new(
            self.domain,
            self.target,
            crate::domain_capabilities::payloads::ForgeQuerySupportContributionPosture::DeclarationTraceability,
            semantic_code,
        )
    }

    pub fn register_invariant_catalog(
        self,
        semantic_code: impl Into<String>,
        invariant_catalog: InvariantCatalog,
    ) -> ForgeQueryIntentInvariantRegistrationDraft {
        ForgeQueryIntentInvariantRegistrationDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            invariant_catalog,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentSupportDraft {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    posture: ForgeQuerySupportContributionPosture,
    semantic_code: String,
}

impl ForgeQueryIntentSupportDraft {
    pub(crate) fn new(
        domain: String,
        target: ForgeQueryDeclarationBoundContributionTarget,
        posture: ForgeQuerySupportContributionPosture,
        semantic_code: impl Into<String>,
    ) -> Self {
        Self {
            domain,
            target,
            posture,
            semantic_code: semantic_code.into(),
        }
    }

    pub fn because(self, detail: impl Into<String>) -> ForgeQueryIntentSupportContribution {
        ForgeQueryIntentSupportContribution {
            domain: self.domain,
            target: self.target,
            posture: self.posture,
            semantic_code: self.semantic_code,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentSupportContribution {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    posture: ForgeQuerySupportContributionPosture,
    semantic_code: String,
    detail: String,
}

impl ForgeQueryIntentSupportContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<
        ForgeQueryIntentDeclarationSupportTraceabilityArtifact,
    > {
        let semantic_code = qualify_semantic_code(&self.domain, &self.semantic_code);
        let target = self.target.clone();
        let requested = match self.posture {
            ForgeQuerySupportContributionPosture::DeclarationSupport => {
                ForgeQuerySupportContributionAuthoring::declaration_support(
                    semantic_code,
                    self.detail,
                )
                .bind_to_declaration_target(self.target)
            }
            ForgeQuerySupportContributionPosture::DeclarationTraceability => {
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    semantic_code,
                    self.detail,
                )
                .bind_to_declaration_target(self.target)
            }
            ForgeQuerySupportContributionPosture::NarrowedSupport => unreachable!(),
        };

        materialize_common_lane(
            "support-traceability",
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
            materialize_intent_declaration_support_traceability_artifact,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        ForgeQueryIntentDeclarationSupportTraceabilityArtifact,
        ForgeQueryDomainCapabilityMaterializationError,
    > {
        self.try_materialize().into_result()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentInvariantRegistrationDraft {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryDeclarationBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) invariant_catalog: InvariantCatalog,
}

impl ForgeQueryIntentInvariantRegistrationDraft {
    pub fn because(
        self,
        detail: impl Into<String>,
    ) -> ForgeQueryIntentInvariantRegistrationContribution {
        ForgeQueryIntentInvariantRegistrationContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            invariant_catalog: self.invariant_catalog,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentInvariantRegistrationContribution {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    semantic_code: String,
    invariant_catalog: InvariantCatalog,
    detail: String,
}

impl ForgeQueryIntentInvariantRegistrationContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryInvariantCatalogRegistrationArtifact>
    {
        let target = self.target.clone();
        let requested = ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
            self.invariant_catalog,
            qualify_semantic_code(&self.domain, &self.semantic_code),
            self.detail,
        )
        .bind_to_declaration_target(self.target);

        materialize_common_lane(
            "invariant-capability",
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration.as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_query_invariant_catalog_registration_artifact,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        ForgeQueryInvariantCatalogRegistrationArtifact,
        ForgeQueryDomainCapabilityMaterializationError,
    > {
        self.try_materialize().into_result()
    }
}
