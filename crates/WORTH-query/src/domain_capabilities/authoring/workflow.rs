use crate::basis::ExecutionPreflightBundle;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::runtime::{WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration};
use crate::workflow::{
    MergeLoweringInput, MutationLoweringInput, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass, WritebackLoweringInput,
};
use crate::WorthQueryEvidenceIdentity;
use worth_relational::facade::history::BranchId;
use worth_relational::facade::identity::EntityId;
use worth_relational::facade::transactions::AspectFieldPatch;
use worth_runtime_bridge::facade::BridgePreviewSessionIdentity;

use super::bind_requested;
use crate::domain_capabilities::payloads::{
    WorthQueryWorkflowContributionPayload, WorthQueryWorkflowContributionPosture,
    WorthQueryWorkflowLoweringSemantics, WorthQueryWorkflowRuntimeBindingSemantics,
    WorthQueryWorkflowRuntimeSemantics,
};
use crate::domain_capabilities::proof_integration::WorthQueryRequestedWorkflowContribution;
use crate::domain_capabilities::targets::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowContributionAuthoring {
    pub(super) payload: WorthQueryWorkflowContributionPayload,
}

impl WorthQueryWorkflowContributionAuthoring {
    pub fn preview_only(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryWorkflowContributionPosture::PreviewOnly,
            semantic_code,
            detail,
        )
    }

    pub fn promotion_eligible(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
        )
    }

    pub fn confirmation_required(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
        )
    }

    pub fn discard_required(semantic_code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self::new(
            WorthQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
        )
    }

    pub fn preview_only_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> Self {
        Self::with_runtime_semantics(
            WorthQueryWorkflowContributionPosture::PreviewOnly,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::preview_foundation(
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
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> Self {
        Self::with_runtime_semantics(
            WorthQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::preview_foundation(
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
        runtime_snapshot_identity: WorthQuerySnapshotIdentity,
        authority_binding_identity: WorthQueryEvidenceIdentity,
        entity_id: EntityId,
        desired_aspect_fields: AspectFieldPatch,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_snapshot_identity(
                    runtime_snapshot_identity,
                ),
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowCostClass::MutationLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            WorthQueryWorkflowLoweringSemantics::mutation(
                authority_binding_identity,
                MutationLoweringInput::IntentReconciliation {
                    entity_id,
                    desired_aspect_fields,
                },
            ),
        )
    }

    pub fn confirmation_required_mutation_reconciliation_from_preflight(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preflight: ExecutionPreflightBundle,
        authority_binding_identity: WorthQueryEvidenceIdentity,
        entity_id: EntityId,
        desired_aspect_fields: AspectFieldPatch,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowCostClass::MutationLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            WorthQueryWorkflowLoweringSemantics::mutation(
                authority_binding_identity,
                MutationLoweringInput::IntentReconciliation {
                    entity_id,
                    desired_aspect_fields,
                },
            ),
        )
    }

    pub fn promotion_eligible_merge_reconciliation(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: BridgePreviewSessionIdentity,
        target_branch: BranchId,
        source_branch: BranchId,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            WorthQueryWorkflowContributionPosture::PromotionEligible,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::MergeLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMerge,
                WorkflowCostClass::MergeLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
            WorthQueryWorkflowLoweringSemantics::merge(MergeLoweringInput::reconcile_into_target(
                target_branch,
                source_branch,
            )),
        )
    }

    pub fn confirmation_required_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_snapshot_identity: WorthQuerySnapshotIdentity,
    ) -> Self {
        Self::with_runtime_semantics(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_snapshot_identity(
                    runtime_snapshot_identity,
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
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
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
        runtime_snapshot_identity: WorthQuerySnapshotIdentity,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_snapshot_identity(
                    runtime_snapshot_identity,
                ),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            WorthQueryWorkflowLoweringSemantics::writeback(
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
            WorthQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::runtime_preflight_bundle(preflight),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            WorthQueryWorkflowLoweringSemantics::writeback(
                WritebackLoweringInput::projected_state_diff(),
            ),
        )
    }

    pub fn discard_required_query_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> Self {
        Self::with_runtime_semantics(
            WorthQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::preview_foundation(
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
        preview_session_identity: BridgePreviewSessionIdentity,
    ) -> Self {
        Self::with_runtime_and_lowering_semantics(
            WorthQueryWorkflowContributionPosture::DiscardRequired,
            semantic_code,
            detail,
            WorthQueryWorkflowRuntimeSemantics::new(
                WorthQueryWorkflowRuntimeBindingSemantics::preview_foundation(
                    preview_session_identity,
                    WorkflowPreviewEvaluationClass::PromotionEligible,
                ),
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowCostClass::WritebackLoweringNarrow,
                WorkflowBudgetClass::AuthorityTargetBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            WorthQueryWorkflowLoweringSemantics::writeback(
                WritebackLoweringInput::projected_state_diff(),
            ),
        )
    }

    pub fn for_intent_declaration(
        self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> WorthQueryRequestedWorkflowContribution<WorthQueryDeclarationBoundContributionTarget> {
        self.bind_to_declaration_target(
            WorthQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_admitted_intent_plan(
        self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> WorthQueryRequestedWorkflowContribution<WorthQueryAdmittedPlanBoundContributionTarget>
    {
        self.bind_to_admitted_plan_target(
            WorthQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn bind_to_declaration_target(
        self,
        target: WorthQueryDeclarationBoundContributionTarget,
    ) -> WorthQueryRequestedWorkflowContribution<WorthQueryDeclarationBoundContributionTarget> {
        bind_requested(self.payload, target)
    }

    pub fn bind_to_admitted_plan_target(
        self,
        target: WorthQueryAdmittedPlanBoundContributionTarget,
    ) -> WorthQueryRequestedWorkflowContribution<WorthQueryAdmittedPlanBoundContributionTarget>
    {
        bind_requested(self.payload, target)
    }

    fn new(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::new(posture, semantic_code, detail),
        }
    }

    fn with_runtime_semantics(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: WorthQueryWorkflowRuntimeSemantics,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::with_runtime_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
            ),
        }
    }

    fn with_runtime_and_lowering_semantics(
        posture: WorthQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: WorthQueryWorkflowRuntimeSemantics,
        lowering_semantics: WorthQueryWorkflowLoweringSemantics,
    ) -> Self {
        Self {
            payload: WorthQueryWorkflowContributionPayload::with_runtime_and_lowering_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
                Some(lowering_semantics),
            ),
        }
    }
}
