use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
    ForgeQueryWorkflowInspectionSemantics, ForgeQueryWorkflowLoweringSemantics,
    ForgeQueryWorkflowRuntimeBindingSemantics, ForgeQueryWorkflowRuntimeSemantics,
};
use crate::domain_capabilities::targets::ForgeQueryDomainCapabilityTargetBinding;
use crate::workflow::WorkflowPreviewEvaluationClass;

pub(super) fn workflow_source_label<T>(
    target: &T,
    payload: &ForgeQueryWorkflowContributionPayload,
) -> String
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    if let Some((name, ..)) = target.semantics().intent_declaration() {
        return name.to_string();
    }
    if let Some((family, entrypoint, ..)) = target.semantics().admitted_intent_plan() {
        return format!(
            "{}:{}:{}",
            payload.semantic_code(),
            family.as_str(),
            entrypoint.as_str()
        );
    }
    payload.semantic_code().to_string()
}

pub(super) fn workflow_runtime_semantics_match_posture(
    posture: ForgeQueryWorkflowContributionPosture,
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
) -> bool {
    match posture {
        ForgeQueryWorkflowContributionPosture::PreviewOnly => matches!(
            runtime_semantics.binding(),
            ForgeQueryWorkflowRuntimeBindingSemantics::PreviewFoundation {
                evaluation_class: WorkflowPreviewEvaluationClass::ReadOnly,
                ..
            }
        ),
        ForgeQueryWorkflowContributionPosture::PromotionEligible => matches!(
            runtime_semantics.binding(),
            ForgeQueryWorkflowRuntimeBindingSemantics::PreviewFoundation {
                evaluation_class: WorkflowPreviewEvaluationClass::PromotionEligible,
                ..
            }
        ),
        ForgeQueryWorkflowContributionPosture::ConfirmationRequired => matches!(
            runtime_semantics.binding(),
            ForgeQueryWorkflowRuntimeBindingSemantics::RuntimePreflight { .. }
                | ForgeQueryWorkflowRuntimeBindingSemantics::RuntimePreflightBundle { .. }
        ),
        ForgeQueryWorkflowContributionPosture::DiscardRequired => matches!(
            runtime_semantics.binding(),
            ForgeQueryWorkflowRuntimeBindingSemantics::PreviewFoundation {
                evaluation_class: WorkflowPreviewEvaluationClass::PromotionEligible,
                ..
            }
        ),
    }
}

pub(super) fn missing_workflow_runtime_semantics_denial(
    operation_label: &'static str,
    payload: &ForgeQueryWorkflowContributionPayload,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!(
            "{operation_label} requires runtime workflow semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

pub(super) fn missing_workflow_lowering_semantics_denial(
    operation_label: &'static str,
    payload: &ForgeQueryWorkflowContributionPayload,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!(
            "{operation_label} requires workflow lowering semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

pub(super) fn inconsistent_workflow_runtime_semantics_denial(
    operation_label: &'static str,
    payload: &ForgeQueryWorkflowContributionPayload,
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!(
            "{operation_label} got workflow runtime semantics `{}:{}` that do not match `{}` posture",
            runtime_semantics.declaration_family().as_str(),
            runtime_semantics.authority_target_family().as_str(),
            payload.posture().as_str()
        ),
    )
}

pub(super) fn inconsistent_workflow_lowering_semantics_denial(
    operation_label: &'static str,
    payload: &ForgeQueryWorkflowContributionPayload,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics,
        "workflow-preview",
        target_kind,
        request_digest,
        format!(
            "{operation_label} got workflow lowering semantics that do not match `{}` workflow runtime semantics",
            payload.posture().as_str()
        ),
    )
}

pub(super) fn workflow_lowering_semantics_match_runtime(
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
    lowering_semantics: &ForgeQueryWorkflowLoweringSemantics,
) -> bool {
    match lowering_semantics {
        ForgeQueryWorkflowLoweringSemantics::Mutation { .. } => {
            runtime_semantics.declaration_family()
                == &crate::workflow::WorkflowDeclarationFamily::MutationLoweringNarrow
                && runtime_semantics.authority_target_family()
                    == &crate::workflow::WorkflowAuthorityTargetFamily::RelationalMutation
        }
        ForgeQueryWorkflowLoweringSemantics::Merge { .. } => {
            runtime_semantics.declaration_family()
                == &crate::workflow::WorkflowDeclarationFamily::MergeLoweringNarrow
                && runtime_semantics.authority_target_family()
                    == &crate::workflow::WorkflowAuthorityTargetFamily::RelationalMerge
        }
        ForgeQueryWorkflowLoweringSemantics::Writeback { .. } => {
            runtime_semantics.declaration_family()
                == &crate::workflow::WorkflowDeclarationFamily::WritebackLoweringNarrow
                && runtime_semantics.authority_target_family()
                    == &crate::workflow::WorkflowAuthorityTargetFamily::BridgeWriteback
        }
    }
}

pub(super) fn workflow_inspection_semantics_match_runtime(
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
    inspection_semantics: &ForgeQueryWorkflowInspectionSemantics,
) -> bool {
    match inspection_semantics {
        ForgeQueryWorkflowInspectionSemantics::MergeConflict { .. } => {
            runtime_semantics.declaration_family()
                == &crate::workflow::WorkflowDeclarationFamily::ConflictInspectionNarrow
                && runtime_semantics.authority_target_family()
                    == &crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection
        }
        ForgeQueryWorkflowInspectionSemantics::PostMergeFromMerge { .. }
        | ForgeQueryWorkflowInspectionSemantics::PostMergeFromWriteback { .. } => {
            runtime_semantics.declaration_family()
                == &crate::workflow::WorkflowDeclarationFamily::PostMergeInspectionNarrow
                && runtime_semantics.authority_target_family()
                    == &crate::workflow::WorkflowAuthorityTargetFamily::QueryInspection
        }
    }
}
