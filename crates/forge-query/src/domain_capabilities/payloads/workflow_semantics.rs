use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use forge_relational::facade::merge::RelationalMergeInspectionArtifact;
use forge_runtime_bridge::facade::BridgePreviewSessionIdentity;

use crate::basis::ExecutionPreflightBundle;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::workflow::{
    LoweredMergeWorkflowDeclaration, MergeLoweringInput, MutationLoweringInput,
    QueryWritebackDeclaration, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass, WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowRuntimeBindingSemantics {
    RuntimePreflight {
        runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
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
    pub fn runtime_preflight_snapshot_identity(
        runtime_snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        Self::RuntimePreflight {
            runtime_snapshot_identity,
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

    pub fn runtime_snapshot_identity(&self) -> Option<ForgeQuerySnapshotIdentity> {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_identity,
            } => Some(runtime_snapshot_identity.clone()),
            Self::RuntimePreflightBundle { preflight } => {
                Some(ForgeQuerySnapshotIdentity::preview(
                    preflight.basis().identity().snapshot_identity().clone(),
                ))
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
    ) -> Option<(
        &BridgePreviewSessionIdentity,
        WorkflowPreviewEvaluationClass,
    )> {
        match self {
            Self::PreviewFoundation {
                preview_session_identity,
                evaluation_class,
            } => Some((preview_session_identity, evaluation_class.clone())),
            Self::RuntimePreflight { .. } | Self::RuntimePreflightBundle { .. } => None,
        }
    }

    pub(crate) fn semantics_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_identity,
            } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_runtime_binding_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "runtime_preflight")
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("runtime_snapshot"),
                &runtime_snapshot_identity.evidence_identity(),
            )
            .seal(),
            Self::RuntimePreflightBundle { preflight } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_runtime_binding_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "runtime_preflight_bundle")
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("plan"),
                &preflight.plan().query().plan_digest().evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("canonical_query"),
                &preflight
                    .plan()
                    .query()
                    .canonical_query_digest()
                    .evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("basis_proof"),
                &preflight.basis().proof().digest().evidence_identity(),
            )
            .seal(),
            Self::PreviewFoundation {
                preview_session_identity,
                evaluation_class,
            } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_runtime_binding_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "preview_foundation")
            .field_bridge_identity(
                ForgeQueryEvidenceTag::new("preview_session"),
                &preview_session_identity.evidence_identity(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("evaluation_class"),
                evaluation_class.as_str(),
            )
            .seal(),
        }
    }

    pub(crate) fn semantics_for_reporting(&self) -> String {
        self.semantics_identity().as_str().to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryWorkflowLoweringSemantics {
    Mutation {
        authority_binding_identity: ForgeQueryEvidenceIdentity,
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
        authority_binding_identity: ForgeQueryEvidenceIdentity,
        input: MutationLoweringInput,
    ) -> Self {
        Self::Mutation {
            authority_binding_identity,
            input,
        }
    }

    pub fn merge(input: MergeLoweringInput) -> Self {
        Self::Merge { input }
    }

    pub fn writeback(input: WritebackLoweringInput) -> Self {
        Self::Writeback { input }
    }

    pub fn mutation_parts(&self) -> Option<(&ForgeQueryEvidenceIdentity, &MutationLoweringInput)> {
        match self {
            Self::Mutation {
                authority_binding_identity,
                input,
            } => Some((authority_binding_identity, input)),
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

    pub(crate) fn semantics_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::Mutation {
                authority_binding_identity,
                input,
            } => {
                let input_label = match input {
                    MutationLoweringInput::IntentReconciliation {
                        entity_id,
                        desired_aspect_fields_external_json,
                    } => format!(
                        "intent_reconciliation:{entity_id:?}:{}",
                        serde_json::to_string(desired_aspect_fields_external_json)
                            .unwrap_or_else(|_| "serialization_failed".to_string())
                    ),
                };
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_workflow_lowering_v1",
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), "mutation")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authority_binding"),
                    authority_binding_identity,
                )
                .field_shape(ForgeQueryEvidenceTag::new("input"), &input_label)
                .seal()
            }
            Self::Merge { input } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_lowering_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "merge")
            .field_shape(ForgeQueryEvidenceTag::new("intent"), input.intent().as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("target_branch"),
                &input.target_branch().0,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("source_branch"),
                &input.source_branch().0,
            )
            .seal(),
            Self::Writeback { input } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_lowering_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "writeback")
            .field_shape(ForgeQueryEvidenceTag::new("family"), input.family().as_str())
            .seal(),
        }
    }

    pub(crate) fn semantics_for_reporting(&self) -> String {
        self.semantics_identity().as_str().to_string()
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

    pub(crate) fn semantics_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::MergeConflict {
                lowered_merge,
                relational_inspection,
            } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_inspection_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "merge_conflict")
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("lowered_merge"),
                lowered_merge.lowering_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("relational_inspection"),
                &workflow_external_relational_inspection_reference_identity(
                    relational_inspection.artifact_digest(),
                ),
            )
            .seal(),
            Self::PostMergeFromMerge { lowered_merge } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_inspection_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), "post_merge_from_merge")
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("lowered_merge"),
                lowered_merge.lowering_identity(),
            )
            .seal(),
            Self::PostMergeFromWriteback { lowered_writeback } => {
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_workflow_inspection_v1",
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), "post_merge_from_writeback")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("lowered_writeback"),
                    lowered_writeback.lowering_identity(),
                )
                .seal()
            }
        }
    }

    pub(crate) fn semantics_for_reporting(&self) -> String {
        self.semantics_identity().as_str().to_string()
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

    pub(crate) fn semantics_identity(&self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "forge_query_workflow_runtime_semantics_v1",
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("binding"),
                &self.binding.semantics_identity(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("declaration_family"),
                self.declaration_family.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("authority_target_family"),
                self.authority_target_family.as_str(),
            )
            .field_shape(ForgeQueryEvidenceTag::new("cost_class"), self.cost_class.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("budget_class"), self.budget_class.as_str())
            .field_shape(
                ForgeQueryEvidenceTag::new("freshness_policy"),
                self.freshness_policy.as_str(),
            )
            .seal()
    }

    pub(crate) fn semantics_for_reporting(&self) -> String {
        self.semantics_identity().as_str().to_string()
    }
}

fn workflow_external_relational_inspection_reference_identity(
    external_artifact_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_workflow_external_relational_inspection_reference_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("external_artifact_digest"),
            external_artifact_digest,
        )
        .seal()
}
