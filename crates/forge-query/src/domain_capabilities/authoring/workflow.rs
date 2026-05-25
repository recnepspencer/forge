use crate::basis::ExecutionPreflightBundle;
use crate::runtime::{ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration};
use crate::workflow::{
    MergeLoweringInput, MutationLoweringInput, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass, WritebackLoweringInput,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::EntityId;

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
    ForgeQueryWorkflowLoweringSemantics, ForgeQueryWorkflowRuntimeBindingSemantics,
    ForgeQueryWorkflowRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::ForgeQueryRequestedWorkflowContribution;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkflowContributionAuthoring {
    pub(super) payload: ForgeQueryWorkflowContributionPayload,
}

impl ForgeQueryWorkflowContributionAuthoring {
    pub fn preview_only(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryWorkflowContributionPosture::PreviewOnly,
            semantic_code,
            detail,
        )
    }

    pub fn promotion_eligible(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
        )
    }

    pub fn confirmation_required(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
        )
    }

    pub fn discard_required(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            ForgeQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
        )
    }

    pub fn preview_only_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryWorkflowContributionPosture::PreviewOnly,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::ReadOnly,
                ),
                WorkflowDeclarationFamily::ConflictInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
        )
    }

    pub fn promotion_eligible_mutation_lowering(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowCostClass::MutationLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
        )
    }

    pub fn confirmation_required_mutation_reconciliation(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_snapshot_token: impl Into<String>,
        authority_binding_digest: impl Into<String>,
        entity_id: EntityId,
        desired_payload: serde_json::Value,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                    runtime_snapshot_token,
                ),
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowCostClass::MutationLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowLoweringSemantics::mutation(
                authority_binding_digest,
                MutationLoweringInput::IntentReconciliation {
                    entity_id,
                    desired_payload,
                },
            ),
        )
    }

    pub fn confirmation_required_mutation_reconciliation_from_preflight(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preflight: ExecutionPreflightBundle,
        authority_binding_digest: impl Into<String>,
        entity_id: EntityId,
        desired_payload: serde_json::Value,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowCostClass::MutationLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowLoweringSemantics::mutation(
                authority_binding_digest,
                MutationLoweringInput::IntentReconciliation {
                    entity_id,
                    desired_payload,
                },
            ),
        )
    }

    pub fn promotion_eligible_merge_reconciliation(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: impl Into<String>,
        target_branch: BranchId,
        source_branch: BranchId,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::MergeLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMerge,
                WorkflowCostClass::MergeLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
            ForgeQueryWorkflowLoweringSemantics::merge(MergeLoweringInput::reconcile_into_target(
                target_branch,
                source_branch,
            )),
        )
    }

    pub fn confirmation_required_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_snapshot_token: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                    runtime_snapshot_token,
                ),
                WorkflowDeclarationFamily::ConflictInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
        )
    }

    pub fn confirmation_required_query_inspection_from_preflight(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preflight: ExecutionPreflightBundle,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
                WorkflowDeclarationFamily::ConflictInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
        )
    }

    pub fn confirmation_required_writeback_projected_state_diff(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_snapshot_token: impl Into<String>,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                    runtime_snapshot_token,
                ),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowLoweringSemantics::writeback(
                WritebackLoweringInput::projected_state_diff(),
            ),
        )
    }

    pub fn confirmation_required_writeback_projected_state_diff_from_preflight(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preflight: ExecutionPreflightBundle,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowLoweringSemantics::writeback(
                WritebackLoweringInput::projected_state_diff(),
            ),
        )
    }

    pub fn discard_required_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> Self {
        Self::with_runtime_semantics(
            ForgeQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::ConflictInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
        )
    }

    pub fn discard_required_writeback_projected_state_diff(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: impl Into<String>,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            ForgeQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowLoweringSemantics::writeback(
                WritebackLoweringInput::projected_state_diff(),
            ),
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> ForgeQueryRequestedWorkflowContribution<ForgeQueryDeclarationBoundContributionTarget> {
        self.bind_to_declaration_target(
            ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &ForgeQueryAdmittedIntentPlan,
    ) -> ForgeQueryRequestedWorkflowContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            ForgeQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn bind_to_declaration_target(
        self,
        target: ForgeQueryDeclarationBoundContributionTarget,
    ) -> ForgeQueryRequestedWorkflowContribution<ForgeQueryDeclarationBoundContributionTarget> {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: ForgeQueryAdmittedPlanBoundContributionTarget,
    ) -> ForgeQueryRequestedWorkflowContribution<ForgeQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: ForgeQueryWorkflowContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryWorkflowRuntimeSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryWorkflowContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }

    fn with_runtime_and_lowering_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryWorkflowRuntimeSemantics,
        lowering_semantics: ForgeQueryWorkflowLoweringSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryWorkflowContributionPayload::with_runtime_and_lowering_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
                Some(lowering_semantics),
            ),
        }
    }
}
