use worth_query_declaration::facade::application_schema::ApplicationSchema;
use worth_query_installation::facade::TypedApplicationValue;

use crate::domain_computation::primary_graph::{
    application_query::{
        read_execution::WorthQueryApplicationReadExecutionDenial,
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
        WorthQueryApplicationQueryAccessContext,
    },
    entity_resolution::validate_entity_freshness_at_snapshot,
    resolution::validate_freshness_at_snapshot,
    WorthQueryAuthenticatedPrincipal, WorthQueryPrimaryGraph,
    WorthQueryPrimaryGraphApplicationRuntime,
};

pub(super) enum WorthQueryAuthorizedApplicationReadDenial {
    StalePrincipal,
    StaleScope,
    StaleBasisScope(crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenialKind),
    Authorization(
        crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind,
        String,
    ),
    Read(WorthQueryApplicationReadExecutionDenial),
}

pub(super) fn execute_authorized_read<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
    Output,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    graph: &WorthQueryPrimaryGraph,
    plan: &WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
    read: impl FnOnce(
        &worth_relational::facade::runtime::RelationalRuntime,
        &WorthQueryPrimaryGraph,
        &WorthQueryAdmittedApplicationQueryPlan<
            '_,
            Schema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            PrincipalIdentity,
            Scope,
        >,
    ) -> Result<Output, WorthQueryApplicationReadExecutionDenial>,
) -> Result<
    (Output, WorthQueryApplicationAuthorizationWorkEvidence),
    WorthQueryAuthorizedApplicationReadDenial,
>
where
    Schema: ApplicationSchema,
{
    graph.integration_handle().with_runtime_mut(|runtime| {
        let current = runtime.snapshots().snapshot();
        let authorization_work =
            validate_current_authorization(application, runtime, &current, graph, plan);
        runtime.snapshots().release_snapshot(&current);
        let authorization_work = authorization_work?;
        validate_entity_freshness_at_snapshot(runtime, plan.basis.snapshot_handle(), plan.scope)
            .map_err(|denial| {
                WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(denial.kind())
            })?;
        let output =
            read(runtime, graph, plan).map_err(WorthQueryAuthorizedApplicationReadDenial::Read)?;
        Ok((output, authorization_work))
    })
}

fn validate_current_authorization<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    current: &worth_relational::facade::snapshots::SnapshotHandle,
    graph: &WorthQueryPrimaryGraph,
    plan: &WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
) -> Result<WorthQueryApplicationAuthorizationWorkEvidence, WorthQueryAuthorizedApplicationReadDenial>
where
    Schema: ApplicationSchema,
{
    validate_principal_at_snapshot(runtime, current, graph, plan.principal)
        .map_err(|_| WorthQueryAuthorizedApplicationReadDenial::StalePrincipal)?;
    validate_entity_freshness_at_snapshot(runtime, current, plan.scope)
        .map_err(|_| WorthQueryAuthorizedApplicationReadDenial::StaleScope)?;
    let access = WorthQueryApplicationQueryAccessContext::new(plan.principal, plan.scope);
    application
        .observe_query_authorization(runtime, current.clone(), plan.query, &access)
        .map_err(|authorization| {
            WorthQueryAuthorizedApplicationReadDenial::Authorization(
                authorization.kind(),
                authorization.subject().to_string(),
            )
        })
}

fn validate_principal_at_snapshot<Schema, Principal, PrincipalIdentity>(
    runtime: &mut worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    graph: &WorthQueryPrimaryGraph,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
) -> Result<(), crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionDenial> {
    let layout = graph
        .layout
        .principal_binding(principal.binding())
        .ok_or_else(|| {
            crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionDenial::new(
                crate::domain_computation::primary_graph::WorthQueryPrincipalResolutionDenialKind::BindingNotInstalled,
                principal.binding(),
            )
        })?;
    let expected = principal
        .external_identity()
        .clone()
        .into_foundational_value();
    validate_freshness_at_snapshot(runtime, snapshot, principal, layout, &expected)
}
