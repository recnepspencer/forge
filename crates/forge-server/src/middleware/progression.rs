use forge_foundational::DiagnosticRichnessProfile;
use forge_proof::{compose_join_transition_outcome, compose_transition_outcome, TransitionOutcome};

use crate::config::ForgeServerMiddlewareConfig;
use crate::{ForgeServerBranchTarget, ForgeServerSurfaceFamily};

use super::{
    ordering::select_primary_denial, ForgeServerAdmission, ForgeServerDenial,
    ForgeServerDenialCode, ForgeServerDenialPriority, ForgeServerMiddlewareDeferred,
    ForgeServerMiddlewareFailure, ForgeServerMiddlewareRebindRequired, ForgeServerMiddlewareStale,
    ForgeServerPipelineInput, ForgeServerPipelineIntent, ForgeServerPipelineStep,
    ForgeServerPreparedQueryHandoffIntent,
};

type MiddlewareTransitionOutcome<T> = TransitionOutcome<
    T,
    ForgeServerDenial,
    ForgeServerMiddlewareDeferred,
    ForgeServerMiddlewareStale,
    ForgeServerMiddlewareRebindRequired,
    ForgeServerMiddlewareFailure,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ForgeServerBudgetAdmitted;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ForgeServerAuthorizationAdmitted;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ForgeServerPolicyAdmitted;

pub(crate) fn admit_pipeline_input(
    config: &ForgeServerMiddlewareConfig,
    pipeline_input: ForgeServerPipelineInput,
) -> super::ForgeServerAdmissionOutcome {
    let policy_outcome = resolve_policy_admission(config, &pipeline_input);
    compose_transition_outcome(policy_outcome, |_| {
        validate_pipeline_input(config, pipeline_input)
    })
}

fn resolve_policy_admission(
    config: &ForgeServerMiddlewareConfig,
    pipeline_input: &ForgeServerPipelineInput,
) -> MiddlewareTransitionOutcome<ForgeServerPolicyAdmitted> {
    let budget_outcome = evaluate_budget_posture(config, pipeline_input);
    let authorization_outcome = evaluate_authorization_posture(config, pipeline_input);

    match (budget_outcome, authorization_outcome) {
        (TransitionOutcome::Denied(left), TransitionOutcome::Denied(right)) => {
            TransitionOutcome::denied(select_primary_denial(left, right))
        }
        (left, right) => compose_join_transition_outcome(
            left,
            || right,
            |_| TransitionOutcome::success(ForgeServerPolicyAdmitted),
        ),
    }
}

fn evaluate_budget_posture(
    config: &ForgeServerMiddlewareConfig,
    pipeline_input: &ForgeServerPipelineInput,
) -> MiddlewareTransitionOutcome<ForgeServerBudgetAdmitted> {
    let resolved_request_context = pipeline_input.resolved_request_context();
    let diagnostics_profile = resolved_request_context
        .request_context()
        .diagnostics_profile();
    let compat_http_budget_exceeded = resolved_request_context.surface_family()
        == ForgeServerSurfaceFamily::CompatHttp
        && diagnostics_profile > config.compat_http_maximum_diagnostics_profile();
    if compat_http_budget_exceeded {
        return TransitionOutcome::denied(ForgeServerDenial::new(
            ForgeServerDenialCode::CompatHttpDiagnosticsBudgetExceeded,
            diagnostics_profile,
            ForgeServerDenialPriority::Budget,
            ForgeServerPipelineStep::BudgetPosture,
            compat_http_diagnostics_budget_detail(
                diagnostics_profile,
                config.compat_http_maximum_diagnostics_profile(),
            ),
        ));
    }

    TransitionOutcome::success(ForgeServerBudgetAdmitted)
}

fn evaluate_authorization_posture(
    config: &ForgeServerMiddlewareConfig,
    pipeline_input: &ForgeServerPipelineInput,
) -> MiddlewareTransitionOutcome<ForgeServerAuthorizationAdmitted> {
    if matches!(
        pipeline_input
            .resolved_request_context()
            .request_context()
            .branch_target(),
        ForgeServerBranchTarget::Preview { .. }
    ) && !config.preview_branch_authorization_enabled()
    {
        return TransitionOutcome::denied(ForgeServerDenial::new(
            ForgeServerDenialCode::PreviewBranchAccessDenied,
            pipeline_input
                .resolved_request_context()
                .request_context()
                .diagnostics_profile(),
            ForgeServerDenialPriority::Authorization,
            ForgeServerPipelineStep::AuthorizationPosture,
            "preview branch access is denied by middleware authorization posture",
        ));
    }

    TransitionOutcome::success(ForgeServerAuthorizationAdmitted)
}

fn validate_pipeline_input(
    config: &ForgeServerMiddlewareConfig,
    pipeline_input: ForgeServerPipelineInput,
) -> MiddlewareTransitionOutcome<ForgeServerAdmission> {
    if matches!(
        pipeline_input.pipeline_intent(),
        ForgeServerPipelineIntent::QueryMutation { .. }
    ) && !config.query_mutation_enabled()
    {
        return TransitionOutcome::denied(ForgeServerDenial::new(
            ForgeServerDenialCode::QueryMutationDisabled,
            pipeline_input
                .resolved_request_context()
                .request_context()
                .diagnostics_profile(),
            ForgeServerDenialPriority::Validation,
            ForgeServerPipelineStep::ValidationPosture,
            "query mutation intent is disabled by middleware validation posture",
        ));
    }

    prepare_query_handoff_admission(pipeline_input)
}

fn prepare_query_handoff_admission(
    pipeline_input: ForgeServerPipelineInput,
) -> MiddlewareTransitionOutcome<ForgeServerAdmission> {
    let (resolved_request_context, pipeline_intent) = pipeline_input.into_parts();
    let prepared_intent =
        ForgeServerPreparedQueryHandoffIntent::from_pipeline_intent(pipeline_intent);
    TransitionOutcome::success(ForgeServerAdmission::new(
        resolved_request_context,
        prepared_intent,
    ))
}

fn compat_http_diagnostics_budget_detail(
    diagnostics_profile: DiagnosticRichnessProfile,
    maximum_profile: DiagnosticRichnessProfile,
) -> String {
    format!(
        "compatibility HTTP transport cannot admit diagnostics profile {:?} above {:?}",
        diagnostics_profile, maximum_profile
    )
}
