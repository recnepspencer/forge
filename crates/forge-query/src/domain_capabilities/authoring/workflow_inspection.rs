use forge_relational::facade::merge::RelationalMergeInspectionArtifact;

use crate::domain_capabilities::payloads::{
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
    ForgeQueryWorkflowInspectionSemantics, ForgeQueryWorkflowRuntimeBindingSemantics,
    ForgeQueryWorkflowRuntimeSemantics,
};
use crate::workflow::{
    LoweredMergeWorkflowDeclaration, QueryWritebackDeclaration, WorkflowAuthorityTargetFamily,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass,
};

use super::workflow::ForgeQueryWorkflowContributionAuthoring;

impl ForgeQueryWorkflowContributionAuthoring {
    pub fn promotion_eligible_merge_conflict_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        lowered_merge: LoweredMergeWorkflowDeclaration,
        relational_inspection: RelationalMergeInspectionArtifact,
    ) -> Self {
        let preview_session_identity = lowered_merge
            .declaration()
            .binding()
            .preview_session_identity()
            .expect(
                "promotion-eligible merge conflict inspection requires preview foundation binding",
            )
            .clone();
        Self::with_runtime_and_inspection_semantics(
            ForgeQueryWorkflowContributionPosture::PromotionEligible,
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
            ForgeQueryWorkflowInspectionSemantics::merge_conflict(
                lowered_merge,
                relational_inspection,
            ),
        )
    }

    pub fn confirmation_required_merge_conflict_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        lowered_merge: LoweredMergeWorkflowDeclaration,
        relational_inspection: RelationalMergeInspectionArtifact,
    ) -> Self {
        let runtime_snapshot_token = lowered_merge
            .declaration()
            .binding()
            .runtime_snapshot_token()
            .expect(
                "confirmation-required merge conflict inspection requires runtime basis binding",
            )
            .to_string();
        Self::with_runtime_and_inspection_semantics(
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
            ForgeQueryWorkflowInspectionSemantics::merge_conflict(
                lowered_merge,
                relational_inspection,
            ),
        )
    }

    pub fn confirmation_required_post_merge_merge_outcome_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        lowered_merge: LoweredMergeWorkflowDeclaration,
    ) -> Self {
        let runtime_snapshot_token = lowered_merge
            .declaration()
            .binding()
            .runtime_snapshot_token()
            .expect("post-merge merge outcome inspection requires runtime basis binding")
            .to_string();
        Self::with_runtime_and_inspection_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                    runtime_snapshot_token,
                ),
                WorkflowDeclarationFamily::PostMergeInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowInspectionSemantics::post_merge_from_merge(lowered_merge),
        )
    }

    pub fn confirmation_required_post_merge_writeback_outcome_inspection(
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        lowered_writeback: QueryWritebackDeclaration,
    ) -> Self {
        let runtime_snapshot_token = lowered_writeback
            .declaration()
            .binding()
            .runtime_snapshot_token()
            .expect("post-merge writeback inspection requires runtime basis binding")
            .to_string();
        Self::with_runtime_and_inspection_semantics(
            ForgeQueryWorkflowContributionPosture::ConfirmationRequired,
            semantic_code,
            detail,
            ForgeQueryWorkflowRuntimeSemantics::new(
                ForgeQueryWorkflowRuntimeBindingSemantics::runtime_preflight(
                    runtime_snapshot_token,
                ),
                WorkflowDeclarationFamily::PostMergeInspectionNarrow,
                WorkflowAuthorityTargetFamily::QueryInspection,
                WorkflowCostClass::InspectionNarrow,
                WorkflowBudgetClass::InspectionBounded,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            ForgeQueryWorkflowInspectionSemantics::post_merge_from_writeback(lowered_writeback),
        )
    }

    fn with_runtime_and_inspection_semantics(
        posture: ForgeQueryWorkflowContributionPosture,
        semantic_code: impl Into<String>,
        detail: impl Into<String>,
        runtime_semantics: ForgeQueryWorkflowRuntimeSemantics,
        inspection_semantics: ForgeQueryWorkflowInspectionSemantics,
    ) -> Self {
        Self {
            payload: ForgeQueryWorkflowContributionPayload::with_runtime_and_inspection_semantics(
                posture,
                semantic_code,
                detail,
                Some(runtime_semantics),
                Some(inspection_semantics),
            ),
        }
    }
}
