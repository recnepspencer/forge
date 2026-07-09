#![allow(dead_code)]

use crate::domain_capabilities::identity::domain_capability_scope_encoder;
use crate::evidence_identity::{WorthQueryEvidenceIdentity, WorthQueryEvidenceTag};
use worth_relational::facade::merge::RelationalMergeInspectionArtifact;
use worth_relational::facade::transactions::AspectFieldPatch;
use worth_runtime_bridge::facade::BridgePreviewSessionIdentity;

use crate::basis::ExecutionPreflightBundle;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::workflow::{
    LoweredMergeWorkflowDeclaration, MergeLoweringInput, MutationLoweringInput,
    QueryWritebackDeclaration, WorkflowAuthorityTargetFamily, WorkflowBudgetClass,
    WorkflowCostClass, WorkflowDeclarationFamily, WorkflowFreshnessPolicy,
    WorkflowPreviewEvaluationClass, WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowRuntimeBindingSemantics {
    RuntimePreflight {
        runtime_snapshot_identity: WorthQuerySnapshotIdentity,
    },
    RuntimePreflightBundle {
        preflight: ExecutionPreflightBundle,
    },
    PreviewFoundation {
        preview_session_identity: BridgePreviewSessionIdentity,
        evaluation_class: WorkflowPreviewEvaluationClass,
    },
}

impl WorthQueryWorkflowRuntimeBindingSemantics {
    pub fn runtime_preflight_snapshot_identity(
        runtime_snapshot_identity: WorthQuerySnapshotIdentity,
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

    pub fn runtime_snapshot_identity(&self) -> Option<WorthQuerySnapshotIdentity> {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_identity,
            } => Some(runtime_snapshot_identity.clone()),
            Self::RuntimePreflightBundle { preflight } => {
                Some(WorthQuerySnapshotIdentity::preview(
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

    pub(crate) fn semantics_identity(&self) -> WorthQueryEvidenceIdentity {
        match self {
            Self::RuntimePreflight {
                runtime_snapshot_identity,
            } => domain_capability_scope_encoder("worth_query_workflow_runtime_binding_v1")
                .field_shape(WorthQueryEvidenceTag::new("kind"), "runtime_preflight")
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("runtime_snapshot"),
                    &runtime_snapshot_identity.evidence_identity(),
                )
                .seal(),
            Self::RuntimePreflightBundle { preflight } => {
                domain_capability_scope_encoder("worth_query_workflow_runtime_binding_v1")
                    .field_shape(
                        WorthQueryEvidenceTag::new("kind"),
                        "runtime_preflight_bundle",
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("plan"),
                        &preflight.plan().query().plan_digest().evidence_identity(),
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("canonical_query"),
                        &preflight
                            .plan()
                            .query()
                            .canonical_query_digest()
                            .evidence_identity(),
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("basis_proof"),
                        &preflight.basis().proof().digest().evidence_identity(),
                    )
                    .seal()
            }
            Self::PreviewFoundation {
                preview_session_identity,
                evaluation_class,
            } => domain_capability_scope_encoder("worth_query_workflow_runtime_binding_v1")
                .field_shape(WorthQueryEvidenceTag::new("kind"), "preview_foundation")
                .field_bridge_authority_identity(
                    WorthQueryEvidenceTag::new("preview_session"),
                    &preview_session_identity.bridge_trust_boundary(),
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("evaluation_class"),
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
pub enum WorthQueryWorkflowLoweringSemantics {
    Mutation {
        authority_binding_identity: WorthQueryEvidenceIdentity,
        input: MutationLoweringInput,
    },
    Merge {
        input: MergeLoweringInput,
    },
    Writeback {
        input: WritebackLoweringInput,
    },
}

impl WorthQueryWorkflowLoweringSemantics {
    pub fn mutation(
        authority_binding_identity: WorthQueryEvidenceIdentity,
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

    pub fn mutation_parts(&self) -> Option<(&WorthQueryEvidenceIdentity, &MutationLoweringInput)> {
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

    pub(crate) fn semantics_identity(&self) -> WorthQueryEvidenceIdentity {
        match self {
            Self::Mutation {
                authority_binding_identity,
                input,
            } => {
                let mut encoder =
                    domain_capability_scope_encoder("worth_query_workflow_lowering_v1")
                        .field_shape(WorthQueryEvidenceTag::new("kind"), "mutation")
                        .field_evidence_identity(
                            WorthQueryEvidenceTag::new("authority_binding"),
                            authority_binding_identity,
                        );
                encoder = match input {
                    MutationLoweringInput::IntentReconciliation {
                        entity_id,
                        desired_aspect_fields,
                    } => encoder
                        .field_shape(
                            WorthQueryEvidenceTag::new("input_kind"),
                            "intent_reconciliation",
                        )
                        .field_usize(
                            WorthQueryEvidenceTag::new("partition_id"),
                            entity_id.partition_id.0 as usize,
                        )
                        .field_usize(
                            WorthQueryEvidenceTag::new("local_slot"),
                            entity_id.local_slot.0 as usize,
                        )
                        .field_usize(
                            WorthQueryEvidenceTag::new("generation"),
                            entity_id.generation.0 as usize,
                        )
                        .field_shape(
                            WorthQueryEvidenceTag::new("desired_aspect_fields"),
                            &aspect_field_patch_identity_text(desired_aspect_fields),
                        ),
                };
                encoder.seal()
            }
            Self::Merge { input } => {
                domain_capability_scope_encoder("worth_query_workflow_lowering_v1")
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "merge")
                    .field_shape(
                        WorthQueryEvidenceTag::new("intent"),
                        input.intent().as_str(),
                    )
                    .field_shape(
                        WorthQueryEvidenceTag::new("target_branch"),
                        &input.target_branch().0,
                    )
                    .field_shape(
                        WorthQueryEvidenceTag::new("source_branch"),
                        &input.source_branch().0,
                    )
                    .seal()
            }
            Self::Writeback { input } => {
                domain_capability_scope_encoder("worth_query_workflow_lowering_v1")
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "writeback")
                    .field_shape(
                        WorthQueryEvidenceTag::new("family"),
                        input.family().as_str(),
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
pub enum WorthQueryWorkflowInspectionSemantics {
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

impl WorthQueryWorkflowInspectionSemantics {
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

    pub(crate) fn semantics_identity(&self) -> WorthQueryEvidenceIdentity {
        match self {
            Self::MergeConflict {
                lowered_merge,
                relational_inspection,
            } => domain_capability_scope_encoder("worth_query_workflow_inspection_v1")
                .field_shape(WorthQueryEvidenceTag::new("kind"), "merge_conflict")
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("lowered_merge"),
                    lowered_merge.lowering_identity(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("relational_inspection"),
                    &workflow_external_relational_inspection_reference_identity(
                        relational_inspection.artifact_digest(),
                    ),
                )
                .seal(),
            Self::PostMergeFromMerge { lowered_merge } => {
                domain_capability_scope_encoder("worth_query_workflow_inspection_v1")
                    .field_shape(WorthQueryEvidenceTag::new("kind"), "post_merge_from_merge")
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("lowered_merge"),
                        lowered_merge.lowering_identity(),
                    )
                    .seal()
            }
            Self::PostMergeFromWriteback { lowered_writeback } => {
                domain_capability_scope_encoder("worth_query_workflow_inspection_v1")
                    .field_shape(
                        WorthQueryEvidenceTag::new("kind"),
                        "post_merge_from_writeback",
                    )
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("lowered_writeback"),
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
pub struct WorthQueryWorkflowRuntimeSemantics {
    binding: WorthQueryWorkflowRuntimeBindingSemantics,
    declaration_family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    cost_class: WorkflowCostClass,
    budget_class: WorkflowBudgetClass,
    freshness_policy: WorkflowFreshnessPolicy,
}

impl WorthQueryWorkflowRuntimeSemantics {
    pub fn new(
        binding: WorthQueryWorkflowRuntimeBindingSemantics,
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

    pub fn binding(&self) -> &WorthQueryWorkflowRuntimeBindingSemantics {
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

    pub(crate) fn semantics_identity(&self) -> WorthQueryEvidenceIdentity {
        domain_capability_scope_encoder("worth_query_workflow_runtime_semantics_v1")
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("binding"),
                &self.binding.semantics_identity(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("declaration_family"),
                self.declaration_family.as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("authority_target_family"),
                self.authority_target_family.as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("cost_class"),
                self.cost_class.as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("budget_class"),
                self.budget_class.as_str(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("freshness_policy"),
                self.freshness_policy.as_str(),
            )
            .seal()
    }

    pub(crate) fn semantics_for_reporting(&self) -> String {
        self.semantics_identity().as_str().to_string()
    }
}

fn aspect_field_patch_identity_text(patch: &AspectFieldPatch) -> String {
    patch
        .to_canonical_bytes()
        .map(|bytes| {
            bytes
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
        .unwrap_or_else(|_| "canonical_patch_serialization_failed".to_string())
}

fn workflow_external_relational_inspection_reference_identity(
    external_artifact_digest: &str,
) -> WorthQueryEvidenceIdentity {
    domain_capability_scope_encoder(
        "worth_query_workflow_external_relational_inspection_reference_v1",
    )
    .field_shape(
        WorthQueryEvidenceTag::new("external_artifact_digest"),
        external_artifact_digest,
    )
    .seal()
}
