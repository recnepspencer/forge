//! Fresh observation of the exact governed support carried by a request receipt.

use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::ApplicationSchema;

use super::super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::super::super::delegation_admission::observe_elevation_upper_bound;
use super::super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAuthorizationDecisionFact,
    WorthQueryAuthorizationTimeSample, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use super::super::WorthQueryElevationRequestBinding;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

pub(super) fn refresh_exact_support<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    requested: &mut WorthQueryElevationRequestBinding,
    access: &mut WorthQueryAdmittedApplicationCapabilityAccess<
        Schema,
        Capability,
        Operation,
        Input,
    >,
) -> Result<WorthQueryAuthorizationTimeSample, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let installed = runtime.installed_capability_plan(requested.supporting.request())?;
    validate_support_owner(requested, installed)?;
    let (sample, decision) = observe_current_support(runtime, requested, access, installed)?;
    let current_sample = sample.clone();
    requested
        .supporting
        .replace_current_session(access.graph_work.identity(), sample, decision)
        .map_err(|()| inconsistent_support(installed.contract.name()))?;
    access
        .authorization
        .retain_supporting(requested.supporting.retained_for_operation())
        .map_err(|()| inconsistent_support(installed.contract.name()))?;
    access.graph_work.record_decision_facts(1);
    Ok(current_sample)
}

fn observe_current_support<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    requested: &WorthQueryElevationRequestBinding,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Result<
    (
        WorthQueryAuthorizationTimeSample,
        WorthQueryAuthorizationDecisionFact,
    ),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let sample = runtime.sample_capability_time(installed)?;
    let snapshot = access
        .graph_work
        .mutation_snapshot()
        .ok_or_else(|| stale_support(installed.contract.name()))?
        .clone();
    let handle = access
        .graph_work
        .mutation_handle()
        .ok_or_else(|| stale_support(installed.contract.name()))?
        .clone();
    let observed = handle
        .with_runtime(|relational| {
            if !requested.supporting.decision().remains_current_in(
                relational,
                &snapshot,
                runtime.authorization.bridge(),
            ) {
                return Err(stale_support(installed.contract.name()));
            }
            observe_elevation_upper_bound(
                access.graph_work.identity(),
                relational,
                snapshot.clone(),
                runtime.authorization.bridge(),
                installed,
                requested.supporting.request(),
                &sample,
                requested.supporting.grant(),
                Some(requested.supporting.decision()),
            )
        })
        .map_err(|denial| support_observation_denial(installed.contract.name(), denial))?;
    let (decision, grant) = observed.into_parts();
    (grant == requested.supporting.grant())
        .then_some((sample, decision))
        .ok_or_else(|| inconsistent_support(installed.contract.name()))
}

fn validate_support_owner(
    requested: &WorthQueryElevationRequestBinding,
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if requested.capability_identity == requested.supporting.request().capability_identity
        && requested.capability_authority_identity.as_ref()
            == requested.supporting.capability_authority_identity()
        && requested.supporting.capability_authority_identity()
            == installed.capability_authority_identity.as_ref()
        && requested.grant() == requested.supporting.grant()
    {
        Ok(())
    } else {
        Err(inconsistent_support(installed.contract.name()))
    }
}

fn support_observation_denial(
    subject: &str,
    denial: WorthQueryOperationAuthorizationDenial,
) -> WorthQueryOperationAuthorizationDenial {
    match denial.kind() {
        WorthQueryOperationAuthorizationDenialKind::PermissionDenied
        | WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected => {
            stale_support(subject)
        }
        _ => denial,
    }
}

fn stale_support(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
        subject,
    )
}

fn inconsistent_support(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
