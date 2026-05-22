use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::ForgeQueryWorkflowRuntimeBindingSemantics;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalWorkflowArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyWorkflowContribution,
};
use crate::workflow::{
    admit_query_workflow_declaration, synthetic_preview_workflow_binding_scoped,
    synthetic_runtime_workflow_binding_scoped, QueryWorkflowDeclaration,
    WorkflowDeclarationRequest,
};

use super::workflow_semantics::{
    inconsistent_workflow_runtime_semantics_denial, missing_workflow_runtime_semantics_denial,
    workflow_runtime_semantics_match_posture, workflow_source_label,
};

pub fn materialize_canonical_workflow_artifact<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryCanonicalWorkflowArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

pub fn materialize_query_workflow_declaration<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryWorkflowDeclaration>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let target = domain_contribution.target();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return TransitionOutcome::Denied(missing_workflow_runtime_semantics_denial(
            "workflow declaration materialization",
            payload,
            target.kind(),
            domain_contribution.request_digest(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return TransitionOutcome::Denied(inconsistent_workflow_runtime_semantics_denial(
            "workflow declaration materialization",
            payload,
            runtime_semantics,
            target.kind(),
            domain_contribution.request_digest(),
        ));
    }

    let source_label = workflow_source_label(target, payload);
    let binding = match runtime_semantics.binding() {
        ForgeQueryWorkflowRuntimeBindingSemantics::RuntimePreflight {
            runtime_snapshot_token,
        } => synthetic_runtime_workflow_binding_scoped(
            source_label.as_str(),
            target.binding_digest(),
            runtime_snapshot_token,
        ),
        ForgeQueryWorkflowRuntimeBindingSemantics::PreviewFoundation {
            preview_session_identity,
            evaluation_class,
        } => synthetic_preview_workflow_binding_scoped(
            source_label.as_str(),
            target.binding_digest(),
            preview_session_identity,
            evaluation_class.clone(),
        ),
    };
    let request = WorkflowDeclarationRequest::new(
        runtime_semantics.declaration_family().clone(),
        runtime_semantics.authority_target_family().clone(),
        runtime_semantics.cost_class().clone(),
        runtime_semantics.budget_class().clone(),
        runtime_semantics.freshness_policy().clone(),
    );

    match admit_query_workflow_declaration(&binding, request) {
        Ok(declaration) => TransitionOutcome::Success(declaration),
        Err(error) => TransitionOutcome::Denied(ForgeQueryDomainCapabilityProgressionDenial::new(
            ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
            "workflow-preview",
            domain_contribution.target().kind(),
            domain_contribution.request_digest(),
            format!(
                "workflow declaration materialization denied with `{:?}`: {}",
                error.failure_class(),
                error.message()
            ),
        )),
    }
}

pub trait ForgeQueryWorkflowDeclarationMaterializationTarget:
    ForgeQueryDomainCapabilityTargetBinding + private::Sealed
{
}

impl ForgeQueryWorkflowDeclarationMaterializationTarget
    for ForgeQueryDeclarationBoundContributionTarget
{
}

impl ForgeQueryWorkflowDeclarationMaterializationTarget
    for ForgeQueryAdmittedPlanBoundContributionTarget
{
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for ForgeQueryDeclarationBoundContributionTarget {}
impl private::Sealed for ForgeQueryAdmittedPlanBoundContributionTarget {}
