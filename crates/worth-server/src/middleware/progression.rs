use worth_foundational::DiagnosticRichnessProfile;
use worth_proof::{compose_join_transition_outcome, compose_transition_outcome, TransitionOutcome};

use crate::config::WorthServerMiddlewareConfig;
use crate::{WorthServerBranchTarget, WorthServerSurfaceFamily};

use super::{
    ordering::select_primary_denial, WorthServerAdmission, WorthServerDenial,
    WorthServerDenialCode, WorthServerDenialPriority, WorthServerMiddlewareDeferred,
    WorthServerMiddlewareFailure, WorthServerMiddlewareRebindRequired, WorthServerMiddlewareStale,
    WorthServerPipelineInput, WorthServerPipelineIntent, WorthServerPipelineStep,
    WorthServerPreparedQueryHandoffIntent,
};

type MiddlewareTransitionOutcome<T> = TransitionOutcome<
    T,
    WorthServerDenial,
    WorthServerMiddlewareDeferred,
    WorthServerMiddlewareStale,
    WorthServerMiddlewareRebindRequired,
    WorthServerMiddlewareFailure,
>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthServerBudgetAdmitted;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthServerAuthorizationAdmitted;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthServerPolicyAdmitted;

pub(crate) fn admit_pipeline_input(
    config: &WorthServerMiddlewareConfig,
    pipeline_input: WorthServerPipelineInput,
) -> super::WorthServerAdmissionOutcome {
    let policy_outcome = resolve_policy_admission(config, &pipeline_input);
    compose_transition_outcome(policy_outcome, |_| {
        validate_pipeline_input(config, pipeline_input)
    })
}

fn resolve_policy_admission(
    config: &WorthServerMiddlewareConfig,
    pipeline_input: &WorthServerPipelineInput,
) -> MiddlewareTransitionOutcome<WorthServerPolicyAdmitted> {
    let budget_outcome = evaluate_budget_posture(config, pipeline_input);
    let authorization_outcome = evaluate_authorization_posture(config, pipeline_input);

    match (budget_outcome, authorization_outcome) {
        (TransitionOutcome::Denied(left), TransitionOutcome::Denied(right)) => {
            TransitionOutcome::denied(select_primary_denial(left, right))
        }
        (left, right) => compose_join_transition_outcome(
            left,
            || right,
            |_| TransitionOutcome::success(WorthServerPolicyAdmitted),
        ),
    }
}

fn evaluate_budget_posture(
    config: &WorthServerMiddlewareConfig,
    pipeline_input: &WorthServerPipelineInput,
) -> MiddlewareTransitionOutcome<WorthServerBudgetAdmitted> {
    let resolved_request_context = pipeline_input.resolved_request_context();
    let diagnostics_profile = resolved_request_context
        .request_context()
        .diagnostics_profile();
    let compat_http_budget_exceeded = resolved_request_context.surface_family()
        == WorthServerSurfaceFamily::CompatHttp
        && diagnostics_profile > config.compat_http_maximum_diagnostics_profile();
    if compat_http_budget_exceeded {
        return TransitionOutcome::denied(WorthServerDenial::new(
            WorthServerDenialCode::CompatHttpDiagnosticsBudgetExceeded,
            diagnostics_profile,
            WorthServerDenialPriority::Budget,
            WorthServerPipelineStep::BudgetPosture,
            compat_http_diagnostics_budget_detail(
                diagnostics_profile,
                config.compat_http_maximum_diagnostics_profile(),
            ),
        ));
    }

    TransitionOutcome::success(WorthServerBudgetAdmitted)
}

fn evaluate_authorization_posture(
    config: &WorthServerMiddlewareConfig,
    pipeline_input: &WorthServerPipelineInput,
) -> MiddlewareTransitionOutcome<WorthServerAuthorizationAdmitted> {
    if matches!(
        pipeline_input
            .resolved_request_context()
            .request_context()
            .branch_target(),
        WorthServerBranchTarget::Preview { .. }
    ) && !config.preview_branch_authorization_enabled()
    {
        return TransitionOutcome::denied(WorthServerDenial::new(
            WorthServerDenialCode::PreviewBranchAccessDenied,
            pipeline_input
                .resolved_request_context()
                .request_context()
                .diagnostics_profile(),
            WorthServerDenialPriority::Authorization,
            WorthServerPipelineStep::AuthorizationPosture,
            "preview branch access is denied by middleware authorization posture",
        ));
    }

    TransitionOutcome::success(WorthServerAuthorizationAdmitted)
}

fn validate_pipeline_input(
    config: &WorthServerMiddlewareConfig,
    pipeline_input: WorthServerPipelineInput,
) -> MiddlewareTransitionOutcome<WorthServerAdmission> {
    if matches!(
        pipeline_input.pipeline_intent(),
        WorthServerPipelineIntent::QueryMutation { .. }
    ) && !config.query_mutation_enabled()
    {
        return TransitionOutcome::denied(WorthServerDenial::new(
            WorthServerDenialCode::QueryMutationDisabled,
            pipeline_input
                .resolved_request_context()
                .request_context()
                .diagnostics_profile(),
            WorthServerDenialPriority::Validation,
            WorthServerPipelineStep::ValidationPosture,
            "query mutation intent is disabled by middleware validation posture",
        ));
    }

    prepare_query_handoff_admission(pipeline_input)
}

fn prepare_query_handoff_admission(
    pipeline_input: WorthServerPipelineInput,
) -> MiddlewareTransitionOutcome<WorthServerAdmission> {
    let (resolved_request_context, pipeline_intent) = pipeline_input.into_parts();
    let prepared_intent =
        WorthServerPreparedQueryHandoffIntent::from_pipeline_intent(pipeline_intent);
    TransitionOutcome::success(WorthServerAdmission::new(
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
