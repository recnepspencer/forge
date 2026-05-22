use forge_proof::TransitionOutcome;
use forge_runtime_bridge::facade::{
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity,
};

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryMaterializationReadyWorkflowContribution,
};
use crate::identity::{hash_parts, CanonicalQueryDigest, ValidatedQueryDigest};
use crate::preview::{
    admit_contributed_preview_workflow_foundation,
    materialize_contributed_preview_workflow_foundation_artifact,
    AdmittedPreviewWorkflowFoundation, PreviewEvaluationClass, PreviewWorkflowFoundationArtifact,
    PreviewWorkflowFoundationRequest,
};

use super::workflow_semantics::{
    inconsistent_workflow_runtime_semantics_denial, missing_workflow_runtime_semantics_denial,
    workflow_runtime_semantics_match_posture, workflow_source_label,
};

pub fn materialize_query_preview_workflow_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<PreviewWorkflowFoundationArtifact>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    match prepare_preview_workflow_foundation(contribution) {
        Ok((artifact, ..)) => TransitionOutcome::Success(artifact),
        Err(denial) => TransitionOutcome::Denied(denial),
    }
}

pub fn materialize_admitted_preview_workflow_foundation<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<AdmittedPreviewWorkflowFoundation>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    match prepare_preview_workflow_foundation(contribution) {
        Ok((artifact, target_kind, request_digest)) => {
            match admit_contributed_preview_workflow_foundation(artifact) {
                Ok(foundation) => TransitionOutcome::Success(foundation),
                Err(error) => {
                    let failure_class = error.failure_class().clone();
                    TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
                        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                        "workflow-preview",
                        target_kind,
                        &request_digest,
                        format!(
                            "preview workflow foundation admission denied with `{:?}`: {}",
                            failure_class,
                            error.message()
                        ),
                    ))
                }
            }
        }
        Err(denial) => TransitionOutcome::Denied(denial),
    }
}

pub trait ForgeQueryWorkflowPreviewMaterializationTarget:
    ForgeQueryDomainCapabilityTargetBinding + private::Sealed
{
}

impl ForgeQueryWorkflowPreviewMaterializationTarget
    for ForgeQueryDeclarationBoundContributionTarget
{
}

impl ForgeQueryWorkflowPreviewMaterializationTarget
    for ForgeQueryAdmittedPlanBoundContributionTarget
{
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for ForgeQueryDeclarationBoundContributionTarget {}
impl private::Sealed for ForgeQueryAdmittedPlanBoundContributionTarget {}

fn prepare_preview_workflow_foundation<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> Result<
    (
        PreviewWorkflowFoundationArtifact,
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
        String,
    ),
    ForgeQueryDomainCapabilityProgressionDenial,
>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let target = domain_contribution.target();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return Err(missing_workflow_runtime_semantics_denial(
            "preview workflow artifact materialization",
            payload,
            target.kind(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return Err(inconsistent_workflow_runtime_semantics_denial(
            "preview workflow artifact materialization",
            payload,
            runtime_semantics,
            target.kind(),
            domain_contribution.request_digest(),
        ));
    }
    let Some((preview_session_identity, evaluation_class)) =
        runtime_semantics.binding().preview_foundation_binding()
    else {
        return Err(unsupported_preview_binding_denial(
            payload,
            target.kind(),
            domain_contribution.request_digest(),
        ));
    };

    let source_label = workflow_source_label(target, payload);
    let preview_session_identity = BridgePreviewSessionIdentity::new(preview_session_identity);
    let request_family = preview_request_family(payload.posture());
    let binding_digest = target.binding_digest().to_string();
    let canonical_query_digest = CanonicalQueryDigest::from_parts(&[
        "forge_query_domain_preview_query_v1".to_string(),
        format!("source:{source_label}"),
        format!("binding:{binding_digest}"),
        format!("preview_session:{}", preview_session_identity.as_str()),
        format!("evaluation:{}", evaluation_class.as_str()),
    ]);
    let validated_query_digest = ValidatedQueryDigest::from_parts(&[
        "forge_query_domain_preview_validated_query_v1".to_string(),
        format!("canonical:{}", canonical_query_digest.as_str()),
    ]);
    let declaration_identity = BridgePreviewSessionDeclarationIdentity::new(format!(
        "domain-preview-declaration:{}:{}",
        payload.semantic_code(),
        binding_digest
    ));
    let declaration_digest = hash_parts(&[
        "forge_query_domain_preview_declaration_v1".to_string(),
        format!("identity:{}", declaration_identity.as_str()),
        format!("canonical:{}", canonical_query_digest.as_str()),
        format!("validated:{}", validated_query_digest.as_str()),
    ]);
    let artifact = materialize_contributed_preview_workflow_foundation_artifact(
        binding_digest,
        canonical_query_digest,
        validated_query_digest,
        request_family,
        preview_session_identity,
        declaration_identity,
        declaration_digest,
        preview_evaluation_class(evaluation_class),
        0,
    );

    Ok((
        artifact,
        target.kind(),
        domain_contribution.request_digest().to_string(),
    ))
}

fn preview_request_family(
    posture: ForgeQueryWorkflowContributionPosture,
) -> PreviewWorkflowFoundationRequest {
    match posture {
        ForgeQueryWorkflowContributionPosture::PreviewOnly
        | ForgeQueryWorkflowContributionPosture::PromotionEligible => {
            PreviewWorkflowFoundationRequest::compare_basis_pair()
        }
        ForgeQueryWorkflowContributionPosture::DiscardRequired => {
            PreviewWorkflowFoundationRequest::deferred_mutation_writeback()
        }
        ForgeQueryWorkflowContributionPosture::ConfirmationRequired => {
            PreviewWorkflowFoundationRequest::compare_basis_pair()
        }
    }
}

fn preview_evaluation_class(
    evaluation_class: crate::workflow::WorkflowPreviewEvaluationClass,
) -> PreviewEvaluationClass {
    match evaluation_class {
        crate::workflow::WorkflowPreviewEvaluationClass::ReadOnly => {
            PreviewEvaluationClass::read_only()
        }
        crate::workflow::WorkflowPreviewEvaluationClass::PromotionEligible => {
            PreviewEvaluationClass::promotion_eligible()
        }
    }
}

fn unsupported_preview_binding_denial(
    payload: &ForgeQueryWorkflowContributionPayload,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "workflow-preview",
        target_kind,
        request_digest,
        format!(
            "preview workflow artifact materialization only supports preview-bound workflow semantics; got `{}`",
            payload.posture().as_str()
        ),
    )
}
