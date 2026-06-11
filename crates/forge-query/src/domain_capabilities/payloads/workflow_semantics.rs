use forge_relational::facade::merge::RelationalMergeInspectionArtifact;
use forge_runtime_bridge::facade::BridgePreviewSessionIdentity;

use crate::basis::ExecutionPreflightBundle;
use crate::workflow::{
    LoweredMergeWorkflowDeclaration, MergeLoweringInput, MutationLoweringInput,
    QueryWritebackDeclaration, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass, WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowRuntimeBindingSemantics {
    RuntimePreflight {
        runtime_snapshot_token: String,
    },
    RuntimePreflightBundle {
        preflight: ExecutionPreflightBundle,
    },
    PreviewFoundation {
        preview_session_identity: BridgePreviewSessionIdentity,
        evaluation_class: WorkflowPreviewEvaluationClass,
    },
}

impl ForgeQueryWorkflowRuntimeBindingSemantics {
    pub fn runtime_preflight(runtime_snapshot_token: impl Into<String>) -> Self {
        Self::RuntimePreflight {
            runtime_snapshot_token: runtime_snapshot_token.into(),
        }
    }

    pub fn runtime_preflight_bundle(preflight: ExecutionPreflightBundle) -> Self {
        Self::RuntimePreflightBundle { preflight }
    }

    pub fn preview_foundation(
        preview_session_identity: BridgePreviewSessionIdentity,
        evaluation_class: WorkflowPreviewEvaluationClass,
    ) -> Self {
        Self::PreviewFoundation {
            preview_session_identity,
            evaluation_class,
        }
    }

    pub fn runtime_snapshot_token(&self) -> Option<&str> {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_token,
            } => Some(runtime_snapshot_token.as_str()),
            Self::RuntimePreflightBundle { preflight } => {
                Some(preflight.basis().identity().snapshot_token())
            }
            Self::PreviewFoundation { .. } => None,
        }
    }

    pub fn runtime_preflight_bundle_ref(&self) -> Option<&ExecutionPreflightBundle> {
        match self {
            Self::RuntimePreflightBundle { preflight } => Some(preflight),
            Self::RuntimePreflight { .. } | Self::PreviewFoundation { .. } => None,
        }
    }

    pub fn preview_foundation_binding(
        &self,
    ) -> Option<(&BridgePreviewSessionIdentity, WorkflowPreviewEvaluationClass)> {
        match self {
            Self::PreviewFoundation {
                preview_session_identity,
                evaluation_class,
            } => Some((preview_session_identity, evaluation_class.clone())),
            Self::RuntimePreflight { .. } | Self::RuntimePreflightBundle { .. } => None,
        }
    }

    pub(crate) fn digest_fragment(&self) -> String {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_token,
            } => format!("runtime:{runtime_snapshot_token}"),
            Self::RuntimePreflightBundle { preflight } => format!(
                "runtime_preflight:{}:{}:{}",
                preflight.plan().query().plan_digest().as_str(),
                preflight.plan().query().canonical_query_digest().as_str(),
                preflight.basis().proof().digest().as_str()
            ),
            Self::PreviewFoundation {
                preview_session_identity,
                evaluation_class,
            } => format!(
                "preview:{}:{}",
                preview_session_identity.as_str(),
                evaluation_class.as_str()
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowLoweringSemantics {
    Mutation {
        authority_binding_digest: String,
        input: MutationLoweringInput,
    },
    Merge {
        input: MergeLoweringInput,
    },
    Writeback {
        input: WritebackLoweringInput,
    },
}

impl ForgeQueryWorkflowLoweringSemantics {
    pub fn mutation(
        authority_binding_digest: impl Into<String>,
        input: MutationLoweringInput,
    ) -> Self {
        Self::Mutation {
            authority_binding_digest: authority_binding_digest.into(),
            input,
        }
    }

    pub fn merge(input: MergeLoweringInput) -> Self {
        Self::Merge { input }
    }

    pub fn writeback(input: WritebackLoweringInput) -> Self {
        Self::Writeback { input }
    }

    pub fn mutation_parts(&self) -> Option<(&str, &MutationLoweringInput)> {
        match self {
            Self::Mutation {
                authority_binding_digest,
                input,
            } => Some((authority_binding_digest.as_str(), input)),
            _ => None,
        }
    }

    pub fn merge_input(&self) -> Option<&MergeLoweringInput> {
        match self {
            Self::Merge { input } => Some(input),
            _ => None,
        }
    }

    pub fn writeback_input(&self) -> Option<&WritebackLoweringInput> {
        match self {
            Self::Writeback { input } => Some(input),
            _ => None,
        }
    }

    pub(crate) fn digest_fragment(&self) -> String {
        match self {
            Self::Mutation {
                authority_binding_digest,
                input,
            } => {
                let input_digest = match input {
                    MutationLoweringInput::IntentReconciliation {
                        entity_id,
                        desired_aspect_fields_external_json,
                    } => format!(
                        "intent_reconciliation:{entity_id:?}:{}",
                        serde_json::to_string(desired_aspect_fields_external_json)
                            .unwrap_or_else(|_| "serialization_failed".to_string())
                    ),
                };
                format!("mutation:{authority_binding_digest}:{input_digest}")
            }
            Self::Merge { input } => format!(
                "merge:{}:{}:{}",
                input.intent().as_str(),
                input.target_branch().0,
                input.source_branch().0
            ),
            Self::Writeback { input } => {
                format!("writeback:{}", input.family().as_str())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowInspectionSemantics {
    MergeConflict {
        lowered_merge: LoweredMergeWorkflowDeclaration,
        relational_inspection: RelationalMergeInspectionArtifact,
    },
    PostMergeFromMerge {
        lowered_merge: LoweredMergeWorkflowDeclaration,
    },
    PostMergeFromWriteback {
        lowered_writeback: QueryWritebackDeclaration,
    },
}

impl ForgeQueryWorkflowInspectionSemantics {
    pub fn merge_conflict(
        lowered_merge: LoweredMergeWorkflowDeclaration,
        relational_inspection: RelationalMergeInspectionArtifact,
    ) -> Self {
        Self::MergeConflict {
            lowered_merge,
            relational_inspection,
        }
    }

    pub fn post_merge_from_merge(lowered_merge: LoweredMergeWorkflowDeclaration) -> Self {
        Self::PostMergeFromMerge { lowered_merge }
    }

    pub fn post_merge_from_writeback(lowered_writeback: QueryWritebackDeclaration) -> Self {
        Self::PostMergeFromWriteback { lowered_writeback }
    }

    pub fn lowered_merge_conflict(
        &self,
    ) -> Option<(
        &LoweredMergeWorkflowDeclaration,
        &RelationalMergeInspectionArtifact,
    )> {
        match self {
            Self::MergeConflict {
                lowered_merge,
                relational_inspection,
            } => Some((lowered_merge, relational_inspection)),
            _ => None,
        }
    }

    pub fn post_merge_from_merge_input(&self) -> Option<&LoweredMergeWorkflowDeclaration> {
        match self {
            Self::PostMergeFromMerge { lowered_merge } => Some(lowered_merge),
            _ => None,
        }
    }

    pub fn post_merge_from_writeback_input(&self) -> Option<&QueryWritebackDeclaration> {
        match self {
            Self::PostMergeFromWriteback { lowered_writeback } => Some(lowered_writeback),
            _ => None,
        }
    }

    pub(crate) fn digest_fragment(&self) -> String {
        match self {
            Self::MergeConflict {
                lowered_merge,
                relational_inspection,
            } => format!(
                "merge_conflict:{}:{}",
                lowered_merge.lowering_digest(),
                relational_inspection.artifact_digest()
            ),
            Self::PostMergeFromMerge { lowered_merge } => {
                format!("post_merge:merge:{}", lowered_merge.lowering_digest())
            }
            Self::PostMergeFromWriteback { lowered_writeback } => format!(
                "post_merge:writeback:{}",
                lowered_writeback.lowering_digest()
            ),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkflowRuntimeSemantics {
    binding: ForgeQueryWorkflowRuntimeBindingSemantics,
    declaration_family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    freshness_policy: WorkflowFreshnessPolicy,
}

impl ForgeQueryWorkflowRuntimeSemantics {
    pub fn new(
        binding: ForgeQueryWorkflowRuntimeBindingSemantics,
        declaration_family: WorkflowDeclarationFamily,
        authority_target_family: WorkflowAuthorityTargetFamily,
        cost_class: WorkflowCostClass,
        budget_class: WorkflowBudgetClass,
        freshness_policy: WorkflowFreshnessPolicy,
    ) -> Self {
        Self {
            binding,
            declaration_family,
            authority_target_family,
            cost_class,
            budget_class,
            freshness_policy,
        }
    }

    pub fn binding(&self) -> &ForgeQueryWorkflowRuntimeBindingSemantics {
        &self.binding
    }

    pub fn declaration_family(&self) -> &WorkflowDeclarationFamily {
        &self.declaration_family
    }

    pub fn authority_target_family(&self) -> &WorkflowAuthorityTargetFamily {
        &self.authority_target_family
    }

    pub fn cost_class(&self) -> &WorkflowCostClass {
        &self.cost_class
    }

    pub fn budget_class(&self) -> &WorkflowBudgetClass {
        &self.budget_class
    }

    pub fn freshness_policy(&self) -> &WorkflowFreshnessPolicy {
        &self.freshness_policy
    }

    pub(crate) fn digest_fragment(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.binding.digest_fragment(),
            self.declaration_family.as_str(),
            self.authority_target_family.as_str(),
            self.cost_class.as_str(),
            self.budget_class.as_str(),
            self.freshness_policy.as_str()
        )
    }
}
