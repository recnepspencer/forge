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
        let branch = plan
            .graph_work_session()
            .branch_affinity()
            .relational_branch()
            .clone();
        let handle = graph.integration_handle();
        let current_version = handle
            .ensure_primary_indexes_current(runtime, &branch)
            .map_err(|_| stale_authorization(plan.query.name()))?;
        let current = runtime
            .snapshots()
            .admit_execution_basis(&branch, current_version)
            .map_err(|_| stale_authorization(plan.query.name()))?;
        let authorization_work = validate_current_authorization(
            application,
            runtime,
            current.snapshot_handle(),
            plan,
        );
        let current_released = current.release().released();
        if !current_released {
            return Err(stale_authorization(plan.query.name()));
        }
        let authorization_work = authorization_work?;
        validate_entity_freshness_at_snapshot(runtime, plan.basis().snapshot_handle(), plan.scope)
            .map_err(|denial| {
                WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(denial.kind())
            })?;
        let output =
            read(runtime, graph, plan).map_err(WorthQueryAuthorizedApplicationReadDenial::Read)?;
        let completion = super::graph_read_completion::WorthQueryApplicationQueryGraphReadCompletion::mint(
            plan.graph_work_session(),
        );
        crate::domain_computation::provider_session::record_application_query_graph_read_completion(
            plan.graph_work_session_mut(),
            completion,
        )
        .map_err(|_| {
            WorthQueryAuthorizedApplicationReadDenial::Authorization(
                crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                plan.query.name().to_string(),
            )
        })?;
        Ok((output, authorization_work))
    })
}

fn stale_authorization(subject: &str) -> WorthQueryAuthorizedApplicationReadDenial {
    WorthQueryAuthorizedApplicationReadDenial::Authorization(
        crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
        subject.to_owned(),
    )
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
