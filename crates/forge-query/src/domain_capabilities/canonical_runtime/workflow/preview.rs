use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryWorkflowContributionPayload, ForgeQueryWorkflowContributionPosture,
    ForgeQueryWorkflowRuntimeSemantics,
};
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryMaterializationReadyWorkflowContribution,
};
use crate::preview::{
    admit_contributed_preview_workflow_foundation,
    materialize_contributed_preview_workflow_foundation_artifact,
    AdmittedPreviewWorkflowFoundation, PreviewEvaluationClass, PreviewWorkflowFoundationArtifact,
    PreviewWorkflowFoundationRequest,
};

use super::preview_identity::{
    canonical_query_digest_from_identity, preview_canonical_query_identity,
    preview_declaration_digest_identity, preview_declaration_identity,
    preview_validated_query_identity, sealed_preview_declaration_bridge_identity,
    validated_query_digest_from_identity,
};
use super::semantics::{
    inconsistent_workflow_runtime_semantics_denial, missing_workflow_runtime_semantics_denial,
    workflow_runtime_semantics_match_posture, workflow_source_label,
};

pub fn materialize_query_preview_workflow_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<PreviewWorkflowFoundationArtifact>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    match validated_preview_workflow_runtime_semantics(
        &contribution,
        "preview workflow artifact materialization",
    )
    .and_then(|runtime_semantics| {
        prepare_validated_preview_workflow_foundation(&contribution, runtime_semantics)
    }) {
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
    match validated_preview_workflow_runtime_semantics(
        &contribution,
        "preview workflow foundation admission",
    )
    .and_then(|runtime_semantics| {
        admit_validated_preview_workflow_foundation(&contribution, runtime_semantics)
    }) {
        Ok(foundation) => TransitionOutcome::Success(foundation),
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

pub(super) fn admit_validated_preview_workflow_foundation<T>(
    contribution: &ForgeQueryMaterializationReadyWorkflowContribution<T>,
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
) -> Result<AdmittedPreviewWorkflowFoundation, ForgeQueryDomainCapabilityProgressionDenial>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    let (artifact, target_kind, request_identity) =
        prepare_validated_preview_workflow_foundation(contribution, runtime_semantics)?;
    match admit_contributed_preview_workflow_foundation(artifact) {
        Ok(foundation) => Ok(foundation),
        Err(error) => {
            let failure_class = error.failure_class().clone();
            Err(ForgeQueryDomainCapabilityProgressionDenial::new(
                ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                "workflow-preview",
                target_kind,
                request_identity,
                format!(
                    "preview workflow foundation admission denied with `{:?}`: {}",
                    failure_class,
                    error.message()
                ),
            ))
        }
    }
}

pub(super) fn validated_preview_workflow_runtime_semantics<'a, T>(
    contribution: &'a ForgeQueryMaterializationReadyWorkflowContribution<T>,
    operation_label: &'static str,
) -> Result<&'a ForgeQueryWorkflowRuntimeSemantics, ForgeQueryDomainCapabilityProgressionDenial>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let target = domain_contribution.target();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return Err(missing_workflow_runtime_semantics_denial(
            operation_label,
            payload,
            target.kind(),
            domain_contribution.request_identity().clone(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return Err(inconsistent_workflow_runtime_semantics_denial(
            operation_label,
            payload,
            runtime_semantics,
            target.kind(),
            domain_contribution.request_identity().clone(),
        ));
    }

    Ok(runtime_semantics)
}

fn prepare_validated_preview_workflow_foundation<T>(
    contribution: &ForgeQueryMaterializationReadyWorkflowContribution<T>,
    runtime_semantics: &ForgeQueryWorkflowRuntimeSemantics,
) -> Result<
    (
        PreviewWorkflowFoundationArtifact,
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
        crate::ForgeQueryEvidenceIdentity,
    ),
    ForgeQueryDomainCapabilityProgressionDenial,
>
where
    T: ForgeQueryWorkflowPreviewMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let target = domain_contribution.target();
    let Some((preview_session_identity, evaluation_class)) =
        runtime_semantics.binding().preview_foundation_binding()
    else {
        return Err(unsupported_preview_binding_denial(
            payload,
            target.kind(),
            domain_contribution.request_identity().clone(),
        ));
    };

    let source_label = workflow_source_label(target, payload);
    let request_family = preview_request_family(payload.posture());
    let request_family_label = request_family.as_str();
    let binding_identity = target.binding_identity();
    let request_identity = domain_contribution.request_identity();
    let canonical_query_identity = preview_canonical_query_identity(
        &source_label,
        &binding_identity,
        request_identity,
        &preview_session_identity,
        &evaluation_class,
        request_family_label,
    );
    let validated_query_identity = preview_validated_query_identity(&canonical_query_identity);
    let canonical_query_digest = canonical_query_digest_from_identity(&canonical_query_identity);
    let validated_query_digest = validated_query_digest_from_identity(&validated_query_identity);
    let preview_declaration_identity = preview_declaration_identity(
        payload,
        &binding_identity,
        request_identity,
        &preview_session_identity,
        &evaluation_class,
        request_family_label,
    );
    let declaration_identity =
        sealed_preview_declaration_bridge_identity(&preview_declaration_identity);
    let declaration_digest_identity = preview_declaration_digest_identity(
        &preview_declaration_identity,
        &canonical_query_digest,
        &validated_query_digest,
    );
    let artifact = materialize_contributed_preview_workflow_foundation_artifact(
        binding_identity,
        canonical_query_digest,
        validated_query_digest,
        request_family,
        preview_session_identity.clone(),
        declaration_identity,
        declaration_digest_identity,
        preview_evaluation_class(evaluation_class),
        0,
    );

    Ok((
        artifact,
        target.kind(),
        domain_contribution.request_identity().clone(),
    ))
}

pub(super) fn preview_request_family(
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
    request_identity: crate::ForgeQueryEvidenceIdentity,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "workflow-preview",
        target_kind,
        request_identity,
        format!(
            "preview workflow artifact materialization only supports preview-bound workflow semantics; got `{}`",
            payload.posture().as_str()
        ),
    )
}
