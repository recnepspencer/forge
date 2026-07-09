use crate::domain_capabilities::authoring::WorthQueryWorkflowContributionAuthoring;
use crate::domain_capabilities::canonical_runtime::{
    materialize_query_preview_workflow_artifact, materialize_query_workflow_declaration,
};
use crate::domain_capabilities::dx::checked::{
    WorthQueryCheckedDomainCapabilityOutcome, WorthQueryDomainCapabilityMaterializationError,
};
use crate::domain_capabilities::eligibility::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    prepare_admitted_domain_capability_contribution_for_materialization,
};
use crate::domain_capabilities::{
    WorthQueryDeclarationBoundContributionTarget, WorthQueryDomainCapabilityTargetKind,
};
use crate::preview::PreviewWorkflowFoundationArtifact;
use crate::workflow::QueryWorkflowDeclaration;
use worth_runtime_bridge::facade::BridgePreviewSessionIdentity;

use super::intent::WorthQueryIntentDomainContributionSurface;
use super::shared::{materialize_common_lane, qualify_semantic_code};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentPreviewInspectionDraft {
    pub(crate) domain: String,
    pub(crate) target: WorthQueryDeclarationBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) preview_session_identity: BridgePreviewSessionIdentity,
}

impl WorthQueryIntentDomainContributionSurface {
    pub fn inspects_query_preview(
        self,
        semantic_code: impl Into<String>,
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> WorthQueryIntentPreviewInspectionDraft {
        WorthQueryIntentPreviewInspectionDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            preview_session_identity,
        }
    }

    pub fn plans_preview_mutation(
        self,
        semantic_code: impl Into<String>,
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> WorthQueryIntentPreviewMutationDraft {
        WorthQueryIntentPreviewMutationDraft {
            domain: self.domain,
            target: self.target,
            semantic_code: semantic_code.into(),
            preview_session_identity,
        }
    }
}

impl WorthQueryIntentPreviewInspectionDraft {
    pub fn because(
        self,
        detail: impl Into<String>,
    ) -> WorthQueryIntentPreviewInspectionContribution {
        WorthQueryIntentPreviewInspectionContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            preview_session_identity: self.preview_session_identity,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentPreviewInspectionContribution {
    domain: String,
    target: WorthQueryDeclarationBoundContributionTarget,
    semantic_code: String,
    preview_session_identity: BridgePreviewSessionIdentity,
    detail: String,
}

impl WorthQueryIntentPreviewInspectionContribution {
    pub fn try_materialize(
        self,
    ) -> WorthQueryCheckedDomainCapabilityOutcome<PreviewWorkflowFoundationArtifact> {
        let target = self.target.clone();
        let requested = WorthQueryWorkflowContributionAuthoring::preview_only_query_inspection(
            qualify_semantic_code(&self.domain, &self.semantic_code),
            self.detail,
            self.preview_session_identity,
        )
        .bind_to_declaration_target(self.target);

        materialize_common_lane(
            "workflow-preview",
            WorthQueryDomainCapabilityTargetKind::IntentDeclaration,
            crate::domain_capabilities::payloads::WorthQueryWorkflowContributionPosture::PreviewOnly
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
    ) -> Result<PreviewWorkflowFoundationArtifact, WorthQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentPreviewMutationDraft {
    pub(crate) domain: String,
    pub(crate) target: WorthQueryDeclarationBoundContributionTarget,
    pub(crate) semantic_code: String,
    pub(crate) preview_session_identity: BridgePreviewSessionIdentity,
}

impl WorthQueryIntentPreviewMutationDraft {
    pub fn because(self, detail: impl Into<String>) -> WorthQueryIntentPreviewMutationContribution {
        WorthQueryIntentPreviewMutationContribution {
            domain: self.domain,
            target: self.target,
            semantic_code: self.semantic_code,
            preview_session_identity: self.preview_session_identity,
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentPreviewMutationContribution {
    domain: String,
    target: WorthQueryDeclarationBoundContributionTarget,
    semantic_code: String,
    preview_session_identity: BridgePreviewSessionIdentity,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentPreviewMutationPlan {
    inner: QueryWorkflowDeclaration,
}

impl WorthQueryIntentPreviewMutationPlan {
    pub fn workflow_declaration(&self) -> &QueryWorkflowDeclaration {
        &self.inner
    }

    pub fn into_workflow_declaration(self) -> QueryWorkflowDeclaration {
        self.inner
    }
}

impl WorthQueryIntentPreviewMutationContribution {
    pub fn try_materialize(
        self,
    ) -> WorthQueryCheckedDomainCapabilityOutcome<WorthQueryIntentPreviewMutationPlan> {
        let target = self.target.clone();
        let requested =
            WorthQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
                qualify_semantic_code(&self.domain, &self.semantic_code),
                self.detail,
                self.preview_session_identity,
            )
            .bind_to_declaration_target(self.target);

        materialize_common_lane(
            "workflow-preview",
            WorthQueryDomainCapabilityTargetKind::IntentDeclaration,
            crate::domain_capabilities::payloads::WorthQueryWorkflowContributionPosture::PromotionEligible
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
                    |declaration| WorthQueryIntentPreviewMutationPlan { inner: declaration },
                )
            },
        )
    }

    pub fn materialize(
        self,
    ) -> Result<WorthQueryIntentPreviewMutationPlan, WorthQueryDomainCapabilityMaterializationError>
    {
        self.try_materialize().into_result()
    }
}
