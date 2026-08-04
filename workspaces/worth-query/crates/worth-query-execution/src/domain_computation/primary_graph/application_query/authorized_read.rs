use crate::domain_computation::primary_graph::{
    application_query::{
        read_execution::WorthQueryApplicationReadExecutionDenial,
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationAuthorizationWorkEvidence,
    },
    entity_resolution::validate_entity_freshness_at_snapshot,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use worth_query_declaration::facade::application_schema::ApplicationSchema;

pub(super) enum WorthQueryAuthorizedApplicationReadDenial {
    StalePrincipal,
    StaleScope,
    StaleBasisScope(crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenialKind),
    Authorization(
        crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind,
        String,
    ),
    Read(WorthQueryApplicationReadExecutionDenial),
    Session,
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
        &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout,
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
    (
        Output,
        WorthQueryApplicationAuthorizationWorkEvidence,
        crate::domain_computation::provider_session::WorthQuerySessionGraphReadProof,
    ),
    WorthQueryAuthorizedApplicationReadDenial,
>
where
    Schema: ApplicationSchema,
{
    let basis = plan.basis.identity();
    let (read_outcome, proof) = plan
        .graph_work
        .execute_query_read(basis, |runtime, layout| {
            let current = runtime.snapshots().snapshot();
            let authorization_work =
                validate_current_authorization(application, runtime, &current, plan);
            runtime.snapshots().release_snapshot(&current);
            let authorization_work = authorization_work?;
            validate_entity_freshness_at_snapshot(
                runtime,
                plan.basis.snapshot_handle(),
                plan.scope,
            )
            .map_err(|denial| {
                WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(denial.kind())
            })?;
            let output = read(runtime, layout, plan)
                .map_err(WorthQueryAuthorizedApplicationReadDenial::Read)?;
            Ok((output, authorization_work))
        })
        .map_err(|_| WorthQueryAuthorizedApplicationReadDenial::Session)?;
    let (output, authorization_work) = read_outcome?;
    Ok((output, authorization_work, proof))
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
    plan.authorization
        .validate_currentness_in(runtime, current, application.authorization.bridge())
        .map_err(|kind| match kind {
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::StalePrincipal => {
                WorthQueryAuthorizedApplicationReadDenial::StalePrincipal
            }
            kind => WorthQueryAuthorizedApplicationReadDenial::Authorization(
                kind, plan.query.name().to_string(),
            ),
        })?;
    if let Some(authorization) = plan.governance.authorization() {
        authorization
            .validate_currentness_in(runtime, current, application.authorization.bridge())
            .map_err(|kind| {
                WorthQueryAuthorizedApplicationReadDenial::Authorization(
                    kind,
                    plan.query.name().to_string(),
                )
            })?;
    }
    if !plan.governance.computation_matches(
        &plan.graph_work,
        application.runtime.authority_identity(),
        plan.query.identity(),
        plan.parameters.identity(),
        plan.principal.principal_entity_id(),
        plan.scope.entity_id(),
    ) {
        return Err(WorthQueryAuthorizedApplicationReadDenial::Authorization(
            crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            plan.query.name().to_string(),
        ));
    }
    validate_entity_freshness_at_snapshot(runtime, current, plan.scope)
        .map_err(|_| WorthQueryAuthorizedApplicationReadDenial::StaleScope)?;
    Ok(plan.authorization_work)
}

pub(super) fn refresh_governed_authorization<
    Schema,
    Query,
    Parameters,
    QueryResult,
    Principal,
    PrincipalIdentity,
    Scope,
>(
    application: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    plan: &mut WorthQueryAdmittedApplicationQueryPlan<
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
    >,
) -> Result<(), WorthQueryAuthorizedApplicationReadDenial>
where
    Schema: ApplicationSchema,
{
    let (governance, graph_work) = (&mut plan.governance, &mut plan.graph_work);
    let Some(authorization) = governance.authorization_mut() else {
        return Ok(());
    };
    application
        .refresh_capability_authorization_for_graph_work(authorization, graph_work)
        .map_err(|denial| {
            WorthQueryAuthorizedApplicationReadDenial::Authorization(
                denial.kind(),
                plan.query.name().to_string(),
            )
        })
}
