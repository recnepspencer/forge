use crate::domain_capabilities::authoring::ForgeQueryWorkflowContributionAuthoring;
use crate::domain_capabilities::canonical_runtime::{
    materialize_query_preview_workflow_artifact, materialize_query_workflow_declaration,
};
use crate::domain_capabilities::dx::checked::{
    ForgeQueryCheckedDomainCapabilityOutcome, ForgeQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityTargetKind,
};
use crate::preview::PreviewWorkflowFoundationArtifact;
use crate::workflow::QueryWorkflowDeclaration;

use super::intent::ForgeQueryIntentDomainContributionSurface;
use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentPreviewInspectionDraft {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryDeclarationBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) preview_session_identity: String,
}

impl ForgeQueryIntentDomainContributionSurface {
    pub fn inspects_query_preview(
        self,
        semantic_code: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> ForgeQueryIntentPreviewInspectionDraft {
        ForgeQueryIntentPreviewInspectionDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            preview_session_identity: preview_session_identity.into(),
        }
    }

    pub fn plans_preview_mutation(
        self,
        semantic_code: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> ForgeQueryIntentPreviewMutationDraft {
        ForgeQueryIntentPreviewMutationDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            preview_session_identity: preview_session_identity.into(),
        }
    }
}

impl ForgeQueryIntentPreviewInspectionDraft {
    pub fn because(
        self,
        detail: impl Into<String>,
    ) -> ForgeQueryIntentPreviewInspectionContribution {
        ForgeQueryIntentPreviewInspectionContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            preview_session_identity: self.preview_session_identity,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentPreviewInspectionContribution {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    semantic_code: String,
    preview_session_identity: String,
    detail: String,
}

impl ForgeQueryIntentPreviewInspectionContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<PreviewWorkflowFoundationArtifact> {
        let target = self.target.clone();
        let requested = ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            qualify_semantic_code(&self.domain, &self.semantic_code),
            self.detail,
            self.preview_session_identity,
        )
        .bind_to_declaration_target(self.target);

        materialize_common_lane(
            "workflow-preview",
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            crate::domain_capabilities::payloads::ForgeQueryWorkflowContributionPosture::PreviewOnly
                .as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            materialize_query_preview_workflow_artifact,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<PreviewWorkflowFoundationArtifact, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentPreviewMutationDraft {
    pub(crate) domain: String,
    pub(crate) target: ForgeQueryDeclarationBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) preview_session_identity: String,
}

impl ForgeQueryIntentPreviewMutationDraft {
    pub fn because(self, detail: impl Into<String>) -> ForgeQueryIntentPreviewMutationContribution {
        ForgeQueryIntentPreviewMutationContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            preview_session_identity: self.preview_session_identity,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentPreviewMutationContribution {
    domain: String,
    target: ForgeQueryDeclarationBoundContributionTarget,
    semantic_code: String,
    preview_session_identity: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentPreviewMutationPlan {
    inner: QueryWorkflowDeclaration,
}

impl ForgeQueryIntentPreviewMutationPlan {
    pub fn workflow_declaration(&self) -> &QueryWorkflowDeclaration {
        &self.inner
    }

    pub fn into_workflow_declaration(self) -> QueryWorkflowDeclaration {
        self.inner
    }
}

impl ForgeQueryIntentPreviewMutationContribution {
    pub fn try_materialize(
        self,
    ) -> ForgeQueryCheckedDomainCapabilityOutcome<ForgeQueryIntentPreviewMutationPlan> {
        let target = self.target.clone();
        let requested =
            ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
                qualify_semantic_code(&self.domain, &self.semantic_code),
                self.detail,
                self.preview_session_identity,
            )
            .bind_to_declaration_target(self.target);

        materialize_common_lane(
            "workflow-preview",
            ForgeQueryDomainCapabilityTargetKind::IntentDeclaration,
            crate::domain_capabilities::payloads::ForgeQueryWorkflowContributionPosture::PromotionEligible
                .as_str(),
            requested,
            evaluate_requested_domain_capability_contribution,
            admit_eligible_domain_capability_contribution,
            |admitted| {
                prepare_admitted_domain_capability_contribution_for_materialization(
                    admitted, target,
                )
            },
            |ready| {
                materialize_query_workflow_declaration(ready).map_success(
                    |declaration| ForgeQueryIntentPreviewMutationPlan { inner: declaration },
                )
            },
        )
    }

    pub fn materialize(
        self,
    ) -> Result<ForgeQueryIntentPreviewMutationPlan, ForgeQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}
