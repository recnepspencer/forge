use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, ForgeQueryAdmissionContributionAuthoring,
    ForgeQueryAdmittedAdmissionContribution, ForgeQueryAdmittedAftermathContribution,
    ForgeQueryAdmittedContinuityContribution, ForgeQueryAdmittedExplanationContribution,
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryAdmittedSupportContribution,
    ForgeQueryAdmittedWorkflowContribution, ForgeQueryAftermathContributionAuthoring,
    ForgeQueryContinuityContributionAuthoring, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryExplanationContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};
use forge_proof::TransitionOutcome;

pub fn admitted_declaration_support(
    declaration_label: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedSupportContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQuerySupportContributionAuthoring::declaration_support(semantic_code, detail)
            .bind_to_declaration_target(declaration_target(declaration_label)),
    )
}

pub fn admitted_declaration_explanation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &ForgeQueryCanonicalDeclarationArtifact<D, I>,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedExplanationContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail)
            .bind_to_declaration_target(canonical_declaration_target(declaration)),
    )
}

pub fn admitted_declaration_advisory<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &ForgeQueryCanonicalDeclarationArtifact<D, I>,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAdmissionContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted(
        ForgeQueryAdmissionContributionAuthoring::advisory(semantic_code, detail)
            .bind_to_declaration_target(canonical_declaration_target(declaration)),
    )
}

pub fn admitted_declaration_workflow(
    declaration_label: &str,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedWorkflowContribution<ForgeQueryDeclarationBoundContributionTarget> {
    admitted_generic(
        ForgeQueryWorkflowContributionAuthoring::preview_only(semantic_code, detail)
            .bind_to_declaration_target(declaration_target(declaration_label)),
    )
}

pub fn admitted_plan_support(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedSupportContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQuerySupportContributionAuthoring::narrowed_support(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_workflow(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedWorkflowContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryWorkflowContributionAuthoring::preview_only(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_continuity(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedContinuityContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryContinuityContributionAuthoring::preserved(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_plan_aftermath(
    plan: &crate::runtime::ForgeQueryAdmittedIntentPlan,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAftermathContribution<ForgeQueryAdmittedPlanBoundContributionTarget> {
    admitted_generic(
        ForgeQueryAftermathContributionAuthoring::declares_residue(semantic_code, detail)
            .for_admitted_intent_plan(plan),
    )
}

pub fn admitted_lower_runtime_explanation(
    envelope: &crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedExplanationContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    admitted_generic(
        ForgeQueryExplanationContributionAuthoring::requires_context(semantic_code, detail)
            .for_lower_runtime_boundary_envelope(envelope),
    )
}

pub fn admitted_lower_runtime_aftermath(
    envelope: &crate::runtime::ForgeQueryLowerRuntimeBoundaryEnvelope,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryAdmittedAftermathContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    admitted_generic(
        ForgeQueryAftermathContributionAuthoring::declares_residue(semantic_code, detail)
            .for_lower_runtime_boundary_envelope(envelope),
    )
}

fn admitted<P>(
    requested: crate::domain_capabilities::ForgeQueryRequestedDomainCapabilityContribution<
        P,
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<
    P,
    ForgeQueryDeclarationBoundContributionTarget,
>
where
    P: crate::domain_capabilities::ForgeQueryDomainCapabilityPayload,
{
    admitted_generic(requested)
}

fn admitted_generic<P, T>(
    requested: crate::domain_capabilities::ForgeQueryRequestedDomainCapabilityContribution<P, T>,
) -> crate::domain_capabilities::ForgeQueryAdmittedDomainCapabilityContribution<P, T>
where
    P: crate::domain_capabilities::ForgeQueryDomainCapabilityPayload,
    T: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetBinding,
{
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    success(admit_eligible_domain_capability_contribution(eligible))
}

fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => {
            panic!("expected success, got denial {:?}", denial.kind())
        }
        TransitionOutcome::Stale(stale) => {
            panic!("expected success, got stale {}", stale.category())
        }
        TransitionOutcome::RebindRequired(rebind) => {
            panic!("expected success, got rebind {}", rebind.category())
        }
        TransitionOutcome::Failed(failure) => {
            panic!("expected success, got failure {}", failure.message())
        }
        TransitionOutcome::Deferred(never) => match never {},
    }
}

fn declaration_target(label: &str) -> ForgeQueryDeclarationBoundContributionTarget {
    let declaration = crate::runtime::ForgeQueryIntentDeclaration::strategy_commit(
        format!("declaration-entry-seam.{label}"),
        format!("forge.declaration_entry_seam.{label}"),
        "1",
        "forge.declaration-entry-seam.fixture",
        crate::runtime::ForgeQueryIntentInput::object([(
            "fixture",
            crate::runtime::ForgeQueryIntentInput::string(label),
        )]),
    );
    ForgeQueryDeclarationBoundContributionTarget::for_intent_declaration(&declaration)
}

fn canonical_declaration_target<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    declaration: &ForgeQueryCanonicalDeclarationArtifact<D, I>,
) -> ForgeQueryDeclarationBoundContributionTarget {
    ForgeQueryDeclarationBoundContributionTarget::for_canonical_declaration(declaration)
}
