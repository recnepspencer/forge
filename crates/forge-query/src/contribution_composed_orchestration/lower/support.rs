use crate::application::{
    ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryDeclarationEntryContributionEvidenceRecord,
    ForgeQueryDeclarationEntryContributionTargetFamily,
};
use crate::contribution_composed_orchestration::artifact::ForgeQueryContributionComposedContribution;
use crate::contribution_composed_orchestration::intent_result::{
    ForgeQueryContributionComposedIntentClassification,
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedIntentResult, ForgeQueryContributionComposedIntentStageResult,
};
use crate::domain_capabilities::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityPayload,
    ForgeQueryDomainCapabilityTargetBinding,
};

pub(super) fn denied_result(
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    evaluation: ForgeQueryContributionComposedIntentStageResult,
) -> ForgeQueryContributionComposedIntentResult {
    ForgeQueryContributionComposedIntentResult::new(
        request,
        evaluation,
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentClassification::Denied,
        None,
    )
}

pub(super) fn stale_result(
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    detail: String,
) -> ForgeQueryContributionComposedIntentResult {
    ForgeQueryContributionComposedIntentResult::new(
        request,
        ForgeQueryContributionComposedIntentStageResult::stale(detail),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentClassification::Stale,
        None,
    )
}

pub(super) fn rebind_required_result(
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    detail: String,
) -> ForgeQueryContributionComposedIntentResult {
    ForgeQueryContributionComposedIntentResult::new(
        request,
        ForgeQueryContributionComposedIntentStageResult::rebind_required(detail),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentClassification::RebindRequired,
        None,
    )
}

pub(super) fn failed_result(
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    detail: String,
) -> ForgeQueryContributionComposedIntentResult {
    ForgeQueryContributionComposedIntentResult::new(
        request,
        ForgeQueryContributionComposedIntentStageResult::failed(detail),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentStageResult::not_attempted(),
        ForgeQueryContributionComposedIntentClassification::Failed,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn retained_after_admission_result(
    request: ForgeQueryContributionComposedIntentRequestDescriptor,
    evaluation: ForgeQueryContributionComposedIntentStageResult,
    admission: ForgeQueryContributionComposedIntentStageResult,
    materialization: ForgeQueryContributionComposedIntentStageResult,
    contribution: ForgeQueryContributionComposedContribution,
) -> ForgeQueryContributionComposedIntentResult {
    ForgeQueryContributionComposedIntentResult::new(
        request,
        evaluation,
        admission,
        materialization,
        ForgeQueryContributionComposedIntentClassification::MaterializationFailedAfterAdmission,
        Some(contribution),
    )
}

pub(super) fn evidence_from_admitted<P>(
    admitted: &crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> ForgeQueryDeclarationEntryContributionEvidence
where
    P: ForgeQueryDomainCapabilityPayload,
{
    let payload = admitted.payload().payload();
    let record = ForgeQueryDeclarationEntryContributionEvidenceRecord {
        target_family: ForgeQueryDeclarationEntryContributionTargetFamily::DeclarationBound,
        target_digest: admitted.payload().target().target_digest().to_string(),
        target_binding_digest: admitted.payload().target().binding_digest().to_string(),
        evidence_digest: admitted.admitted_digest(),
        posture_label: payload.posture_label().to_string(),
        semantic_code: payload.semantic_code().to_string(),
        detail: payload.detail().to_string(),
        decision_stage: None,
    };
    match payload.category() {
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::Admission => {
            ForgeQueryDeclarationEntryContributionEvidence::Admission(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::SupportTraceability => {
            ForgeQueryDeclarationEntryContributionEvidence::SupportTraceability(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ExplanationInspection => {
            ForgeQueryDeclarationEntryContributionEvidence::ExplanationInspection(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::WorkflowPreview => {
            ForgeQueryDeclarationEntryContributionEvidence::WorkflowPreview(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ContinuityLineage => {
            ForgeQueryDeclarationEntryContributionEvidence::ContinuityLineage(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::ConsequenceAftermath => {
            ForgeQueryDeclarationEntryContributionEvidence::ConsequenceAftermath(record)
        }
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::InvariantCapability => {
            ForgeQueryDeclarationEntryContributionEvidence::SupportTraceability(record)
        }
    }
}
