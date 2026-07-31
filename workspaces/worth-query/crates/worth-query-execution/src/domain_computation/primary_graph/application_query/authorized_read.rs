use crate::domain_computation::primary_graph::{
    application_query::{
        read_execution::WorthQueryApplicationReadExecutionDenial,
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
    },
    entity_resolution::validate_entity_freshness_at_snapshot,
    WorthQueryPrimaryGraph, WorthQueryPrimaryGraphApplicationRuntime,
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

pub(super) enum WorthQueryAuthorizedApplicationReadDenial {
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
            validate_current_authorization(application, runtime, &current, plan);
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
    if !plan
        .authorization
        .remains_current_in(runtime, current, application.authorization.bridge())
    {
        return Err(WorthQueryAuthorizedApplicationReadDenial::Authorization(
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
            plan.query.name().to_string(),
        ));
    }
    validate_entity_freshness_at_snapshot(runtime, current, plan.scope)
        .map_err(|_| WorthQueryAuthorizedApplicationReadDenial::StaleScope)?;
    Ok(plan.authorization_work)
}
