use bank_domain::model::BankPrincipalId;
use bank_domain::schema::{BankSchema, Principal};
use worth_query_host::facade::{
    declaration::application_schema::{
        ApplicationFieldCurrency, TypedApplicationValue, WritePosture,
    },
    primary_graph::{
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationOneShotResult,
        WorthQueryApplicationPreviewResult, WorthQueryApplicationProjection,
        WorthQueryApplicationQueryAccessContext, WorthQueryPrimaryGraphApplicationRuntime,
        WorthQueryPrincipalResolutionMode,
    },
};

use super::{BankApplicationQueryDenial, BankApplicationQueryInvocation};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub(crate) fn execute_one_shot<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeCurrency,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeCurrency,
    >,
) -> Result<WorthQueryApplicationOneShotResult<Query, QueryResult>, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeCurrency: ApplicationFieldCurrency,
{
    execute_with_lane(runtime, principal, invocation, |application, plan| {
        application
            .execute_application_query_one_shot(plan)
            .map_err(BankApplicationQueryDenial::Execution)
    })
}

pub(crate) fn execute_preview<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeCurrency,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeCurrency,
    >,
) -> Result<WorthQueryApplicationPreviewResult<Query, QueryResult>, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeCurrency: ApplicationFieldCurrency,
{
    execute_with_lane(runtime, principal, invocation, |application, plan| {
        application
            .execute_application_query_preview(plan)
            .map_err(BankApplicationQueryDenial::PreviewExecution)
    })
}

fn execute_with_lane<
    Query,
    Parameters,
    QueryResult,
    Scope,
    ScopeAspect,
    ScopeField,
    ScopeIdentity,
    ScopeWrite,
    ScopeCurrency,
    Output,
>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    invocation: BankApplicationQueryInvocation<
        '_,
        Query,
        Parameters,
        QueryResult,
        Scope,
        ScopeAspect,
        ScopeField,
        ScopeIdentity,
        ScopeWrite,
        ScopeCurrency,
    >,
    execute: impl FnOnce(
        &WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
        WorthQueryAdmittedApplicationQueryPlan<
            '_,
            BankSchema,
            Query,
            Parameters,
            QueryResult,
            Principal,
            BankPrincipalId,
            Scope,
        >,
    ) -> Result<Output, BankApplicationQueryDenial>,
) -> Result<Output, BankApplicationQueryDenial>
where
    QueryResult: WorthQueryApplicationProjection<BankSchema, Query>,
    ScopeIdentity: TypedApplicationValue,
    ScopeWrite: WritePosture,
    ScopeCurrency: ApplicationFieldCurrency,
{
    let BankApplicationQueryInvocation {
        reference,
        scope_field,
        scope_identity,
        parameters,
        controls,
    } = invocation;
    let application = runtime.application_runtime();
    let query = application
        .installed_schema()
        .application_query(reference)
        .map_err(BankApplicationQueryDenial::Installation)?;
    let scope = application
        .resolve_entity(
            scope_field,
            scope_identity,
            controls.request_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(BankApplicationQueryDenial::ScopeResolution)?;
    let access = WorthQueryApplicationQueryAccessContext::<
        BankSchema,
        Principal,
        BankPrincipalId,
        Scope,
    >::new(principal.query(), &scope);
    let plan = application
        .admit_application_query(&query, &access, parameters, controls)
        .map_err(BankApplicationQueryDenial::Admission)?;
    execute(application, plan)
}
