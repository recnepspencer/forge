use forge_proof::TransitionOutcome;
use forge_runtime_bridge::facade::{
    BridgeIdentityEvidence, BridgePreviewSessionDeclarationIdentity,
};

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
use crate::identity::{hash_parts, CanonicalQueryDigest, ValidatedQueryDigest};
use crate::preview::{
    admit_contributed_preview_workflow_foundation,
    materialize_contributed_preview_workflow_foundation_artifact,
    AdmittedPreviewWorkflowFoundation, PreviewEvaluationClass, PreviewWorkflowFoundationArtifact,
    PreviewWorkflowFoundationRequest,
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
    let (artifact, target_kind, request_digest) =
        prepare_validated_preview_workflow_foundation(contribution, runtime_semantics)?;
    match admit_contributed_preview_workflow_foundation(artifact) {
        Ok(foundation) => Ok(foundation),
        Err(error) => {
            let failure_class = error.failure_class().clone();
            Err(ForgeQueryDomainCapabilityProgressionDenial::new(
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
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return Err(inconsistent_workflow_runtime_semantics_denial(
            operation_label,
            payload,
            runtime_semantics,
            target.kind(),
            domain_contribution.request_digest(),
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
    let request_family = preview_request_family(payload.posture());
    let request_family_label = request_family.as_str();
    let binding_identity = target.binding_identity();
    let binding_digest = binding_identity.as_str().to_string();
    let canonical_query_digest = CanonicalQueryDigest::from_parts(&[
        "forge_query_domain_preview_query_v1".to_string(),
        format!("source:{source_label}"),
        format!("binding:{}", binding_identity.as_str()),
        format!(
            "preview_session:{}",
            preview_session_identity.evidence_identity().as_str()
        ),
        format!("evaluation:{}", evaluation_class.as_str()),
        format!("request_family:{request_family_label}"),
    ]);
    let validated_query_digest = ValidatedQueryDigest::from_parts(&[
        "forge_query_domain_preview_validated_query_v1".to_string(),
        format!("canonical:{}", canonical_query_digest.as_str()),
    ]);
    let preview_declaration_evidence = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowContextBinding,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("identity_family"),
        "domain_preview_declaration_v1",
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("semantic_code"),
        payload.semantic_code(),
    )
    .field_evidence_identity(
        crate::ForgeQueryEvidenceTag::new("binding"),
        &binding_identity,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("request_family"),
        request_family_label,
    )
    .seal();
    let declaration_identity = BridgePreviewSessionDeclarationIdentity::from_bridge_evidence(
        &BridgeIdentityEvidence::from_external_authority(preview_declaration_evidence),
    );
    let declaration_digest = hash_parts(&[
        "forge_query_domain_preview_declaration_v1".to_string(),
        format!(
            "identity:{}",
            declaration_identity.evidence_identity().as_str()
        ),
        format!("canonical:{}", canonical_query_digest.as_str()),
        format!("validated:{}", validated_query_digest.as_str()),
    ]);
    let artifact = materialize_contributed_preview_workflow_foundation_artifact(
        binding_digest,
        canonical_query_digest,
        validated_query_digest,
        request_family,
        preview_session_identity.clone(),
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
