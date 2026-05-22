use forge_proof::TransitionOutcome;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
    ForgeQueryDomainCapabilityRebindRequired, ForgeQueryDomainCapabilityStale,
};
use crate::domain_capabilities::{
    ForgeQueryDomainCapabilityTransitionOutcome, ForgeQueryMaterializationReadyWorkflowContribution,
};
use crate::identity::hash_parts;
use crate::workflow::{
    lower_merge_workflow_declaration, lower_mutation_intent_declaration,
    lower_query_writeback_declaration, LoweredMergeWorkflowDeclaration,
    LoweredMutationIntentDeclaration, QueryWritebackDeclaration, WorkflowLoweringError,
    WorkflowStalenessClass,
};

use super::workflow::materialize_query_workflow_declaration;
use super::workflow::ForgeQueryWorkflowDeclarationMaterializationTarget;
use super::workflow_semantics::{
    inconsistent_workflow_lowering_semantics_denial, missing_workflow_lowering_semantics_denial,
    workflow_lowering_semantics_match_runtime,
};

pub fn materialize_lowered_mutation_intent_declaration<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<LoweredMutationIntentDeclaration>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let target_kind = contribution.payload().target().kind();
    let target_digest = contribution.payload().target().target_digest().to_string();
    let request_digest = contribution.payload().request_digest().to_string();
    let lowering = match extract_workflow_lowering_semantics(
        "mutation workflow lowering",
        contribution.payload(),
    ) {
        Ok(lowering) => lowering.clone(),
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let (authority_binding_digest, input) = match lowering.mutation_parts() {
        Some(parts) => parts,
        None => {
            return TransitionOutcome::Denied(inconsistent_workflow_lowering_semantics_denial(
                "mutation workflow lowering",
                contribution.payload().payload(),
                target_kind,
                &request_digest,
            ))
        }
    };
    let declaration = match materialize_query_workflow_declaration(contribution) {
        TransitionOutcome::Success(declaration) => declaration,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => return TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => {
            return TransitionOutcome::RebindRequired(rebind)
        }
        TransitionOutcome::Failed(failure) => return TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    };

    lower_mutation_intent_declaration(&declaration, authority_binding_digest, input.clone())
        .map_or_else(
            |error| {
                lowering_error_outcome(
                    "workflow-preview",
                    target_kind,
                    &request_digest,
                    &target_digest,
                    error,
                )
            },
            TransitionOutcome::Success,
        )
}

pub fn materialize_lowered_merge_workflow_declaration<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<LoweredMergeWorkflowDeclaration>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let target_kind = contribution.payload().target().kind();
    let target_digest = contribution.payload().target().target_digest().to_string();
    let request_digest = contribution.payload().request_digest().to_string();
    let lowering = match extract_workflow_lowering_semantics(
        "merge workflow lowering",
        contribution.payload(),
    ) {
        Ok(lowering) => lowering.clone(),
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let input = match lowering.merge_input() {
        Some(input) => input.clone(),
        None => {
            return TransitionOutcome::Denied(inconsistent_workflow_lowering_semantics_denial(
                "merge workflow lowering",
                contribution.payload().payload(),
                target_kind,
                &request_digest,
            ))
        }
    };
    let declaration = match materialize_query_workflow_declaration(contribution) {
        TransitionOutcome::Success(declaration) => declaration,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => return TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => {
            return TransitionOutcome::RebindRequired(rebind)
        }
        TransitionOutcome::Failed(failure) => return TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    };

    lower_merge_workflow_declaration(&declaration, input).map_or_else(
        |error| {
            lowering_error_outcome(
                "workflow-preview",
                target_kind,
                &request_digest,
                &target_digest,
                error,
            )
        },
        TransitionOutcome::Success,
    )
}

pub fn materialize_query_writeback_lowering<T>(
    contribution: ForgeQueryMaterializationReadyWorkflowContribution<T>,
) -> ForgeQueryDomainCapabilityTransitionOutcome<QueryWritebackDeclaration>
where
    T: ForgeQueryWorkflowDeclarationMaterializationTarget,
{
    let target_kind = contribution.payload().target().kind();
    let target_digest = contribution.payload().target().target_digest().to_string();
    let request_digest = contribution.payload().request_digest().to_string();
    let lowering = match extract_workflow_lowering_semantics(
        "writeback workflow lowering",
        contribution.payload(),
    ) {
        Ok(lowering) => lowering.clone(),
        Err(denial) => return TransitionOutcome::Denied(denial),
    };
    let input = match lowering.writeback_input() {
        Some(input) => input.clone(),
        None => {
            return TransitionOutcome::Denied(inconsistent_workflow_lowering_semantics_denial(
                "writeback workflow lowering",
                contribution.payload().payload(),
                target_kind,
                &request_digest,
            ))
        }
    };
    let declaration = match materialize_query_workflow_declaration(contribution) {
        TransitionOutcome::Success(declaration) => declaration,
        TransitionOutcome::Denied(denial) => return TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => return TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => {
            return TransitionOutcome::RebindRequired(rebind)
        }
        TransitionOutcome::Failed(failure) => return TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    };

    lower_query_writeback_declaration(&declaration, input).map_or_else(
        |error| {
            lowering_error_outcome(
                "workflow-preview",
                target_kind,
                &request_digest,
                &target_digest,
                error,
            )
        },
        TransitionOutcome::Success,
    )
}

fn extract_workflow_lowering_semantics<'a, T>(
    operation_label: &'static str,
    contribution: &'a crate::domain_capabilities::ForgeQueryDomainCapabilityContribution<
        crate::domain_capabilities::ForgeQueryWorkflowContributionPayload,
        T,
    >,
) -> Result<
    &'a crate::domain_capabilities::ForgeQueryWorkflowLoweringSemantics,
    ForgeQueryDomainCapabilityProgressionDenial,
>
where
    T: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetBinding,
{
    let payload = contribution.payload();
    let Some(runtime_semantics) = payload.runtime_semantics() else {
        return Err(missing_workflow_lowering_semantics_denial(
            operation_label,
            payload,
            contribution.target().kind(),
            contribution.request_digest(),
        ));
    };
    let Some(lowering_semantics) = payload.lowering_semantics() else {
        return Err(missing_workflow_lowering_semantics_denial(
            operation_label,
            payload,
            contribution.target().kind(),
            contribution.request_digest(),
        ));
    };
    if !workflow_lowering_semantics_match_runtime(runtime_semantics, lowering_semantics) {
        return Err(inconsistent_workflow_lowering_semantics_denial(
            operation_label,
            payload,
            contribution.target().kind(),
            contribution.request_digest(),
        ));
    }
    Ok(lowering_semantics)
}

fn lowering_error_outcome<S>(
    category: &'static str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    request_digest: &str,
    target_digest: &str,
    error: WorkflowLoweringError,
) -> ForgeQueryDomainCapabilityTransitionOutcome<S> {
    match error.staleness_class() {
        WorkflowStalenessClass::StaleDenied => {
            TransitionOutcome::Stale(ForgeQueryDomainCapabilityStale::new(
                category,
                target_digest,
                lowering_posture_digest(target_digest, &error),
            ))
        }
        WorkflowStalenessClass::ExplicitRebindRequired => {
            TransitionOutcome::RebindRequired(ForgeQueryDomainCapabilityRebindRequired::new(
                category,
                target_digest,
                lowering_posture_digest(target_digest, &error),
            ))
        }
        WorkflowStalenessClass::ExactBasisPreserved
        | WorkflowStalenessClass::AuthorityValidationRequired => TransitionOutcome::Denied(
            lowering_error_denial(category, request_digest, target_kind, target_digest, error),
        ),
    }
}

fn lowering_error_denial(
    category: &'static str,
    request_digest: &str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    target_digest: &str,
    error: WorkflowLoweringError,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        category,
        target_kind,
        request_digest,
        format!(
            "workflow lowering denied at target `{target_digest}` with `{:?}` / `{:?}`: {}",
            error.failure_class(),
            error.staleness_class(),
            error.message()
        ),
    )
}

fn lowering_posture_digest(target_digest: &str, error: &WorkflowLoweringError) -> String {
    hash_parts(&[
        "forge_query_domain_capability_workflow_lowering_posture_v1".to_string(),
        format!("target:{target_digest}"),
        format!("failure:{:?}", error.failure_class()),
        format!("staleness:{}", error.staleness_class().as_str()),
        format!("message:{}", error.message()),
    ])
}
