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
    admit_query_workflow_declaration, bind_workflow_context,
    scoped_runtime_preflight_workflow_binding_for_binding_identity, QueryWorkflowDeclaration,
    WorkflowBindingScopeField, WorkflowBindingSource, WorkflowDeclarationRequest,
};

use self::semantics::{
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
            domain_contribution.request_identity().clone(),
        ));
    };
    if !workflow_runtime_semantics_match_posture(payload.posture(), runtime_semantics) {
        return TransitionOutcome::Denied(inconsistent_workflow_runtime_semantics_denial(
            "workflow declaration materialization",
            payload,
            runtime_semantics,
            target.kind(),
            domain_contribution.request_identity().clone(),
        ));
    }

    let source_label = workflow_source_label(target, payload);
    let binding = match runtime_semantics.binding() {
        ForgeQueryWorkflowRuntimeBindingSemantics::RuntimePreflight {
            runtime_snapshot_identity,
        } => {
            let binding_scope =
                WorkflowBindingScopeField::Identity(&target.binding_identity());
            crate::workflow::synthetic_runtime_workflow_binding_scoped_for_snapshot_binding_identity(
                source_label.as_str(),
                &binding_scope,
                runtime_snapshot_identity.clone(),
            )
        }
        ForgeQueryWorkflowRuntimeBindingSemantics::RuntimePreflightBundle { preflight } =>
            match scoped_runtime_preflight_workflow_binding_for_binding_identity(
                preflight,
                &target.binding_identity(),
            ) {
                Ok(binding) => binding,
                Err(error) => {
                    return TransitionOutcome::Denied(
                        ForgeQueryDomainCapabilityProgressionDenial::new(
                            ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                            "workflow-preview",
                            domain_contribution.target().kind(),
                            domain_contribution.request_identity().clone(),
                            format!(
                                "workflow runtime-preflight binding admission denied with `{:?}`: {}",
                                error.failure_class(),
                                error.message()
                            ),
                        ),
                    )
                }
            },
        ForgeQueryWorkflowRuntimeBindingSemantics::PreviewFoundation { .. } => {
            let foundation = match preview::admit_validated_preview_workflow_foundation(
                &contribution,
                runtime_semantics,
            ) {
                Ok(foundation) => foundation,
                Err(denial) => return TransitionOutcome::Denied(denial),
            };
            match bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation)) {
                Ok(binding) => binding,
                Err(error) => {
                    return TransitionOutcome::Denied(
                        ForgeQueryDomainCapabilityProgressionDenial::new(
                            ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
                            "workflow-preview",
                            domain_contribution.target().kind(),
                            domain_contribution.request_identity().clone(),
                            format!(
                                "workflow preview binding admission denied with `{:?}`: {}",
                                error.failure_class(),
                                error.message()
                            ),
                        ),
                    )
                }
            }
        }
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
            domain_contribution.request_identity().clone(),
            format!(
                "workflow declaration materialization denied with `{:?}`: {}",
                error.failure_class(),
                error.message()
            ),
        )),
    }
}

pub trait ForgeQueryWorkflowDeclarationMaterializationTarget:
    ForgeQueryDomainCapabilityTargetBinding
    + preview::ForgeQueryWorkflowPreviewMaterializationTarget
    + private::Sealed
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

mod inspection;
mod lowering;
mod preview;
mod preview_identity;
mod semantics;

pub use inspection::*;
pub use lowering::*;
pub use preview::*;
